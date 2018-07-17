# DivRem

[Documentation][docs-rs]

Rust library providing division and modulus variants not available in the standard library:

* Floored division and remainder
* Ceiled division and remainder
* Euclidian division and remaider

For every definition, we provide a `Div`, a `Rem` and a `DivRem` variant.

A `DivRem` variant of the truncated division is also provided for convenience since it does not exist in the standard library.

This crate is `no_std`.

[docs-rs]: https://docs.rs/divrem
