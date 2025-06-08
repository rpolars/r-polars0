# NEWS

## polars0 1.0.0

Initial release after renaming from polars to polars0.
This package is intended to facilitate the transition from Polars R package version 0 to 1,
and active maintenance is not planned.

Just a package name change from polars to polars0 of the package version 0.22.4.

To use this package instead of the polars package,
please replace `polars` with `polars0` in your code as like:

```diff
- library(polars)
+ library(polars0)
```

```diff
- polars::as_polars_df(mtcars)
+ polars0::as_polars_df(mtcars)
```
