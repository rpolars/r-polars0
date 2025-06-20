/// this file implements extendr wrapper class Series
/// Should really be called Rseries on rust side, but
/// r-polars has only one class 'Series' on R side and extendr_api currently
/// requires the class/struct named the same in R and polars.
/// Therefore there annoyingly exists pl::Series and Series
use crate::apply_input;
use crate::apply_output;
use crate::conversion_r_to_s::robjname2series;
use crate::conversion_s_to_r::pl_series_to_list;
use crate::handle_type;
use crate::lazy::dsl::RPolarsExpr;
use crate::make_r_na_fun;
use crate::rdataframe::RPolarsDataFrame;
use crate::rdatatype::RPolarsDataType;
use crate::robj_to;
use crate::rpolarserr::RResult;
use crate::utils::wrappers::null_to_opt;
use crate::utils::{r_error_list, r_result_list};
use extendr_api::{extendr, prelude::*, rprintln};
use pl::SeriesMethods;
use polars::datatypes::*;
use polars::prelude as pl;
use polars::prelude::{ArgAgg, IntoColumn, IntoSeries};
use polars_core::series::IsSorted;
pub const R_INT_NA_ENC: i32 = -2147483648;
use crate::rpolarserr::polars_to_rpolars_err;
use std::result::Result;

use polars_core::error::PolarsError;
use polars_core::utils::arrow;

pub struct OwnedSeriesIterator {
    series: pl::Series,
    idx: usize,
    n_chunks: usize,
    compat_level: CompatLevel,
}

impl OwnedSeriesIterator {
    pub fn new(s: pl::Series, compat_level: CompatLevel) -> Self {
        Self {
            series: s.slice(0, s.len()),
            idx: 0,
            n_chunks: s.n_chunks(),
            compat_level: compat_level,
        }
    }
}

impl Iterator for OwnedSeriesIterator {
    type Item = Result<Box<dyn arrow::array::Array>, PolarsError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n_chunks {
            None
        } else {
            let batch = self.series.to_arrow(self.idx, self.compat_level);
            self.idx += 1;

            Some(std::result::Result::Ok(batch))
        }
    }
}

#[derive(Debug, Clone)]
pub struct RPolarsSeries(pub pl::Series);

impl From<polars::prelude::Series> for RPolarsSeries {
    fn from(pls: polars::prelude::Series) -> Self {
        RPolarsSeries(pls)
    }
}

impl From<&RPolarsExpr> for pl::PolarsResult<RPolarsSeries> {
    fn from(expr: &RPolarsExpr) -> Self {
        RPolarsDataFrame::default()
            .lazy()
            .0
            .select(&[expr.0.clone()])
            .collect()
            .map(|df| {
                df.select_at_idx(0)
                    .cloned()
                    .unwrap_or_else(|| {
                        pl::Series::new_empty("".into(), &pl::DataType::Null).into_column()
                    })
                    .as_materialized_series()
                    .clone()
                    .into()
            })
    }
}

#[extendr]
impl RPolarsSeries {
    //utility methods
    pub fn new(name: Robj, values: Robj) -> RResult<Self> {
        robjname2series(values, robj_to!(str, name)?)
            .map_err(polars_to_rpolars_err)
            .map(RPolarsSeries)
    }

    // named like this to no collide with clone trait but still export with extendr
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> RPolarsSeries {
        RPolarsSeries(self.0.clone())
    }

    //function for debugging only
    pub fn sleep(&self, millis: i32) -> RPolarsSeries {
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
        self.clone()
    }

    pub fn panic(&self) -> RPolarsSeries {
        panic!("somebody panicked on purpose");
    }

    pub fn to_r(&self, int64_conversion: &str) -> std::result::Result<Robj, String> {
        pl_series_to_list(&self.0, true, int64_conversion)
            .map_err(|err| format!("in to_r: {:?}", err))
    }
    //any mut method exposed in R suffixed _mut
    pub fn rename_mut(&mut self, name: &str) {
        self.0.rename(name.into());
    }

    //any other method or trait method in alphabetical order

    //skip arr, cat, dt namespace methods

    pub fn dtype(&self) -> RPolarsDataType {
        RPolarsDataType(self.0.dtype().clone())
    }

    fn n_unique(&self) -> Result<usize, String> {
        let n = self.0.n_unique().map_err(|err| err.to_string())?;
        Ok(n)
    }

    //wait inner_dtype until list supported

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn sort(
        &mut self,
        descending: Robj,
        nulls_last: Robj,
        multithreaded: Robj,
    ) -> RResult<Self> {
        let descending = robj_to!(bool, descending)?;
        let nulls_last = robj_to!(bool, nulls_last)?;
        let multithreaded = robj_to!(bool, multithreaded)?;
        Ok(self
            .0
            .sort(
                pl::SortOptions::default()
                    .with_order_descending(descending)
                    .with_nulls_last(nulls_last)
                    .with_multithreaded(multithreaded),
            )
            .map_err(polars_to_rpolars_err)?
            .into())
    }

    pub fn value_counts(
        &self,
        sort: bool,
        parallel: bool,
        name: String,
        normalize: bool,
    ) -> std::result::Result<RPolarsDataFrame, String> {
        self.0
            .value_counts(sort, parallel, name.into(), normalize)
            .map(RPolarsDataFrame)
            .map_err(|err| format!("in value_counts: {:?}", err))
    }

    pub fn arg_min(&self) -> Option<usize> {
        self.0.arg_min()
    }

    pub fn arg_max(&self) -> Option<usize> {
        self.0.arg_max()
    }

    pub fn can_fast_explode_flag(&self) -> bool {
        match self.0.list() {
            Err(_) => false,
            Ok(list) => list._can_fast_explode(),
        }
    }

    pub fn is_sorted_ascending_flag(&self) -> bool {
        matches!(self.0.is_sorted_flag(), IsSorted::Ascending)
    }

    pub fn is_sorted_descending_flag(&self) -> bool {
        matches!(self.0.is_sorted_flag(), IsSorted::Descending)
    }

    pub fn is_sorted(&self, descending: Robj) -> RResult<bool> {
        let descending = robj_to!(bool, descending)?;
        let options = pl::SortOptions {
            descending,
            nulls_last: descending,
            multithreaded: true,
            maintain_order: false,
            limit: None,
        };
        self.0.is_sorted(options).map_err(polars_to_rpolars_err)
    }

    pub fn equals(&self, other: &RPolarsSeries, null_equal: bool, strict: bool) -> bool {
        if strict && (self.0.dtype() != other.0.dtype()) {
            return false;
        }
        if null_equal {
            self.0.equals_missing(&other.0)
        } else {
            self.0.equals(&other.0)
        }
    }

    //panics, if index out of bound
    fn get_fmt(&self, index: u32, str_length: u32) -> String {
        let val = format!(
            "{}",
            self.0.get(index.try_into().expect("usize>u32")).unwrap()
        );
        if let DataType::String | DataType::Categorical(_, _) = self.0.dtype() {
            let v_trunc = &val[..val
                .char_indices()
                .take(str_length.try_into().expect("usize>u32"))
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(0)];
            if val == v_trunc {
                val
            } else {
                format!("{v_trunc}…",)
            }
        } else {
            val
        }
    }

    fn to_fmt_char(&self, str_length: u32) -> Vec<String> {
        let len = self.0.len();
        let mut res = Vec::with_capacity(len);
        for i in 0..len {
            res.push(self.get_fmt(i.try_into().expect("usize>u32"), str_length));
        }
        res
    }

    //names repeat_ as repeat is locked keyword in R
    pub fn rep(&self, n: Robj, rechunk: Robj) -> std::result::Result<RPolarsSeries, String> {
        use crate::robj_to;
        self.rep_impl(robj_to!(usize, n)?, robj_to!(bool, rechunk)?)
            .map_err(|err| format!("{:?}", err))
    }

    pub fn shape(&self) -> Robj {
        r!([self.0.len() as f64, 1.0])
    }

    pub fn len(&self) -> f64 {
        self.0.len() as f64
    }

    pub fn chunk_lengths(&self) -> Vec<f64> {
        self.0.chunk_lengths().map(|val| val as f64).collect()
    }

    pub fn n_chunks(&self) -> f64 {
        self.0.n_chunks() as f64
    }

    pub fn alias(&self, name: &str) -> RPolarsSeries {
        let mut s = self.0.clone();
        s.rename(name.into());
        RPolarsSeries(s)
    }

    pub fn all(&self) -> List {
        let mut one_not_true = false;
        let result = || -> std::result::Result<bool, Box<dyn std::error::Error>> {
            for i in self.0.bool()?.into_iter() {
                if let Some(b) = i {
                    if b {
                        continue;
                    }
                }
                one_not_true = true;
                break;
            }
            Ok(!one_not_true)
        }()
        .map_err(|err| format!("{:?}", err));
        r_result_list(result)
    }

    pub fn any(&self) -> std::result::Result<bool, String> {
        use polars::prelude::*;
        if *self.0.dtype() == DataType::Boolean {
            let mut one_seen_true = false;

            let iter = self.0.bool().unwrap().into_iter();

            for i in iter.flatten() {
                if i {
                    one_seen_true = true;
                    break;
                }
            }

            Ok(one_seen_true)
        } else {
            Err("Series DataType is not a bool".to_string())
        }
    }

    pub fn append_mut(&mut self, other: &RPolarsSeries) -> List {
        r_result_list(
            self.0
                .append(&other.0)
                .map(|_| ())
                .map_err(|err| format!("{:?}", err)),
        )
    }

    pub fn add(&self, other: &RPolarsSeries) -> RResult<Self> {
        Ok((&self.0 + &other.0)
            .map(Into::into)
            .map_err(polars_to_rpolars_err)?)
    }

    pub fn sub(&self, other: &RPolarsSeries) -> RResult<Self> {
        Ok((&self.0 - &other.0)
            .map(Into::into)
            .map_err(polars_to_rpolars_err)?)
    }

    pub fn mul(&self, other: &RPolarsSeries) -> RResult<Self> {
        Ok((&self.0 * &other.0)
            .map(Into::into)
            .map_err(polars_to_rpolars_err)?)
    }

    pub fn div(&self, other: &RPolarsSeries) -> RResult<Self> {
        Ok((&self.0 / &other.0)
            .map(Into::into)
            .map_err(polars_to_rpolars_err)?)
    }

    pub fn rem(&self, other: &RPolarsSeries) -> RResult<Self> {
        Ok((&self.0 % &other.0)
            .map(Into::into)
            .map_err(polars_to_rpolars_err)?)
    }

    pub fn map_elements(
        &self,
        robj: Robj,
        rdatatype: Nullable<&RPolarsDataType>,
        strict: bool,
        allow_fail_eval: bool,
    ) -> list::List {
        //prepare lamda function from R side

        let rfun = if let Some(rfun) = robj.as_function() {
            rfun
        } else {
            return r_error_list("fun arg must be a function");
        };

        //function to wrap lambda to only pass the appropiate R NA type when polars null
        #[allow(unused_assignments)] //is actually used via macros
        let mut na_fun = R!(
            "function(x) stop('wait im just a mut placeholder function to extend the lifetime')"
        ) // actual function is set with apply_input! calling make_r_na_fun!
        .unwrap()
        .as_function()
        .unwrap();

        let inp_type = self.0.dtype();
        let out_type = null_to_opt(rdatatype).map_or_else(|| self.0.dtype(), |rdt| &rdt.0);

        //handle any input type to lambda, make iterator which yields lambda returns as Robj's
        use pl::DataType::*;
        let r_iter: Box<dyn Iterator<Item = Option<Robj>>> = match inp_type {
            Float64 => apply_input!(self.0, f64, rfun, na_fun),
            Float32 => apply_input!(self.0, f32, rfun, na_fun),
            Int64 => apply_input!(self.0, i64, rfun, na_fun),
            Int32 => apply_input!(self.0, i32, rfun, na_fun),
            Int16 => apply_input!(self.0, i16, rfun, na_fun),
            Int8 => apply_input!(self.0, i8, rfun, na_fun),
            String => apply_input!(self.0, str, rfun, na_fun),
            Boolean => apply_input!(self.0, bool, rfun, na_fun),
            //List(..) => apply_input!(self.0, list, rfun, na_fun),
            List(..) => {
                let ca_list = self.0.list().unwrap();

                let y = ca_list.into_iter().map(|opt_ser| {
                    if let Some(ser) = opt_ser {
                        rfun.call(pairlist!(RPolarsSeries(ser))).ok()
                    } else {
                        unreachable!("internal error: oh it was possible to get a None Series");
                    }
                });

                Box::new(y)
            }
            x => {
                dbg!(x);
                todo!("this input type is not implemented")
            }
        };

        //handle any return type from R and collect into Series
        let s: extendr_api::Result<RPolarsSeries> = {
            match out_type {
                Float64 => apply_output!(r_iter, strict, allow_fail_eval, Doubles, Float64Chunked),
                Int32 => apply_output!(r_iter, strict, allow_fail_eval, Integers, Int32Chunked),
                String => apply_output!(r_iter, strict, allow_fail_eval, Strings, StringChunked),
                Boolean => apply_output!(r_iter, strict, allow_fail_eval, Logicals, BooleanChunked),
                List(..) => {
                    //ierate over R return values, opt if never run (no values), err if fail
                    let mut all_length_one = true;
                    let xx = r_iter.map(|opt_r| -> pl::PolarsResult<_> {
                        if let Some(robj) = opt_r {
                            //convert Robj of Series or something "into series" to pl Series
                            let s = RPolarsSeries::any_robj_to_pl_series_result(robj)?;

                            if s.len() > 1 {
                                all_length_one = false;
                            }

                            Ok(Some(s)) //return Ok some polars series if success
                        } else {
                            Ok(None) //return Ok None if computation never took place
                        }
                    });

                    let lc_res: pl::PolarsResult<ListChunked> = xx.collect::<pl::PolarsResult<_>>();

                    let s: extendr_api::Result<RPolarsSeries> = lc_res
                        .map(|lc| lc.into_series())
                        .and_then(|s| if all_length_one { s.explode() } else { Ok(s) })
                        .map(RPolarsSeries)
                        .map_err(|e| extendr_api::error::Error::Other(e.to_string()));

                    s
                }

                _ => todo!("this output type is not implemented"),
            }
        };

        let s = s.map(move |mut x| {
            x.rename_mut(&format!("{}_apply", &self.name()));
            x
        });

        //if ok rename with prefix apply, convert Result<RPolarsSeries> in r_result_list
        r_result_list(s)
    }

    // TODO: add the int64 conversion option
    pub fn mean(&self) -> Result<Robj, String> {
        match self.0.dtype() {
            DataType::Boolean => {
                let s = self
                    .0
                    .cast(&DataType::UInt8)
                    .map_err(polars_to_rpolars_err)?
                    .mean_reduce()
                    .into_series("".into());
                RPolarsSeries(s).to_r("double")
            }
            DataType::Datetime(_, _) | DataType::Duration(_) | DataType::Time => {
                let s = self.0.mean_reduce().into_series("".into());
                RPolarsSeries(s).to_r("double")
            }
            _ => Ok(self.0.mean().into()),
        }
    }

    pub fn median(&self) -> Result<Robj, String> {
        match self.0.dtype() {
            DataType::Boolean => {
                let s = self
                    .0
                    .cast(&DataType::UInt8)
                    .map_err(polars_to_rpolars_err)?
                    .median_reduce()
                    .map_err(polars_to_rpolars_err)?
                    .into_series("".into());
                RPolarsSeries(s).to_r("double")
            }
            DataType::Datetime(_, _) | DataType::Duration(_) | DataType::Time => {
                let s = self
                    .0
                    .median_reduce()
                    .map_err(polars_to_rpolars_err)?
                    .into_series("".into());
                RPolarsSeries(s).to_r("double")
            }
            _ => Ok(self.0.median().into()),
        }
    }

    pub fn min(&self) -> Result<Robj, String> {
        RPolarsSeries(
            self.0
                .min_reduce()
                .map_err(polars_to_rpolars_err)?
                .into_series("".into()),
        )
        .to_r("double")
    }

    pub fn max(&self) -> Result<Robj, String> {
        RPolarsSeries(
            self.0
                .max_reduce()
                .map_err(polars_to_rpolars_err)?
                .into_series("".into()),
        )
        .to_r("double")
    }

    pub fn sum(&self) -> Result<Robj, String> {
        RPolarsSeries(
            self.0
                .sum_reduce()
                .map_err(polars_to_rpolars_err)?
                .into_series("".into()),
        )
        .to_r("double")
    }

    pub fn std(&self, ddof: Robj) -> Result<Robj, String> {
        let ddof = robj_to!(u8, ddof)?;

        RPolarsSeries(
            self.0
                .std_reduce(ddof)
                .map_err(polars_to_rpolars_err)?
                .into_series("".into()),
        )
        .to_r("double")
    }

    pub fn var(&self, ddof: Robj) -> Result<Robj, String> {
        let ddof = robj_to!(u8, ddof)?;

        RPolarsSeries(
            self.0
                .var_reduce(ddof)
                .map_err(polars_to_rpolars_err)?
                .into_series("".into()),
        )
        .to_r("double")
    }

    pub fn print(&self) {
        rprintln!("{:#?}", self.0);
    }

    pub fn to_frame(&self) -> std::result::Result<RPolarsDataFrame, String> {
        let mut df = RPolarsDataFrame::new_with_capacity(1);
        df.set_column_from_series(self)?;
        Ok(df)
    }

    pub fn set_sorted_mut(&mut self, descending: bool) {
        if descending {
            self.0.set_sorted_flag(polars::series::IsSorted::Descending)
        } else {
            self.0.set_sorted_flag(polars::series::IsSorted::Ascending)
        };
    }

    fn struct_fields(&self) -> RResult<Vec<&str>> {
        let ca = self.0.struct_().map_err(polars_to_rpolars_err)?;
        Ok(ca
            .struct_fields()
            .iter()
            .map(|s| s.name().as_str())
            .collect())
    }

    pub fn export_stream(&self, stream_ptr: &str, compat_level: Robj) {
        let compat_level = robj_to!(CompatLevel, compat_level).unwrap();
        let field = self.0.field().to_arrow(compat_level);

        let iter_boxed = Box::new(OwnedSeriesIterator::new(self.0.clone(), compat_level));
        let mut stream = arrow::ffi::export_iterator(iter_boxed, field);
        let stream_out_ptr_addr: usize = stream_ptr.parse().unwrap();
        let stream_out_ptr = stream_out_ptr_addr as *mut arrow::ffi::ArrowArrayStream;
        unsafe {
            std::ptr::swap_nonoverlapping(
                stream_out_ptr,
                &mut stream as *mut arrow::ffi::ArrowArrayStream,
                1,
            );
        }
    }

    pub fn import_stream(stream_ptr: Robj) -> RResult<Self> {
        let stream_in_ptr_addr = robj_to!(usize, stream_ptr)?;
        let stream_in_ptr =
            unsafe { Box::from_raw(stream_in_ptr_addr as *mut arrow::ffi::ArrowArrayStream) };

        let mut stream = unsafe { arrow::ffi::ArrowArrayStreamReader::try_new(stream_in_ptr)? };
        let mut arrays: Vec<Box<dyn arrow::array::Array>> = Vec::new();
        while let Some(array_res) = unsafe { stream.next() } {
            arrays.push(array_res?);
        }

        let chunks = arrays.into_iter().collect::<Vec<_>>();
        let s = pl::Series::try_from((stream.field(), chunks)).map_err(polars_to_rpolars_err)?;

        Ok(s.into())
    }

    pub fn from_arrow_array_robj(name: Robj, array: Robj) -> Result<Self, String> {
        let name: PlSmallStr = robj_to!(str, name)?.into();
        let arr = crate::arrow_interop::to_rust::arrow_array_to_rust(array)?;

        match arr.dtype() {
            ArrowDataType::LargeList(_) => {
                let array = arr.as_any().downcast_ref::<pl::LargeListArray>().unwrap();

                let mut previous = 0;
                let mut fast_explode = true;
                for &o in array.offsets().as_slice()[1..].iter() {
                    if o == previous {
                        fast_explode = false;
                        break;
                    }
                    previous = o;
                }
                let mut out = unsafe { ListChunked::from_chunks(name.into(), vec![arr]) };
                if fast_explode {
                    out.set_fast_explode()
                }
                Ok(out.into_series().into())
            }
            _ => {
                let series_res: Result<pl::Series, pl::PolarsError> =
                    std::convert::TryFrom::try_from((name, arr));
                Ok(series_res.map_err(|err| err.to_string())?.into())
            }
        }
    }

    fn clear(&self) -> RResult<Self> {
        Ok(self.0.clear().into())
    }
}

//inner_from_robj only when used within Series, do not have to comply with extendr_api macro supported types
impl RPolarsSeries {
    pub fn inner_from_robj_clone(robj: &Robj) -> std::result::Result<Self, &'static str> {
        if robj.check_external_ptr_type::<RPolarsSeries>() {
            let x: RPolarsSeries =
                unsafe { &mut *robj.external_ptr_addr::<RPolarsSeries>() }.clone();
            Ok(x)
        } else {
            Err("expected Series")
        }
    }

    pub fn any_robj_to_pl_series_result(robj: Robj) -> pl::PolarsResult<pl::Series> {
        let r_s = R!("polars0:::result(polars0::as_polars_series({{&robj}}))")
            .map_err(|err| pl::PolarsError::ComputeError(err.to_string().into()))?;
        let s = if let Ok(s) = robj_to!(PLSeries, r_s) {
            s
        } else {
            RPolarsSeries::inner_from_robj_clone(&robj)
                .map_err(|err| {
                    //convert any error from R to a polars error
                    pl::PolarsError::ComputeError(err.into())
                })?
                .0
        };
        Ok(s)
    }

    pub fn rep_impl(&self, n: usize, rechunk: bool) -> pl::PolarsResult<Self> {
        if n == 0 {
            return Ok(RPolarsSeries(self.clone().0.slice(0, 0)));
        }
        let mut s = self.0.clone();
        for _ in 1..n {
            s.append(&self.0)?;
        }
        if rechunk {
            s = s.rechunk();
        }
        Ok(RPolarsSeries(s))
    }

    pub unsafe fn into_frame(&self) -> RPolarsDataFrame {
        RPolarsDataFrame(self.0.clone().into_frame().clone())
    }
}

impl From<&RPolarsSeries> for pl::Series {
    fn from(x: &RPolarsSeries) -> Self {
        x.clone().0
    }
}

extendr_module! {
    mod series;
    impl RPolarsSeries;
}
