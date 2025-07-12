
<!-- README.md is generated from README.Rmd. Please edit that file -->

# polars0

<!-- badges: start -->

[![Lifecycle:
superseded](https://img.shields.io/badge/lifecycle-superseded-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#superseded)
[![R-multiverse
status](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fcommunity.r-multiverse.org%2Fapi%2Fpackages%2Fpolars0&query=%24.Version&label=r-multiverse)](https://community.r-multiverse.org/polars0)
[![polars0 status
badge](https://rpolars.r-universe.dev/polars0/badges/version)](https://rpolars.r-universe.dev/polars0)
<!-- badges: end -->

This package is a continuation of the [polars R
package](https://github.com/pola-rs/r-polars) version 0, intended as a
mitigation for the too many breaking changes from polars R package
version 0 to 1.

There are no plans for active feature additions or bug fixes in this
package, so it is recommended to migrate to the new version of the
polars R package if possible.

We can trace the history of this package back to the original polars R
package, with the following steps:

``` sh
git clone https://github.com/rpolars/r-polars0.git
pushd r-polars0
git remote add r-polars https://github.com/pola-rs/r-polars.git
git fetch r-polars v0
git replace 03f3f689df98788455d233806ab9a5e984bc35f2 a37bc3422d1a0d334b068cdf137b5ee77e210163
```

## Install

This package can be installed from the R-universe rpolars repository.

``` r
Sys.setenv(NOT_CRAN = "true")
install.packages("polars0", repos = "https://rpolars.r-universe.dev")
```
