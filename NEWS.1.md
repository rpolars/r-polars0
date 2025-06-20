# NEWS (pre-v1.0.0, the package name was `polars`)

> [!IMPORTANT]
>
> This package is now in maintenance mode,
> meaning that important bugs will be fixed if possible but we won't adding new features in the next few weeks / months.
>
> The focus of future development is on a completely rewritten version (`neopolars`) that currently exists in the `next` branch.
> The package in that branch will become the new `polars` package once the rewrite is complete.
>
> Please check the GitHub issue #1152.

## Polars R Package (development version)

## Polars R Package 0.22.4

Candidate for last version based on current code base.

### Bug fixes

- Changes related to build to pass R CMD check on R 4.5.0 (#1357).

## Polars R Package 0.22.3

### Bug fixes

- Some non-API calls for R were removed to pass `R CMD check` on R 4.5.0 (#1339).
- The name of the Series is now correctly exported and imported when using the Arrow C stream interface
  via the `nanoarrow_array_stream` object (#1347).

## Polars R Package 0.22.2

### Bug fixes

- Functions to convert Polars to R (`<Series>$to_r()` etc.) no longer fail with
  the "NA in coercion to boolean" error in R 4.5 (#1337).

## Polars R Package 0.22.1

This is a small hot-fix release to fix the build error on R-multiverse.
No user-facing changes.

## Polars R Package 0.22.0

### Breaking changes

- Updated Rust Polars to 0.45.1 (#1302).
  - The `ddof` argument of `pl$corr()` is removed.

## Polars R Package 0.21.0

### Breaking changes

- Updated Rust Polars to 0.44.2 (#1271).
  - Minimum supported Rust version (MSRV) is now 1.82.0.
  - `$reshape()`'s `nested_type` argument is removed.
  - `$approx_n_unique()` no longer works on Categorical type.
- `<Series>$compare()` is removed. (#1272)

### Deprecations

- Passing a single data.frame to `pl$DataFrame()` or `pl$LazyFrame()` to convert a
  data.frame to a polars DataFrame or LazyFrame is deprecated and a warning will
  be shown. Use `as_polars_df()` or `as_polars_lf()` instead (#1275).

### Bug fixes

- Maintain level order when converting Enums to factors (#1252, @andyquinterom).

## Polars R Package 0.20.0

- Updated Rust Polars to 0.43.1 (#1230).

### Breaking changes

- In `pl$scan_ipc()` and `pl$read_ipc()`, the argument `memory_map` is removed
  (#1230).
- In `$serialize()`, in the field `schema`, the field `inner` is renamed `fields`,
  and the fields `output_schema` and `filter` are removed (#1230).

### New features

- New method `$cast()` for `DataFrame` and `LazyFrame` (#1219).
- New argument `strict` in `$drop()` to determine whether unknown column names
  should trigger an error (#1220).
- New method `$to_dummies()` for `DataFrame` (#1225).
- New argument `include_file_paths` in `pl_scan_csv()` and `pl_read_csv()` (#1235).
- New method `$join_where()` for `DataFrame` and `LazyFrame` to perform
  inequality joins (#1237).

### Bug fixes

- Converting data of datatype `Null` to R doesn't error anymore. It now creates
  a column filled with `NA` (#1217).

## Polars R Package 0.19.1

- This is a maintenance release. No user facing changes.

## Polars R Package 0.19.0

### Breaking changes

- Updated Rust Polars to unreleased 2024-08-20, after 0.42.0 (#1183).
- `$describe_plan()` and `$describe_optimized_plan()` are removed. Use
  respectively `$explain(optimized = FALSE)` and `$explain()` instead (#1182).
- The parameter `inherit_optimization` is removed from all functions that had it
  (#1183).
- In `$write_parquet()` and `$sink_parquet()`, the parameter `data_pagesize_limit`
  is renamed `data_page_size` (#1183).
- The LazyFrame method `$get_optimization_toggle()` is removed, and
  `$set_optimization_toggle()` is renamed `$optimization_toggle()` (#1183).
- In `$unpivot()`, the parameter `streamable` is removed (#1183).
- Some functions have a parameter `future` that determines the compatibility level
  when exporting Polars' internal data structures. This parameter is renamed
  `compat_level`, which takes `FALSE` for the oldest flavor (more compatible)
  and `TRUE` for the newest one (less compatible). It can also take an integer
  determining a specific compatibility level when more are added in the future.
  For now, `future = FALSE` can be replaced by `compat_level = FALSE` (#1183).
- In `$scan_parquet()` and `$read_parquet()`, the default value of
  `hive_partitioning` is now `NULL` (#1189).
- In `$dt$epoch()`, the argument `tu` is renamed to `time_unit` (#1196).
- In `$fill_nan()` for `DataFrame`, `LazyFrame` and `Expr`, the argument is
  renamed `value` (#1198).
- `$shift_and_fill()` is removed and replaced by a new argument `fill_value` in
  `$shift()`. `$shift_and_fill(fill_value, periods)` can be replaced by
  `$shift(n, fill_value)` (#1201).
- In `$shift()` for various `Expr`, the argument `periods` is renamed `n` (#1201).
- In `$clip()`, arguments `min` and `max` are renamed `lower_bound` and
  `upper_bound` (#1203).
- `$clip_min()` and `$clip_max()` are removed. Use `$clip()` with only
  `lower_bound` or `upper_bound` instead (#1203).
- In `$write_csv` and `$sink_csv()`, the argument `quote` is renamed
  `quote_char` (#1206).

### New features

- New method `$str$extract_many()` (#1163).
- Converting a `nanoarrow_array` with zero rows to an `RPolarsDataFrame` via
  `as_polars_df()` now keeps the original schema (#1177).
- `$write_parquet()` has two new arguments `partition_by` and
  `partition_chunk_size_bytes` to write a `DataFrame` to a hive-partitioned
  directory (#1183).
- New method `$bin$size()` (#1183).
- In `$scan_parquet()` and `$read_parquet()`, the `parallel` argument can take
  the new value `"prefiltered"` (#1183).
- `$scan_parquet()`, `$scan_ipc()` and `$read_parquet()` have a new argument
  `include_file_paths` to automatically add a column containing the path to the
  source file(s) (#1183).
- `$scan_ipc()` can read a hive-partitioned directory with its new arguments
  `hive_partitioning`, `hive_schema`, and `try_parse_hive_dates` (#1183).
- `$scan_parquet()` and `$read_parquet()` gain two new arguments for more control
  on importing hive partitions: `hive_schema` and `try_parse_hive_dates` (#1189).
- New method `$gather_every()` for `LazyFrame` and `DataFrame` (#1199).
- `$glimpse()` for `DataFrame` has two new arguments `max_items_per_column` and
  `max_colname_length` (#1200).
- New method `$list$sample()` (#1204).
- New argument `coalesce` in `$join_asof()` (#1205).
- New argument `maintain_order` in `$list$unique()` (#1207).

### Other changes

- In `$unnest()` for `DataFrame` and `LazyFrame`, the `names` argument is removed
  and replaced by `...`. This doesn't change the previous behavior, e.g.
  `df$unnest(names = c("a", "b"))` still works (#1170).

## Polars R Package 0.18.0

### Breaking changes

- Updated Rust Polars to 0.41.3 (#1147, #1156).
- In `$n_chunks()`, the default value of `strategy` now is `"first"` (#1137).
- `$sample()` for Expr and DataFrame (#1136):
  - the argument `frac` is renamed `fraction`;
  - all the arguments except `n` must be named;
  - for the Expr method only, the first argument is now `n` (it was already the
    case for the DataFrame method);
  - for the Expr method only, the default value for `with_replacement` is now
    `FALSE` (it was already the case for the DataFrame method).
- `$melt()` had several changes (#1147):
  - `melt()` is renamed `$unpivot()`.
  - Some arguments were renamed: `id_vars` is now `index`, `value_vars` is now
    `on`.
  - The order of arguments has changed: `on` is now first, then `index`. The
    order of the other arguments hasn't changed. Note that `on` can be unnamed
    but all the other arguments must be named.
- `pivot()` had several changes (#1147):
  - The argument `columns` is renamed `on`.
  - The order of arguments has changed: `on` is now first, then `index` and
    `values`. The order of the other arguments hasn't changed. Note that `on`
    can be unnamed but all the other arguments must be named.
- In `$write_parquet()` and `$sink_parquet()`, the default value of argument
  `statistics` is now `TRUE` and can take other values than `TRUE/FALSE` (#1147).
- In `$dt$truncate()` and `$dt$round()`, the argument `offset` has been removed.
  Use `$dt$offset_by()` after those functions instead (#1147).
- In `$top_k()` and `$bottom_k()` for `Expr`, the arguments `nulls_last`,
  `maintain_order` and `multithreaded` have been removed. If any `null` values
  are in the top/bottom `k` values, they will always be positioned last (#1147).
- `$replace()` has been split in two functions depending on the desired
  behaviour (#1147):
  - `$replace()` recodes some values in the column, leaving all other values
    unchanged. Compared to the previous version, it doesn't use the arguments
    `default` and `return_dtype` anymore.
  - `$replace_strict()` replaces all values by different values. If a value
    doesn't have a specific mapping, it is replaced by the `default` value.
- `$str$concat()` is deprecated, use `$str$join()` (with the same arguments)
  instead (#1147).
- In `pl$date_range()` and `pl$date_ranges()`, the arguments `time_unit` and
  `time_zone` have been removed. They were deprecated in previous versions
  (#1147).
- In `$join()`, when `how = "cross"`, `on`, `left_on` and `right_on` must be
  `NULL` (#1147).

### New features

- New method `$has_nulls()` (#1133).
- New method `$list$explode()` (#1139).
- `$over()` gains a new argument `order_by` to specify the order of values
  within each group. This is useful when the operation depends on the order of
  values, such as `$shift()` (#1147).
- `$value_counts()` gains an argument `normalize` to give relative frequencies
  of unique values instead of their count (#1147).

## Polars R Package 0.17.0

### Breaking changes

- Updated Rust Polars to unreleased version (> 0.40.0) (#1104, #1110, #1117, #1124):
  - In `$join()`, there is a new argument `coalesce` and the `how` options now
    accept `"full"` instead of `"outer"` and `"outer_coalesce"`.
  - `$top_k()` and `$bottom_k()` gain three arguments `nulls_last`,
    `maintain_order` and `multithreaded`.
  - All `$rolling_*()` functions lose the arguments `by`, `closed` and
    `warn_if_unsorted`. Rolling computations based on `by` must be made via the
    corresponding `rolling_*_by()`, e.g `rolling_mean_by()` instead of
    `rolling_mean(by =)` (#1115).
  - `pl$scan_parquet()` and `pl$read_parquet()` gain an argument `glob` which
    defaults to `TRUE`. Set it to `FALSE` to avoid considering `*` as a globing
    pattern.
  - `$is_not_nan()` on a `null` value (`NA` in R) now returns `null`. Previously,
    it returned `TRUE`.
  - In `$reshape()`, argument `dims` is renamed `dimensions` and there is a new
    argument `nested_type` specifying if the output should be of type List or
    Array.
  - In `$value_counts()`, all arguments must be named and there is a new argument
    `name` to specify the name of the output.
  - In all functions accepting optimization parameter (such as
    `projection_pushdown`), there is a new parameter `cluster_with_columns` to
    combine sequential independent calls to `$with_columns()`.
  - `$str$explode()` is removed.
  - The `check_sorted` argument is removed from `$rolling()` and `$group_by_dynamic()`.
    Sortedness is now verified in a quick manner, so this argument is no longer needed
    (pola-rs/polars#16494).
  - `$name$map()` stacks on Linux, so this method is deprecated and the document
    is removed. Please use other methods like `<LazyFrame>$rename(<function>)` instead (#1123).
- As warned in v0.16.0, the order of arguments in `pl$Series` is changed (#1071).
  The first argument is now `name`, and the second argument is `values`.
- `$to_struct()` on an Expr is removed. This method is now only available for
  `Series`, `DataFrame`, and in the `$list` and `$arr` subnamespaces. For example,
  `pl$col("a", "b", "c")$to_struct()` should be replaced with
  `pl$struct(c("a", "b", "c"))` (#1092).
- `pl$Struct()` now only accepts named inputs and objects of class `RPolarsField`.
  For example, `pl$Struct(pl$Boolean)` doesn't work anymore and should be named
  like `pl$Struct(a = pl$Boolean)` (#1053).
- In `$all()` and `$any()`, the argument `drop_nulls` is renamed `ignore_nulls`,
  and this argument must be named (#1050).
- New method `$struct$with_fields()` (#1109) and new function `pl$field()` to
  be used in expressions in `$struct$with_fields()` (#1113).
- New methods for `RPolarsDataType`: `$is_enum()`, `$is_categorical()`,
  `$is_known()`, `$is_string()`, `$contains_views()`, `$contains_categorical()`
  (#1112).
- In `$dt$combine()`, the arguments `tm` and `tu` are renamed `time` and
  `time_unit` (#1116).
- The default value of the `rechunk` argument of `pl$concat()` is changed from
  `TRUE` to `FALSE` (#1125).
- In `$rename()` for LazyFrame and DataFrame, key-value pairs of names are changed to
  `old_name = "new_name"` instead of `new_name = "old_name"` (#1129).
- In `$rename()` for LazyFrame and DataFrame, no argument is not allowed (#1129).
- In all `$rolling_*()` functions, the arguments `center` and `ddof` must be
  named (#1115).

### New features

- Allow specify a function in `$rename()` for LazyFrame and DataFrame.
  They are equivalent to `polars.LazyFrame.rename(mapping: Callable[[str], str])`
  or `polars.DataFrame.rename(mapping: Callable[[str], str])` in Python Polars (#1122, #1129).

## Polars R Package 0.16.4

### New features

- `pl$read_ipc()` can read a raw vector of Apache Arrow IPC file (#1072).
- New method `<DataFrame>$to_raw_ipc()` to serialize a DataFrame to a raw vector
  of Apache Arrow IPC file format (#1072).
- New method `<LazyFrame>$serialize()` to serialize a LazyFrame to a character
  vector of JSON representation (#1073).
- New function `pl$deserialize_lf()` to deserialize a LazyFrame from a character
  vector of JSON representation (#1073).
- New methods `$str$head()` and `$str$tail()` (#1074).
- New S3 methods `nanoarrow::as_nanoarrow_array_stream()` and `nanoarrow::infer_nanoarrow_schema()`
  for `RPolarsSeries` (#1076).
- New method `$dt$is_leap_year()` (#1077).
- `as_polars_df()` and `as_polars_series()` supports `arrow::RecordBatchReader` (#1078).
- The new `experimental` argument for `as_polars_df(<ArrowTabular>)`, `as_polars_df(<RecordBatchReader>)`,
  `as_polars_series(<nanoarrow_array_stream>)`, and `as_polars_df(<nanoarrow_array_stream>)` (#1078).
  If `experimental = TRUE`, these functions switch to use
  [the Arrow C stream interface](https://arrow.apache.org/docs/format/CStreamInterface.html) internally.
  At this point, the performance is degraded under the expected use cases,
  so the default is set to `experimental = FALSE`.

## Polars R Package 0.16.3

### New features

- New method `<SQLContext>$register_globals()` (#1064).
- New experimental method `$sql()` for DataFrame and LazyFrame (#1065).

### Miscellaneous

- Move the API document website to the new place (#1067, #1068).
  Access to the old website is set to redirect to the top page of the new website.
  - Old URL: `https://rpolars.github.io/`
  - New URL: `https://pola-rs.github.io/r-polars/`

## Polars R Package 0.16.2

### New features

- `$cut()` and `$qcut()` to bin continuous values into discrete categories (#1057).
- `pl$scan_parquet()` and `pl$read_parquet()` can read data from the internet by specifying a URL
  to the first argument (#1056, @andyquinterom).
- `pl$scan_parquet()` and `pl$read_parquet()` gain an argument `storage_options`
  to scan/read data via cloud storage providers (GCP, AWS, Azure). Note that this
  support is experimental (#1056, @andyquinterom).
- Add support for the `Enum` datatype via `pl$Enum()` (#1061).

### Bug fixes

- In some read/scan functions, downloading files could fail if the URL was too
  long. This is now fixed (#1049, @DyfanJones).

## Polars R Package 0.16.1

This is a small hot-fix release to update dependent Rust Polars to 0.39.1 (#1042).

Also, there are some updates.

### Bug fixes

- `$len()` now correctly includes `null` values in the count (#1044).

### Other improvements

- `$arr$max()` and `$arr$min()` work without the `nightly` feature (#1042).

## Polars R Package 0.16.0

### Breaking changes

- Rust Polars is updated to 0.39.0 (#937, #1034).
- R objects inside an R list are now converted to Polars data types via
  `as_polars_series()` (#1021, #1022, #1023). For example, up to polars 0.15.1,
  a list containing a data.frame with a column of `{clock}` naive-time class
  was converted to a nested List type of Float64:

  ```r
  data = data.frame(time = clock::naive_time_parse("1990-01-01", precision = "day"))
  pl$select(
    nested_data = pl$lit(list(data))
  )
  #> shape: (1, 1)
  #> ┌──────────────────────────┐
  #> │ nested_data              │
  #> │ ---                      │
  #> │ list[list[list[f64]]]    │
  #> ╞══════════════════════════╡
  #> │ [[[2.1475e9], [7305.0]]] │
  #> └──────────────────────────┘
  ```

  From 0.16.0, nested types are correctly converted, so that will be
  a List type of Struct type containing a Datetime type.

  ```r
  data = data.frame(time = clock::naive_time_parse("1990-01-01", precision = "day"))
  pl$select(
    nested_data = pl$lit(list(data))
  )
  #> shape: (1, 1)
  #> ┌─────────────────────────┐
  #> │ nested_data             │
  #> │ ---                     │
  #> │ list[struct[1]]         │
  #> ╞═════════════════════════╡
  #> │ [{1990-01-01 00:00:00}] │
  #> └─────────────────────────┘
  ```

- Several functions have been rewritten to match the behavior of Python Polars.
  There are four types of changes: a) change in argument names, b) change in
  the way arguments are passed (named or by position), c) arguments are removed,
  and d) change in the default and accepted values. Those are addressed separately
  below.

  1. Change in argument names:

     - In `$reshape()`, the `dims` argument is renamed to `dimensions` (#1019).
     - In `pl$read_*` and `pl$scan_*` functions, the first argument is now
       `source` (#935).
     - In `pl$Series()`, the argument `x` is renamed `values` (#933).
     - In `<DataFrame>$write_*` functions, the first argument is now `file` (#935).
     - In `<LazyFrame>$sink_*` functions, the first argument is now `path` (#935).
     - In `<LazyFrame>$sink_ipc()`, the argument `memmap` is renamed to `memory_map` (#1032).
     - In `<DataFrame>$rolling()`, `<LazyFrame>$rolling()`, `<DataFrame>$group_by_dynamic()`
       and `<LazyFrame>$group_by_dynamic()`, the `by` argument is renamed to
       `group_by` (#983).
     - In `$dt$convert_time_zone()` and `$dt$replace_time_zone()`, the `tz`
       argument is renamed to `time_zone` (#944).
     - In `$str$strptime()`, the argument `datatype` is renamed to `dtype` (#939).
     - In `$str$to_integer()` (renamed from `$str$parse_int()`), argument `radix` is
       renamed to `base` (#1038).

  2. Change in the way arguments are passed:

     - In all input/output functions, all arguments except the first argument
       must be named arguments (#935).
     - In `<DataFrame>$rolling()` and `<DataFrame>$group_by_dynamic()`, all
       arguments except `index_column` must be named arguments (#983).
     - In `$unique()` for `DataFrame` and `LazyFrame`, arguments `keep` and
       `maintain_order` must be named (#953).
     - In `$bin$decode()`, the `strict` argument must be a named argument (#980).
     - In `$dt$replace_time_zone()`, all arguments except `time_zone` must be named
       arguments (#944).
     - In `$str$contains()`, the arguments `literal` and `strict` must be named
       (#982).
     - In `$str$contains_any()`, the `ascii_case_insensitive` argument must be
       named (#986).
     - In `$str$count_matches()`, `$str$replace()` and `$str$replace_all()`,
       the `literal` argument must be named (#987).
     - In `$str$strptime()`, `$str$to_date()`, `$str$to_datetime()`, and
       `$str$to_time()`, all arguments (except the first one) must be named (#939).
     - In `$str$to_integer()` (renamed from `$str$parse_int()`), all arguments
       must be named (#1038).
     - In `pl$date_range()`, the arguments `closed`, `time_unit`, and `time_zone`
       must be named (#950).
     - In `$set_sorted()` and `$sort_by()`, argument `descending` must be named
       (#1034).
     - In `pl$Series()`, using positional arguments throws a warning, since the
       argument positions will be changed in the future (#966).

       ```r
       # polars 0.15.1 or earlier
       # The first argument is `x`, the second argument is `name`.
       pl$Series(1:3, "foo")

       # The code above will warn in 0.16.0
       # Use named arguments to silence the warning.
       pl$Series(values = 1:3, name = "foo")
       pl$Series(name = "foo", values = 1:3)

       # polars 0.17.0 or later (future version)
       # The first argument is `name`, the second argument is `values`.
       pl$Series("foo", 1:3)
       ```

       This warning can also be silenced by replacing `pl$Series(<values>, <name>)`
       by `as_polars_series(<values>, <name>)`.

  3. Arguments removed:

     - The argument `columns` in `$drop()` is removed. `$drop()` now accepts
       several character scalars, such as `$drop("a", "b", "c")` (#912).
     - In `pl$col()`, the `name` argument is removed, and the `...` argument no
       longer accepts a list of characters and `RPolarsSeries` class objects (#923).
     - In `pl$date_range()`, the unused argument (not working in recent versions)
       `explode` is removed. (#950).

  4. Change in arguments default and accepted values:

     - In `pl$Series()`, the argument `values` has a new default value `NULL`
       (#966).
     - In `$unique()` for `DataFrame` and `LazyFrame`, argument `keep` has a new
       default value `"any"` (#953).
     - In rolling aggregation functions (such as `$rolling_mean()`), the default
       value of argument `closed` now is `NULL`. Using `closed` with a fixed
       `window_size` now throws an error (#937).
     - In `pl$date_range()`, the argument `end` must be specified and the default
       value of `interval` is changed to `"1d"`. The arguments `start` and `end`
       no longer accept numeric values (#950).
     - In `pl$scan_parquet()`, the default value of the argument `rechunk` is
       changed from `TRUE` to `FALSE` (#1033).
     - In `pl$scan_parquet()` and `pl$read_parquet()`, the argument `parallel`
       only accepts `"auto"`, `"columns"`, `"row_groups"`, and `"none"`.
       Previously, it also accepted upper-case notation of `"auto"`, `"columns"`,
       `"none"`, and `"RowGroups"` instead of `"row_groups"` (#1033).
     - In `$str$to_integer()` (renamed from `$str$parse_int()`), the default
       value of `base` is changed from `2` to `10` (#1038).

- The usage of `pl$date_range()` to create a range of `Datetime` data type is
  deprecated. `pl$date_range()` will always create a range of `Date` data type
  in the future. Use `pl$datetime_range()` if you want to create a range of
  `Datetime` instead (#950).
- `<DataFrame>$get_columns()` now returns an unnamed list instead of a named
  list (#991).
- Removed `$argsort()` which was an old alias for `$arg_sort()` (#930).
- Removed `pl$expr_to_r()` which was an alias for `$to_r()` (#938).
- `<Series>$to_r_list()` is renamed `<Series>$to_list()` (#938).
- Removed `<Series>$to_r_vector()` which was an old alias for
  `<Series>$to_vector()` (#938).
- Removed `<Expr>$rep_extend()`, which was an experimental method created at the
  early stage of this package and does not exist in other language APIs (#1028).
- The following deprecated functions are now removed: `pl$threadpool_size()`,
  `<DataFrame>$with_row_count()`, `<LazyFrame>$with_row_count()` (#965).
- In `$group_by_dynamic()`, the first datapoint is always preserved (#1034).
- `$str$parse_int()` is renamed to `$str$to_integer()` (#1038).

### New features

- New functions:

  - `pl$arg_sort_by()` (#929).
  - `pl$arg_where()` to get the indices that match a condition (#922).
  - `pl$datetime()`, `pl$date()`, and `pl$time()` to easily create Expr of class
    datetime, date, and time via columns and literals (#918).
  - `pl$datetime_range()`, `pl$date_ranges()` and `pl$datetime_ranges()` (#950, #962).
  - `pl$int_range()` and `pl$int_ranges()` (#968)
  - `pl$mean_horizontal()` (#959)
  - `pl$read_ipc()` (#1033).
  - `is_polars_dtype()` (#927).

- New methods:

  - `<LazyFrame>$to_dot()` to print the query plan of a LazyFrame with graphviz
    dot syntax (#928).
  - `$clear()` for `DataFrame`, `LazyFrame`, and `Series` (#1004).
  - `$item()` for `DataFrame` and `Series` (#992).
  - `$select_seq()` and `$with_columns_seq()` for `DataFrame` and `LazyFrame`
    (#1003).
  - `$arr$to_list()` (#1018).
  - `$str$extract_groups()` (#979).
  - `$str$find()` (#985).
  - `<DataFrame>$write_ipc()` (#1032).
  - `RPolarsDataType` gains several methods to check the datatype, such as
    `$is_integer()`, `$is_null()` or `$is_list()` (#1036).

- New arguments or argument values:

  - `ambiguous` can now take the value `"null"` to convert ambigous datetimes to
    null values (#937).
  - `n` in `$str$replace()` (#987).
  - `non_existent` in `$dt$replace_time_zone()` to specify what should happen
    when a datetime doesn't exist.
  - `mapping_strategy` in `$over()` (#984, #988).
  - `raise_if_undetermined` in `$meta$output_name()` (#961).
  - `null_on_oob` in `$arr$get()` and `$list$get()` to determine what happens
    when the index is out of bounds (#1034).
  - `nulls_last`, `multithreaded`, and `maintain_order` in `$sort_by()` (#1034).

- Other:

  - `pl$Series()` now calls `as_polars_series()` internally, so it can convert
    more classes to Series properly (#1015).
  - Export the `Duration` datatype (#955).
  - New active binding `<Series>$struct$fields` (#1002).
  - All `$write_*()` and `$sink_*()` functions now invisibly return the input
    data (#1039).

### Bug fixes

- The `join_nulls` and `validate` arguments of `<DataFrame>$join()` now work
  correctly (#945).
- We said in the changelog of 0.14.0 that all `row_count_*` args in I/O functions
  were renamed `row_index_*`, but this change was not made for CSV and IPC
  functions. This renaming is now made (#964).
- Evaluating `Series` methods from `Expr` inside functions now works correctly (#973).
  Thanks @Yunuuuu for the report.
- The dependent crate `extendr-api` is updated to 2024-03-31 unreleased version (#995).
  The issue that the R session crashes when a panic occurs in the Rust side is resolved.
  Thanks @CGMossa for the upstream fix.
- The `parallel` argument of `pl$scan_parquet()` and `pl$read_parquet()` now works
  correctly (#1033). Previously, any correct value was treated as `"auto"`.

## Polars R Package 0.15.1

### New features

- Rust Polars is updated to 0.38.2 (#907).
  - Minimum supported Rust version (MSRV) is now 1.76.0.
- `as_polars_df(<nanoarrow_array>)` is added (#893).
- It is now possible to create an empty `DataFrame` with a specific schema
  with `pl$DataFrame(schema = my_schema)` (#901).
- New arguments `dtype` and `nan_to_null` for `pl$Series()` (#902).
- New method `<DataFrame>$partition_by()` (#898).

### Bug fixes

- The default value of the `format` of `$str$strptime()` is now correctly set (#892).

### Other improvements

- Performance of `as_polars_df(<nanoarrow_array_stream>)` is improved (#896).

## Polars R Package 0.15.0

### Breaking changes due to Rust Polars update

- Rust Polars is updated to 0.38.1 (#865, #872).
  - in `$pivot()`, arguments `aggregate_function`, `maintain_order`,
    `sort_columns` and `separator` must be named. Values that are passed
    by position are ignored.
  - in `$describe()`, the name of the first column changed from `"describe"`
    to `"statistic"`.
  - `$mod()` methods and `%%` works correctly to guarantee
    `x == (x %% y) + y * (x %/% y)`.

### Other breaking changes

- Removed `as.list()` for class `RPolarsExpr` as it is a simple wrapper around
  `list()` (#843).
- Several functions have been rewritten to match the behavior of Python Polars.
  - `pl$col(...)` requires at least one argument. (#852)
  - `pl$head()`, `pl$tail()`, `pl$count()`, `pl$first()`, `pl$last()`, `pl$max()`,
    `pl$min()`, `pl$mean()`, `pl$media()`, `pl$std()`, `pl$sum()`, `pl$var()`,
    `pl$n_unique()`, and `pl$approx_n_unique()` are syntactic sugar for
    `pl$col(...)$<method()>`. The argument `...` now only accepts characters,
    that are either column names or regular expressions (#852).
  - There is no argument for `pl$len()`. If you want to measure the length of
    specific columns, you should use `pl$count(...)` (#852).
  - `<Expr>$str$concat()` method's `delimiter` argument's default value is
    changed from `"-"` to `""` (#853).
  - `<Expr>$str$concat()` method's `ignore_nulls` argument must be a
    named argument (#853).
  - `pl$Datetime()`'s arguments are renamed: `tu` to `time_unit`,
    and `tz` to `time_zone` (#887).
- `pl$Categorical()` has been improved to allow specifying the `ordering` type
  (either lexical or physical). This also means that calling `pl$Categorical`
  doesn't create a `DataType` anymore. All calls to `pl$Categorical` must be
  replaced by `pl$Categorical()` (#860).
- `<Series>$rem()` is removed. Use `<Series>$mod()` instead (#886).
- The conversion strategy between the POSIXct type without time zone attribute
  and Polars datetime has been changed (#878).
  `POSIXct` class vectors without a time zone attribute have UTC time internally
  and is displayed based on the system's time zone. Previous versions of `polars`
  only considered the internal value and interpreted it as UTC time, so the
  time displayed as `POSIXct` and in Polars was different.

  ```r
  # polars 0.14.1
  Sys.setenv(TZ = "Europe/Paris")
  datetime = as.POSIXct("1900-01-01")
  datetime
  #> [1] "1900-01-01 PMT"

  s = polars::as_polars_series(datetime)
  s
  #> polars Series: shape: (1,)
  #> Series: '' [datetime[ms]]
  #> [
  #>  1899-12-31 23:50:39
  #> ]

  as.vector(s)
  #> [1] "1900-01-01 PMT"
  ```

  Now the internal value is updated to match the displayed value.

  ```r
  # polars 0.15.0
  Sys.setenv(TZ = "Europe/Paris")
  datetime = as.POSIXct("1900-01-01")
  datetime
  #> [1] "1900-01-01 PMT"

  s = polars::as_polars_series(datetime)
  s
  #> polars Series: shape: (1,)
  #> Series: '' [datetime[ms]]
  #> [
  #>  1900-01-01 00:00:00
  #> ]

  as.vector(s)
  #> [1] "1900-01-01 PMT"
  ```

  This update may cause errors when converting from Polars to `POSIXct` for non-existent
  or ambiguous times. It is recommended to explicitly add a time zone before converting
  from Polars to R.

  ```r
  Sys.setenv(TZ = "America/New_York")
  ambiguous_time = as.POSIXct("2020-11-01 01:00:00")
  ambiguous_time
  #> [1] "2020-11-01 01:00:00 EDT"

  pls = polars::as_polars_series(ambiguous_time)
  pls
  #> polars Series: shape: (1,)
  #> Series: '' [datetime[ms]]
  #> [
  #>  2020-11-01 01:00:00
  #> ]

  ## This will be error!
  # pls |> as.vector()

  pls$dt$replace_time_zone("UTC") |> as.vector()
  #> [1] "2020-11-01 01:00:00 UTC"
  ```

- Removed argument `eager` in `pl$date_range()` and `pl$struct()` for more
  consistency of output. It is possible to replace `eager = TRUE` by calling
  `$to_series()` (#882).

### New features

- In the when-then-otherwise expressions, the last `$otherwise()` is now optional,
  as in Python Polars. If `$otherwise()` is not specified, rows that don't respect
  the condition set in `$when()` will be filled with `null` (#836).
- `<DataFrame>$head()` and `<DataFrame>$tail()` methods now support negative
  row numbers (#840).
- `$group_by()` now works with named expressions (#846).
- New methods for the `arr` subnamespace: `$median()`, `$var()`, `$std()`,
  `$shift()`, `$to_struct()` (#867).
- `$min()` and `max()` now work on categorical variables (#868).
- New methods for the `list` subnamespace: `$n_unique()`, `$gather_every()`
  (#869).
- Converts `clock_time_point` and `clock_zoned_time` objects from
  the `{clock}` package to Polars datetime type (#861).
- New methods for the `name` subnamespace: `$prefix_fields()` and
  `suffix_fields()` (#873).
- `pl$Datetime()`'s `time_zone` argument now accepts `"*"` to match
  any time zone (#887).

### Bug fixes

- R no longer crashes when calling an invalid Polars object that points
  to a null pointer (#874). This was occurring, such as when a Polars object
  was saved in an RDS file and loaded from another session.

## Polars R Package 0.14.1

### Breaking changes

- Since most of the methods of `Expr` are now available for `Series`, the
  experimental `<Series>$expr` subnamespace is removed (#831).
  Use `<Series>$<method>` instead of `<Series>$expr$<method>`.

### New features

- New active bindings `$flags` for `DataFrame` to show the flags used internally
  for each column. The output of `$flags` for `Series` was also improved and now
  contains `FAST_EXPLODE` for `Series` of type `list` and `array` (#809).
- Most of `Expr` methods are also available for `Series` (#819, #828, #831).
- `as_polars_df()` for `data.frame` is more memory-efficient and new arguments
  `schema` and `schema_overrides` are added (#817).
- Use `polars_code_completion_activate()` to enable code suggestions and
  autocompletion after `$` on polars objects. This is an experimental feature
  that is disabled by default. For now, it is only supported in the native R
  terminal and in RStudio (#597).

### Bug fixes

- `<Series>$list` sub namespace methods returns `Series` class object correctly (#819).

## Polars R Package 0.14.0

### Breaking changes due to Rust Polars update

- Rust Polars is updated to 0.37.0 (#776).
  - Minimum supported Rust version (MSRV) is now 1.74.1.
  - `$with_row_count()` for `DataFrame` and `LazyFrame` is deprecated and
    will be removed in 0.15.0. It is replaced by `$with_row_index()`.
  - `pl$count()` is deprecated and will be removed in 0.15.0. It is replaced
    by `pl$len()`.
  - `$explode()` for `DataFrame` and `LazyFrame` doesn't work anymore on
    string columns.
  - `$list$join()` and `pl$concat_str()` gain an argument `ignore_nulls`.
    The current behavior is to return a `null` if the row contains any `null`.
    Setting `ignore_nulls = TRUE` changes that.
  - All `row_count_*` args in reading/scanning functions are renamed
    `row_index_*`.
  - `$sort()` for `Series` gains an argument `nulls_last`.
  - `$str$extract()` and `$str$zfill()` now accept an `Expr` and parse
    strings as column names. Use `pl$lit()` to recover the old behavior.
  - `$cum_count()` now starts from 1 instead of 0.

### Other breaking changes

- The `simd` feature of the Rust library is removed in favor of
  the new `nightly` feature (#800).
  If you specified `simd` via the `LIBR_POLARS_FEATURES` environment variable
  during source installations, please use `nightly` instead;
  there is no change if you specified `full_features` because
  it now contains `nightly` instead of `simd`.
- The following functions were deprecated in 0.13.0 and are now removed (#783):
  - `$list$lengths()` -> `$list$len()`
  - `pl$from_arrow()` -> `as_polars_df()` or `as_polars_series()`
  - `pl$set_options()` and `pl$reset_options()` -> `polars_options()`
- `$is_between()` had several changes (#788):
  - arguments `start` and `end` are renamed `lower_bound` and `upper_bound`.
    Their behaviour doesn't change.
  - `include_bounds` is renamed `closed` and must be one of `"left"`,
    `"right"`, `"both"`, or `"none"`.
- `polars_info()` returns a slightly changed list.
  - `$threadpool_size`, which means the number of threads used by Polars,
    is changed to `$thread_pool_size` (#784)
  - `$version`, which indicates the version of this package,
    is changed to `$versions$r_package` (#791).
  - `$rust_polars`, which indicates the version of the dependent Rust Polars,
    is changed to `$versions$rust_crate` (#791).
- New behavior when creating a `DataFrame` with a single list-variable.
  `pl$DataFrame(x = list(1:2, 3:4))` used to create a `DataFrame` with two
  columns named "new_column" and "new_column_1", which was unexpected. It now
  produces a `DataFrame` with a single `list` variable. This also applies to
  list-column created in `$with_columns()` and `$select()` (#794).

### Deprecations

- `pl$threadpool_size()` is deprecated and will be removed in 0.15.0. Use
  `pl$thread_pool_size()` instead (#784).

### New features

- Implementation of the subnamespace `$arr` for expressions on `array`-type
  columns. An `array` column is similar to a `list` column, but is stricter as
  each sub-array must have the same number of elements (#790).

### Other improvements

- The `sql` feature is included in the default feature (#800).
  This means that functionality related to the `RPolarsSQLContext` class
  is now always included in the binary package.

## Polars R Package 0.13.1

### New features

- New method `$write_parquet()` for DataFrame (#758).
- S3 methods of `as.data.frame()` for `RPolarsDataFrame` and `RPolarsLazyFrame`
  accepts more arguments of `as_polars_df()` and `<DataFrame>$to_data_frame()` (#762).
- S3 methods of `arrow::as_arrow_table()` and `arrow::as_record_batch_reader()` for
  `RPolarsDataFrame` no longer need the `{nanoarrow}` package (#754).
- Some S3 methods for the `{nanoarrow}` package are added (#730).
  - `as_polars_df(<nanoarrow_array_stream>)`
  - `as_polars_series(<nanoarrow_array>)`
  - `as_polars_series(<nanoarrow_array_stream>)`

### Bug fixes

- `$sort()` no longer panicks when `descending = NULL` (#748).

### Other enhancements

- `downlit::autolink()` now recognize the reference pages of this package (#739).

## Polars R Package 0.13.0

### Breaking changes

- `<Expr>$where()` is removed. Use `<Expr>$filter()` instead (#718).
- Deprecated functions from 0.12.x are removed (#714).
  - `<Expr>$apply()` and `<Expr>$map()`, use `$map_elements()` and
    `$map_batches()` instead.
  - `pl$polars_info()`, use `polars_info()` instead.
- The environment variables used when building the library have been changed
  (#693). This only affects selecting the feature flag and selecting profiles
  during source installation.
  - `RPOLARS_PROFILE` is renamed to `LIBR_POLARS_PROFILE`
  - `RPOLARS_FULL_FEATURES` is removed and `LIBR_POLARS_FEATURES` is added.
    To select the `full_features`, set `LIBR_POLARS_FEATURES="full_features"`.
  - `RPOLARS_RUST_SOURCE`, which was used for development, has been removed.
    If you want to use library binaries located elsewhere, use `LIBR_POLARS_PATH`
    instead.
- Remove the `eager` argument of `<SQLContext>$execute()`.
  Use the `$collect()` method after `$execute()` or `as_polars_df` to get the
  result as a `DataFrame`. (#719)
- The argument `name_generator` of `$list$to_struct()` is renamed `fields`
  (#724).
- The S3 method `[` for the `$list` subnamespace is removed (#724).
- The option `polars.df_print` has been renamed `polars.df_knitr_print` (#726).

### Deprecations

- `$list$lengths()` is deprecated and will be removed in 0.14.0. Use
  `$list$len()` instead (#724).
- `pl$from_arrow()` is deprecated and will be removed in 0.14.0.
  Use `as_polars_df()` or `as_polars_series()` instead (#728).
- `pl$set_options()` and `pl$reset_options()` are deprecated and will be
  removed in 0.14.0. See `?polars_options` for details (#726).

### New features

- For compatibility with CRAN, the number of threads used by Polars is automatically set to 2
  if the environment variable `POLARS_MAX_THREADS` is not set (#720).
  To disable this behavior and have the maximum number of threads used automatically,
  one of the following ways can be used:
  - Build the Rust library with the `disable_limit_max_threads` feature.
  - Set the `polars.limit_max_threads` option to `FALSE` with the `options()` function
    before loading the package.
- New method `$rolling()` for `DataFrame` and `LazyFrame`. When this is
  applied, it creates an object of class `RPolarsRollingGroupBy` (#682, #694).
- New method `$group_by_dynamic()` for `DataFrame` and `LazyFrame`. When this
  is applied, it creates an object of class `RPolarsDynamicGroupBy` (#691).
- New method `$sink_ndjson()` for LazyFrame (#681).
- New function `pl$duration()` to create a duration by components (week, day,
  hour, etc.), and use them with date(time) variables (#692).
- New methods `$list$any()` and `$list$all()` (#709).
- New function `pl$from_epoch()` to convert a Unix timestamp to a date(time)
  variable (#708).
- New methods for the `list` subnamespace: `$set_union()`, `$set_intersection()`,
  `$set_difference()`, `$set_symmetric_difference()` (#712).
- New option `int64_conversion` to specify how Int64 columns (that don't have
  equivalent in base R) should be converted. This option can either be set
  globally with `pl$set_options()` or on a case-by-case basis, e.g with
  `$to_data_frame(int64_conversion =)` (#706).
- Several changes in `$join()` for `DataFrame` and `LazyFrame` (#716):
  - `<LazyFrame>$join()` now errors if `other` is not a `LazyFrame` and
    `<DataFrame>$join()` errors if `other` is not a `DataFrame`.
  - Some arguments have been reordered (e.g `how` now comes before `left_on`).
    This can lead to bugs if the user didn't use argument names.
  - Argument `how` now accepts `"outer_coalesce"` to coalesce the join keys
    automatically after joining.
  - New argument `validate` to perform some checks on join keys (e.g ensure
    that there is a one-to-one matching between join keys).
  - New argument `join_nulls` to consider `null` values as a valid key.
- `<DataFrame>$describe()` now works with all datatypes. It also gains an
  `interpolation` argument that is used for quantiles computation (#717).
- `as_polars_df()` and `as_polars_series()` for the `arrow` package classes have been
  rewritten and work better (#727).
- Options handling has been rewritten to match the standard option handling in
  R (#726):
  - Options are now passed via `options()`. The option names don't change but
    they must be prefixed with `"polars."`. For example, we can now pass
    `options(polars.strictly_immutable = FALSE)`.
  - Options can be accessed with `polars_options()`, which returns a named
    list (this is the replacement of `pl$options`).
  - Options can be reset with `polars_options_reset()` (this is the
    replacement of `pl$reset_options()`).
- New function `polars_envvars()` to print the list of environment variables
  related to polars (#735).

## Polars R Package 0.12.2

This is a small release including a few documentation improvements and internal updates.

## Polars R Package 0.12.1

This version includes a few additional features and
a large amount of documentation improvements.

### Deprecations

- `pl$polars_info()` is moved to `polars_info()`. `pl$polars_info()` is deprecated
  and will be removed in 0.13.0 (#662).

### Rust Polars update

- Rust Polars is updated to 0.36.2 (#659). Most of the changes from 0.35.x to 0.36.2
  were covered in R polars 0.12.0.
  The main change is that `pl$Utf8` is replaced by `pl$String`.
  `pl$Utf8` is an alias and will keep working, but `pl$String` is now preferred
  in the documentation and in new code.

### What's changed

- New methods `$str$reverse()`, `$str$contains_any()`, and `$str$replace_many()`
  (#641).
- New methods `$rle()` and `$rle_id()` (#648).
- New functions `is_polars_df()`, `is_polars_lf()`, `is_polars_series()` (#658).
- `$gather()` now accepts negative indexing (#659).

### Miscellaneous

- Remove the `Makefile` in favor of `Taskfile.yml`.
  Please use `task` instead of `make` as a task runner in the development (#654).

## Polars R Package 0.12.0

### BREAKING CHANGES DUE TO Rust Polars UPDATE

- Rust Polars is updated to 2023-12-25 unreleased version (#601, #622).
  This is the same version of Python Polars package 0.20.2, so please check
  the [upgrade guide](https://pola-rs.github.io/polars/releases/upgrade/0.20/) for details too.
  - `pl$scan_csv()` and `pl$read_csv()`'s `comment_char` argument is renamed `comment_prefix`.
  - `<DataFrame>$frame_equal()` and `<Series>$series_equal()` are renamed
    to `<DataFrame>$equals()` and `<Series>$equals()`.
  - `<Expr>$rolling_*` functions gained an argument `warn_if_unsorted`.
  - `<Expr>$str$json_extract()` is renamed to `<Expr>$str$json_decode()`.
  - Change default join behavior with regard to `null` values.
  - Preserve left and right join keys in outer joins.
  - `count` now ignores null values.
  - `NaN` values are now considered equal.
  - `$gather_every()` gained an argument `offset`.

### Breaking changes and deprecations

- `$apply()` on an Expr or a Series is renamed `$map_elements()`, and `$map()`
  is renamed `$map_batches()`. `$map()` and `$apply()` will be removed in 0.13.0 (#534).
- Removed `$days()`, `$hours()`, `$minutes()`, `$seconds()`, `$milliseconds()`,
  `$microseconds()`, `$nanoseconds()`. Those were deprecated in 0.11.0 (#550).
- `pl$concat_list()`: elements being strings are now interpreted as column names.
  Use `pl$lit` to concat with a string.
- `<RPolarsExpr>$lit_to_s()` is renamed to `<RPolarsExpr>$to_series()` (#582).
- `<RPolarsExpr>$lit_to_df()` is removed (#582).
- Change class names and function names associated with class names.
  - The class name of all objects created by polars (`DataFrame`, `LazyFrame`,
    `Expr`, `Series`, etc.) has changed. They now start with `RPolars`, for example
    `RPolarsDataFrame`. This will only break your code if you directly use those
    class names, such as in S3 methods (#554, #585).
  - Private methods have been unified so that they do not have the `RPolars` prefix (#584).

### What's changed

- The Extract function (`[`) for DataFrame can use columns not included in the
  result for filtering (#547).
- The Extract function (`[`) for LazyFrame can filter rows with Expressions (#547).
- `as_polars_df()` for `data.frame` has a new argument `rownames` for to convert
  the row.names attribute to a column.
  This option is inspired by the `tibble::as_tibble()` function (#561).
- `as_polars_df()` for `data.frame` has a new argument `make_names_unique` (#561).
- New methods `$str$to_date()`, `$str$to_time()`, `$str$to_datetime()` as
  alternatives to `$str$strptime()` (#558).
- The `dim()` function for DataFrame and LazyFrame correctly returns integer instead of
  double (#577).
- The conversion of R's `POSIXct` class to Polars datetime now works correctly with millisecond
  precision (#589).
- `<LazyFrame>$filter()`, `<DataFrame>$filter()`, and `pl$when()` now allow multiple conditions
  to be separated by commas, like `lf$filter(pl$col("foo") == 1, pl$col("bar") != 2)` (#598).
- New method `$replace()` for expressions (#601).
- Better error messages for trailing argument commas such as `pl$DataFrame()$select("a",)` (#607).
- New function `pl$threadpool_size()` to get the number of threads used by Polars (#620).
  Thread pool size is also included in the output of `pl$polars_info()`.

## Polars R Package 0.11.0

### BREAKING CHANGES DUE TO Rust Polars UPDATE

- Rust Polars is updated to 0.35.0 (2023-11-17) (#515)
  - changes in `$write_csv()` and `sink_csv()`: `has_header` is renamed
    `include_header` and there's a new argument `include_bom`.
  - `pl$cov()` gains a `ddof` argument.
  - `$cumsum()`, `$cumprod()`, `$cummin()`, `$cummax()`, `$cumcount()` are
    renamed `$cum_sum()`, `$cum_prod()`, `$cum_min()`, `$cum_max()`,
    `$cum_count()`.
  - `take()` and `take_every()` are renamed `$gather()` and `gather_every()`.
  - `$shift()` and `$shift_and_fill()` now accept Expr as input.
  - when `reverse = TRUE`, `$arg_sort()` now places null values in the first
    positions.
  - Removed argument `ambiguous` in `$dt$truncate()` and `$dt$round()`.
  - `$str$concat()` gains an argument `ignore_nulls`.

### Breaking changes and deprecations

- The rowwise computation when several columns are passed to `pl$min()`, `pl$max()`,
  and `pl$sum()` is deprecated and will be removed in 0.12.0. Passing several
  columns to these functions will now compute the min/max/sum in each column
  separately. Use `pl$min_horizontal()` `pl$max_horizontal()`, and
  `pl$sum_horizontal()` instead for rowwise computation (#508).
- `$is_not()` is deprecated and will be removed in 0.12.0. Use `$not()` instead
  (#511, #531).
- `$is_first()` is deprecated and will be removed in 0.12.0. Use `$is_first_distinct()`
  instead (#531).
- In `pl$concat()`, the argument `to_supertypes` is removed. Use the suffix
  `"_relaxed"` in the `how` argument to cast columns to their shared supertypes
  (#523).
- All duration methods (`days()`, `hours()`, `minutes()`, `seconds()`,
  `milliseconds()`, `microseconds()`, `nanoseconds()`) are renamed, for example
  from `$dt$days()` to `$dt$total_days()`. The old usage is deprecated and will
  be removed in 0.12.0 (#530).
- DataFrame methods `$as_data_frame()` is removed in favor of `$to_data_frame()` (#533).
- GroupBy methods `$as_data_frame()` and `$to_data_frame()` which were used to
  convert GroupBy objects to R data frames are removed.
  Use `$ungroup()` method and the `as.data.frame()` function instead (#533).

### What's changed

- Fix the installation issue on Ubuntu 20.04 (#528, thanks @brownag).
- New methods `$write_json()` and `$write_ndjson()` for DataFrame (#502).
- Removed argument `name` in `pl$date_range()`, which was deprecated for a while
  (#503).
- New private method `.pr$DataFrame$drop_all_in_place(df)` to drop `DataFrame`
  in-place, to release memory without invoking gc(). However, if there are other
  strong references to any of the underlying Series or arrow arrays, that memory
  will specifically not be released. This method is aimed for r-polars extensions,
  and will be kept stable as much as possible (#504).
- New functions `pl$min_horizontal()`, `pl$max_horizontal()`, `pl$sum_horizontal()`,
  `pl$all_horizontal()`, `pl$any_horizontal()` (#508).
- New generic functions `as_polars_df()` and `as_polars_lf()` to create polars
  DataFrames and LazyFrames (#519).
- New method `$ungroup()` for `GroupBy` and `LazyGroupBy` (#522).
- New method `$rolling()` to apply an Expr over a rolling window based on
  date/datetime/numeric indices (#470).
- New methods `$name$to_lowercase()` and `$name$to_uppercase()` to transform
  variable names (#529).
- New method `$is_last_distinct()` (#531).
- New methods of the Expressions class, `$floor_div()`, `$mod()`, `$eq_missing()`
  and `$neq_missing()`. The base R operators `%/%` and `%%` for Expressions are
  now translated to `$floor_div()` and `$mod()` (#523).
  - Note that `$mod()` of Polars is different from the R operator `%%`, which is
    not guaranteed `x == (x %% y) + y * (x %/% y)`.
    Please check the upstream issue [pola-rs/polars#10570](https://github.com/pola-rs/polars/issues/10570).
- The extract function (`[`) for polars objects now behave more like for base R objects (#543).

## Polars R Package 0.10.1

### What's changed

- The argument `quote_style` in `$write_csv()` and `$sink_csv()` can now take
  the value `"never"` (#483).
- `pl$DataFrame()` now errors if the variables specified in `schema` do not exist
  in the data (#486).
- S3 methods for base R functions are well documented (#494).
- A bug that failing `pl$SQLContext()$register()` without load the package was fixed (#496).

## Polars R Package 0.10.0

### BREAKING CHANGES DUE TO Rust Polars UPDATE

- Rust Polars is updated to 2023-10-25 unreleased version (#442)
  - Minimum supported Rust version (MSRV) is now 1.73.
  - New subnamespace `"name"` that contains methods `$prefix()`, `$suffix()`
    `keep()` (renamed from `keep_name()`) and `map()` (renamed from `map_alias()`).
  - `$dt$round()` gains an argument `ambiguous`.
  - The following methods now accept an `Expr` as input: `$top_k()`, `$bottom_k()`,
    `$list$join()`, `$str$strip_chars()`, `$str$strip_chars_start()`,
    `$str$strip_chars_end()`, `$str$split_exact()`.
  - The following methods were renamed:
    - `$str$n_chars()` -> `$str$len_chars()`
    - `$str$lengths()` -> `$str$len_bytes()`
    - `$str$ljust()` -> `$str$pad_end()`
    - `$str$rjust()` -> `$str$pad_start()`
  - `$concat()` with `how = "diagonal"` now accepts an argument `to_supertypes`
    to automatically convert concatenated columns to the same type.
  - `pl$enable_string_cache()` doesn't take any argument anymore. The string cache
    can now be disabled with `pl$disable_string_cache()`.
  - `$scan_parquet()` gains an argument `hive_partitioning`.
  - `$meta$tree_format()` has a better formatted output.

### Breaking changes

- `$scan_csv()` and `$read_csv()` now match more closely the Python-Polars API (#455):
  - `sep` is renamed `separator`, `overwrite_dtypes` is renamed `dtypes`,
    `parse_dates` is renamed `try_parse_dates`.
  - new arguments `rechunk`, `eol_char`, `raise_if_empty`, `truncate_ragged_lines`
  - `path` can now be a vector of characters indicating several paths to CSV files.
    This only works if all CSV files have the same schema.

### What's changed

- New class `RPolarsSQLContext` and its methods to perform SQL queries on DataFrame-
  like objects. To use this feature, needs to build Rust library with full features
  (#457).
- New methods `$peak_min()` and `$peak_max()` to find local minima and maxima in
  an Expr (#462).
- New methods `$read_ndjson()` and `$scan_ndjson()` (#471).
- New method `$with_context()` for `LazyFrame` to have access to columns from
  other Data/LazyFrames during the computation (#475).

## Polars R Package 0.9.0

### BREAKING CHANGES DUE TO Rust Polars UPDATE

- Rust Polars is updated to 0.33.2 (#417)
  - In all date-time related methods, the argument `use_earliest` is replaced by `ambiguous`.
  - In `$sample()` and `$shuffle()`, the argument `fixed_seed` is removed.
  - In `$value_counts()`, the arguments `multithreaded` and `sort`
    (sometimes called `sorted`) have been swapped and renamed `sort` and `parallel`.
  - `$str$count_match()` gains a `literal` argument.
  - `$arg_min()` doesn't consider `NA` as the minimum anymore (this was already the behavior of `$min()`).
  - Using `$is_in()` with `NA` on both sides now returns `NA` and not `TRUE` anymore.
  - Argument `pattern` of `$str$count_matches()` can now use expressions.
  - Needs Rust toolchain `nightly-2023-08-26` for to build with full features.
- Rename R functions to match Rust Polars
  - `$str$count_match()` -> `$str$count_matches()` (#417)
  - `$str$strip()` -> `$str$strip_chars()` (#417)
  - `$str$lstrip()` -> `$str$strip_chars_start()` (#417)
  - `$str$rstrip()` -> `$str$strip_chars_end()` (#417)
  - `$groupby()` is renamed `$group_by()`. (#427)

### Breaking changes

- Remove some deprecated methods.
  - Method `$with_column()` has been removed (it was deprecated since 0.8.0).
    Use `$with_columns()` instead (#402).
  - Subnamespace `$arr` has been removed (it was deprecated since 0.8.1).
    Use `$list` instead (#402).
- Setting and getting polars options is now made with `pl$options`,
  `pl$set_options()` and `pl$reset_options()` (#384).

### What's changed

- Bump supported R version to 4.2 or later (#435).
- `pl$concat()` now also supports `Series`, `Expr` and `LazyFrame` (#407).
- New method `$unnest()` for `LazyFrame` (#397).
- New method `$sample()` for `DataFrame` (#399).
- New method `$meta$tree_format()` to display an `Expr` as a tree (#401).
- New argument `schema` in `pl$DataFrame()` and `pl$LazyFrame()` to override the
  automatic type detection (#385).
- Fix bug when calling R from polars via e.g. `$map()` where query would not
  complete in one edge case (#409).
- New method `$cat$get_categories()` to list unique values of categorical
  variables (#412).
- New methods `$fold()` and `$reduce()` to apply an R function rowwise (#403).
- New function `pl$raw_list` and class `rpolars_raw_list` a list of R Raw's, where missing is
  encoded as `NULL` to aid conversion to polars binary Series. Support back and forth conversion
  from polars binary literal and Series to R raw (#417).
- New method `$write_csv()` for `DataFrame` (#414).
- New method `$sink_csv()` for `LazyFrame` (#432).
- New method `$dt$time()` to extract the time from a `datetime` variable (#428).
- Method `$profile()` gains optimization arguments and plot-related arguments (#429).
- New method `pl$read_parquet()` that is a shortcut for `pl$scan_parquet()$collect()` (#434).
- Rename `$str$str_explode()` to `$str$explode()` (#436).
- New method `$transpose()` for `DataFrame` (#440).
- New argument `eager` of `LazyFrame$set_optimization_toggle()` (#439).
- `{polars}` can now be installed with "R source package with Rust library binary",
  by a mechanism copied from [the prqlr package](https://CRAN.R-project.org/package=prqlr).

  ```r
  Sys.setenv(NOT_CRAN = "true")
  install.packages("polars", repos = "https://rpolars.r-universe.dev")
  ```

  The URL and SHA256 hash of the available binaries are recorded in `tools/lib-sums.tsv`.
  (#435, #448, #450, #451)

## Polars R Package 0.8.1

### What's changed

- New string method `to_titlecase()` (#371).
- Although stated in news for PR (#334) `strip = true` was not actually set for the
  "release-optimized" compilation profile. Now it is, but the binary sizes seems unchanged (#377).
- New vignette on best practices to improve `polars` performance (#188).
- Subnamespace name "arr" as in `<Expr>$arr$` & `<Series>$arr$` is deprecated
  in favor of "list". The subnamespace "arr" will be removed in polars 0.9.0 (#375).

## Polars R Package 0.8.0

### BREAKING CHANGES DUE TO Rust Polars UPDATE

Rust Polars was updated to 0.32.0, which comes with many breaking changes and new
features. Unrelated breaking changes and new features are put in separate sections
(#334):

- update of rust toolchain: nightly bumped to nightly-2023-07-27 and MSRV is
  now >=1.70.
- param `common_subplan_elimination = TRUE` in `<LazyFrame>` methods `$collect()`,
  `$sink_ipc()` and `$sink_parquet()` is renamed and split into
  `comm_subplan_elim = TRUE` and `comm_subexpr_elim = TRUE`.
- Series_is_sorted: nulls_last argument is dropped.
- `when-then-otherwise` classes are renamed to `When`, `Then`, `ChainedWhen`
  and `ChainedThen`. The syntactically illegal methods have been removed, e.g.
  chaining `$when()` twice.
- Github release + R-universe is compiled with `profile=release-optimized`,
  which now includes `strip=false`, `lto=fat` & `codegen-units=1`. This should
  make the binary a bit smaller and faster. See also FULL_FEATURES=`true` env
  flag to enable simd with nightly rust. For development or faster compilation,
  use instead `profile=release`.
- `fmt` arg is renamed `format` in `pl$Ptimes` and `<Expr>$str$strptime`.
- `<Expr>$approx_unique()` changed name to `<Expr>$approx_n_unique()`.
- `<Expr>$str$json_extract` arg `pat` changed to `dtype` and has a new argument
  `infer_schema_length = 100`.
- Some arguments in `pl$date_range()` have changed: `low` -> `start`,
  `high` -> `end`, `lazy = TRUE` -> `eager = FALSE`. Args `time_zone` and `time_unit`
  can no longer be used to implicitly cast time types. These two args can only
  be used to annotate a naive time unit. Mixing `time_zone` and `time_unit` for
  `start` and `end` is not allowed anymore.
- `<Expr>$is_in()` operation no longer supported for dtype `null`.
- Various subtle changes:
  - `(pl$lit(NA_real_) == pl$lit(NA_real_))$lit_to_s()` renders now to `null`
    not `true`.
  - `pl$lit(NA_real_)$is_in(pl$lit(NULL))$lit_to_s()` renders now to `false`
    and before `true`
  - `pl$lit(numeric(0))$sum()$lit_to_s()` now yields `0f64` and not `null`.
- `<Expr>$all()` and `<Expr>$any()` have a new arg `drop_nulls = TRUE`.
- `<Expr>$sample()` and `<Expr>$shuffle()` have a new arg `fix_seed`.
- `<DataFrame>$sort()` and `<LazyFrame>$sort()` have a new arg
  `maintain_order = FALSE`.

### OTHER BREAKING CHANGES

- `$rpow()` is removed. It should never have been translated. Use `^` and `$pow()`
  instead (#346).
- `<LazyFrame>$collect_background()` renamed `<LazyFrame>$collect_in_background()`
  and reworked. Likewise `PolarsBackgroundHandle` reworked and renamed to
  `RThreadHandle` (#311).
- `pl$scan_arrow_ipc` is now called `pl$scan_ipc` (#343).

### Other changes

- Stream query to file with `pl$sink_ipc()` and `pl$sink_parquet()` (#343)
- New method `$explode()` for `DataFrame` and `LazyFrame` (#314).
- New method `$clone()` for `LazyFrame` (#347).
- New method `$fetch()` for `LazyFrame` (#319).
- New methods `$optimization_toggle()` and `$profile()` for `LazyFrame` (#323).
- `$with_column()` is now deprecated (following upstream `polars`). It will be
  removed in 0.9.0. It should be replaced with `$with_columns()` (#313).
- New lazy function translated: `concat_str()` to concatenate several columns
  into one (#349).
- New stat functions `pl$cov()`, `pl$rolling_cov()` `pl$corr()`, `pl$rolling_corr()` (#351).
- Add functions `pl$set_global_rpool_cap()`, `pl$get_global_rpool_cap()`, class `RThreadHandle` and
  `in_background = FALSE` param to `<Expr>$map()` and `$apply()`. It is now possible to run R code
  with `<LazyFrame>collect_in_background()` and/or let polars parallize R code in an R processes
  pool. See `RThreadHandle-class` in reference docs for more info. (#311)
- Internal IPC/shared-mem channel to serialize and send R objects / polars DataFrame across
  R processes. (#311)
- Compile environment flag RPOLARS_ALL_FEATURES changes name to RPOLARS_FULL_FEATURES. If 'true'
  will trigger something like `Cargo build --features "full_features"` which is not exactly the same
  as `Cargo build --all-features`. Some dev features are not included in "full_features" (#311).
- Fix bug to allow using polars without library(polars) (#355).
- New methods `<LazyFrame>$optimization_toggle()` + `$profile()` and enable Rust Polars feature
  CSE: "Activate common subplan elimination optimization" (#323)
- Named expression e.g. `pl$select(newname = pl$lit(2))` are no longer experimental
  and allowed as default (#357).
- Added methods `pl$enable_string_cache()`, `pl$with_string_cache()` and `pl$using_string_cache()`
  for joining/comparing Categorical series/columns (#361).
- Added an S3 generic `as_polars_series()` where users or developers of extensions
  can define a custom way to convert their format to Polars format. This generic
  must return a Polars series. See #368 for an example (#369).
- Private API Support for Arrow Stream import/export of DataFrame between two R packages that uses
  Rust Polars. [See R package example here](https://github.com/rpolars/extendrpolarsexamples)
  (#326).

## Polars R Package 0.7.0

### BREAKING CHANGES

- Replace the argument `reverse` by `descending` in all sorting functions. This
  is for consistency with the upstream Polars (#291, #293).
- Bump Rust Polars from 2023-04-20 unreleased version to version 0.30.0 released in 2023-05-30 (#289).
  - Rename `concat_lst` to `concat_list`.
  - Rename `$str$explode` to `$str$str_explode`.
  - Remove `tz_aware` and `utc` arguments from `str_parse`.
  - in `$date_range`'s the `lazy` argument is now `TRUE` by default.
- The functions to read CSV have been renamed `scan_csv` and `read_csv` for
  consistency with the upstream Polars. `scan_xxx` and `read_xxx` functions are now accessed via `pl`,
  e.g. `pl$scan_csv()` (#305).

### What's changed

- New method `$rename()` for `LazyFrame` and `DataFrame` (#239)
- `<DataFrame>$unique()` and `<LazyFrame>$unique()` gain a `maintain_order` argument (#238).
- New `pl$LazyFrame()` to quickly create a `LazyFrame`, mostly in examples or
  for demonstration purposes (#240).
- Polars is internally moving away from string errors to a new error-type called `RPolarsErr` both on rust- and R-side. Final error messages should look very similar (#233).
- `$columns()`, `$schema()`, `$dtypes()` for `LazyFrame` implemented (#250).
- Improvements to internal `RPolarsErr`. Also `RPolarsErr` will now print each context of the error on a separate line (#250).
- Fix memory leak on error bug. Fix printing of `%` bug. Prepare for renaming of polars classes (#252).
- Add helpful reference landing page at `polars.github.io/reference_home` (#223, #264).
- Supports Rust 1.65 (#262, #280)
  - Rust Polars' `simd` feature is now disabled by default. To enable it, set the environment variable
    `RPOLARS_ALL_FEATURES` to `true` when build r-polars (#262).
  - `opt-level` of `argminmax` is now set to `1` in the `release` profile to support Rust < 1.66.
    The profile can be changed by setting the environment variable `RPOLARS_PROFILE` (when set to `release-optimized`,
    `opt-level` of `argminmax` is set to `3`).
- A new function `pl$polars_info()` will tell which features enabled (#271, #285, #305).
- `select()` now accepts lists of expressions. For example, `<DataFrame>$select(l_expr)`
  works with `l_expr = list(pl$col("a"))` (#265).
- LazyFrame gets some new S3 methods: `[`, `dim()`, `dimnames()`, `length()`, `names()` (#301)
- `<DataFrame>$glimpse()` is a fast `str()`-like view of a `DataFrame` (#277).
- `$over()` now accepts a vector of column names (#287).
- New method `<DataFrame>$describe()` (#268).
- Cross joining is now possible with `how = "cross"` in `$join()` (#310).
- Add license info of all rust crates to `LICENSE.note` (#309).
- With CRAN 0.7.0 release candidate (#308).
  - New author accredited, SHIMA Tatsuya (@eitsupi).
  - DESCRIPTION revised.

## Polars R Package 0.6.1

### What's changed

- use `pl$set_polars_options(debug_polars = TRUE)` to profile/debug method-calls of a polars query (#193)
- add `<DataFrame>$melt(), <DataFrame>$pivot() + <LazyFrame>$melt()` methods (#232)
- lazy functions translated: `pl$implode`, `pl$explode`, `pl$unique`, `pl$approx_unique`, `pl$head`, `pl$tail` (#196)
- `pl$list` is deprecated, use `pl$implode` instead. (#196)
- Docs improvements. (#210, #213)
- Update nix flake. (#227)

## Polars R Package 0.6.0

### BREAKING CHANGES

- Bump Rust Polars from 2023-02-17 unreleased version to 2023-04-20 unreleased version. (#183)
  - `top_k`'s `reverse` option is removed. Use the new `bottom_k` method instead.
  - The name of the `fmt` argument of some methods (e.g. `parse_date`) has been changed to `format`.

### What's changed

- `DataFrame` objects can be subsetted using brackets like standard R data frames: `pl$DataFrame(mtcars)[2:4, c("mpg", "hp")]` (#140 @vincentarelbundock)
- An experimental `knit_print()` method has been added to DataFrame that outputs HTML tables
  (similar to py-polars' HTML output) (#125 @eitsupi)
- `Series` gains new methods: `$mean`, `$median`, `$std`, `$var` (#170 @vincentarelbundock)
- A new option `use_earliest` of `replace_time_zone`. (#183)
- A new option `strict` of `parse_int`. (#183)
- Perform joins on nearest keys with method `join_asof`. (#172)

## Polars R Package v0.5.0

### BREAKING CHANGE

- The package name was changed from `rpolars` to `polars`. (#84)

### What's changed

- Several new methods for DataFrame, LazyFrame & GroupBy translated (#103, #105 @vincentarelbundock)
- Doc fixes (#102, #109 @etiennebacher)
- Experimental opt-in auto completion (#96 @sorhawell)
- Base R functions work on DataFrame and LazyFrame objects via S3 methods: as.data.frame, as.matrix, dim, head, length, max, mean, median, min, na.omit, names, sum, tail, unique, ncol, nrow (#107 @vincentarelbundock).

### New Contributors

- @etiennebacher made their first contribution in #102
- @vincentarelbundock made their first contribution in #103

Release date: 2023-04-16. Full changelog:
[v0.4.6...v0.5.0](https://github.com/pola-rs/r-polars/compare/v0.4.7...v0.5.0)
