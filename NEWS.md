# NEWS

## polars0 1.0.0

Initial release after renaming from polars to polars0.

Just a package name change from polars to polars0 from polars package version 0.22.4.

Replace `polars` with `polars0` in your code as follows:

```diff
- library(polars)
+ library(polars0)
```

```diff
- polars::as_polars_df(mtcars)
+ polars0::as_polars_df(mtcars)
```
