
<!-- README.md is generated from README.Rmd. Please edit that file -->

# polars0

<!-- badges: start -->

[![R-universe status
badge](https://rpolars.r-universe.dev/badges/polars0)](https://rpolars.r-universe.dev)
<!-- badges: end -->

This package is a continuation of the [Polars R
package](https://github.com/pola-rs/r-polars) version 0, intended as a
mitigation for the too many breaking changes from Polars R package
version 0 to 1.

There are no plans for active feature additions or bug fixes in this
package, so it is recommended to migrate to the new version of the
Polars R package if possible.

We can trace the history of this package back to the original Polars R
package, with the following steps:

``` sh
git clone https://github.com/rpolars/r-polars0.git
pushd r-polars0
git remote add r-polars https://github.com/pola-rs/r-polars.git
git fetch r-polars v0
git replace 03f3f689df98788455d233806ab9a5e984bc35f2 a37bc3422d1a0d334b068cdf137b5ee77e210163
```

## Install

``` r
Sys.setenv(NOT_CRAN = "true")
install.packages("polars0", repos = "https://rpolars.r-universe.dev")
```
