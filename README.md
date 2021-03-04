# Coin CBC Rust bindings

Rust bindings to the CoinOR CBC MILP Solver using the C API.

Tested on Debian 10, AMD64, coinor-libcbc3 2.9.9+repack1-1.
For more details on installing the `libCbc` dependencies, [see below](#prerequisites-installing-cbc-library-files).

## ⚠️ Important warning

By default, the cbc library is not thread safe, and this crate does not prevent manipulating multiple models in parallel.
This means that by default, this crate **violates rust memory safety rules**.
This will expose you to potentially exploitable memory corruption issues.
In particular:
 - If you are using this library inside a web server, you may be exposing yourself to remote code execution.
 - By default, cargo runs tests in parallel. If you are using this library in your tests, you are exposing yourself to non-deterministic test failures.

For more information and potential workarounds see [issue 9](https://github.com/KardinalAI/coin_cbc/issues/9).

## `coin_cbc_sys`

This crate exposes raw bindings to the C functions.

## `coin_cbc`

This crate exposes safe rust bindings using `coin_cbc_sys`.
`coin_cbc::raw::Model` exposes direct translation of the C function with assert to guaranty safe use.
`coin_cbc::Model` exposes a more user friendly, rustic and efficient API: it was used successfully to solve MILP with 250,000 binary variables with unnoticeable overhead.

## Examples

See the [examples directory](examples/).

## Prerequisites: installing `Cbc` library files

The library files of the [`COIN-OR` Solver `Cbc`](https://github.com/coin-or/Cbc) need to present on your system when compiling a project that depends on `coin_cbc`.
On a Debian system with a user with admin rights, this is easily achieved with:
```
sudo apt install coinor-libcbc-dev
```

For other systems, without admin rights or if you need a newer version of `Cbc` (e.g. with bug fixes), you can install `Cbc` through `coinbrew`:
https://coin-or.github.io/user_introduction#building-from-source

You will then have to either:
1. register the resulting library files with your system, or 
2. provide `cargo` with the location of that library.
For the first option, `coinbrew` provides a command suggestion after successful compilation.
The second option can e.g. be done via:
```
RUSTFLAGS='-L /path/to/your/cbc/install/lib' cargo test
```

## License

This project is distributed under the [MIT License](LICENSE) by
[Kardinal](https://kardinal.ai).
