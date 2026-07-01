= ivar

**ivar** is a Rust implementation of [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic).

*Current state*: This project is in active development. Most functionality is implemented, but some implementations are naive and require improvments, and more testing and documentation is required. Not on `crates.io` yet.

## Goals

[ ] Implement [IEEE Std 1788-2015](https://doi.org/10.1109/IEEESTD.2015.7140721).
[x] `#[no_std]`.
[x] Convenient API (subjective).
[x] Have minimal dependencies. Current implementation has only one required dependency `crlibm` for exponential and trigonometric functions with correct rounding.
[x] As an optional but default feature, implement common traits from `num_traits` for intervals and decorated intervals.

## Inspiration

This project was inspired by [inari](https://github.com/unageek/inari) crate that also implements interval arithmetic. Some differences between `inari` and `ivar` are:
- `ivar` does not use multi-precision libraries `gmp` or `mpfr` like `inari` does, instead, only standard `f64` operations are used to implement basic interval operations, and `crlibm` bindings are used for trancendental functions.
- `ivar` does not use `mxcsr` for changing FPU rounding mode to implement arithmetical operations with rounding up and down. Turns out that even for addition this approach is 6-7 times slower than utilizing `2sum` algorithm. Also setting `mxcsr` requires inline assembly, and I am not sure if this approach is portable.
- `ivar` does not require configuration with `.cargo/config.toml` setup
- `ivar` is `no-std`,
- `ivar` implements reverse elementary functions from IEEE Std 1788-2015,
- `ivar` implements `num_traits`, that allow intervals to be used with other crates, like `num_complex` for complex box arithmetic.


## References

- IEEE Std 1788-2015 - IEEE Standard for Interval Arithmetic. [DOI: 10.1109/IEEESTD.2015.7140721](https://doi.org/10.1109/IEEESTD.2015.7140721)
- IEEE Std 1788.1-2017 - IEEE Standard for Interval Arithmetic (Simplified). [DOI: 10.1109/IEEESTD.2018.8277144](https://doi.org/10.1109/IEEESTD.2018.8277144)
-  Jean-Michel Muller et. al., Handbook of Floating-Point Arithmetic, 2018 [DOI: 10.1007/978-3-319-76526-6](https://doi.org/10.1007/978-3-319-76526-6)

