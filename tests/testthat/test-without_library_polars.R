test_that("without library(polars0)", {
  # calling sort("mpg") triggers rust to call pl$lit() which will be available even though
  # polars0 is not added to serach with search() library(polars0)
  skip_if_not_installed("callr")
  # positive test:
  # Will work because robj_to! now calls polars0::pl$lit and polars0::pl$col
  expect_identical(
    callr::r(\() {
      polars0::as_polars_df(mtcars)$sort("mpg")$to_list()
    }),
    polars0::as_polars_df(mtcars)$sort("mpg")$to_list()
  )

  # Negative control:
  # This will fail because test_wrong_call_pl_lit just uses pl$col and pl$lit
  expect_false(
    callr::r(\() polars0:::test_wrong_call_pl_lit(42) |> polars0:::is_ok())
  )

  # Positive-Negative control
  # This works because library(polars0) puts polars0 in search()
  expect_true(polars0:::test_wrong_call_pl_lit(42) |> polars0:::is_ok())
})




test_that("scan read parquet from other process", {
  skip_if_not_installed("callr")

  tmpf = tempfile()
  on.exit(unlink(tmpf))
  lf_exp = polars0::as_polars_lf(mtcars)
  lf_exp$sink_parquet(tmpf, compression = "snappy")
  df_exp = lf_exp$collect()$to_data_frame()

  # simple scan
  expect_identical(
    callr::r(\(tmpf) polars0::pl$scan_parquet(tmpf)$collect()$to_data_frame(), args = list(tmpf = tmpf)),
    df_exp
  )

  # simple read
  expect_identical(
    callr::r(\(tmpf) polars0::pl$read_parquet(tmpf)$to_data_frame(), args = list(tmpf = tmpf)),
    df_exp
  )
})
