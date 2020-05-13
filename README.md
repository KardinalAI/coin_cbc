# Coin CBC Rust bindings

Rust bindings to the CoinOR CBC MILP Solveur using the C API.

Tested on Debian 10, AMD64, coinor-libcbc3 2.9.9+repack1-1.

## `coin_cbc_sys`

This crate exposes raw bindings to the C functions.

## `coin_cbc`

This crate expose safe rust bindings using
`coin_cbc_sys`. `coin_cbc::raw::Model` exposes direct translation of
the C function with assert to guaranty safe use. `coin_cbc::Model`
exposes a more user friendly, rustic and efficient API: it was used
successfully to solve MILP with 250,000 binary variables with
unnoticeable overhead.

## Examples

See the [examples directory](examples/).

## License

This project is distributed under the [MIT License](LICENSE) by
[Kardinal](https://kardinal.ai).
