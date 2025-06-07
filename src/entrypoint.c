// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_polars0_extendr(void *dll);

void R_init_polars0(void *dll) {
    R_init_polars0_extendr(dll);  //first or last
}
