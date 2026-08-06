extern crate pkg_config;
use std::env;

fn main() {
    match env::var("CARGO_CFG_TARGET_FAMILY")
        .as_ref()
        .map(String::as_str)
    {
        Ok("unix") => {
            let _ = pkg_config::probe_library("cbc");
        }
        Ok("windows") if env::var("CBC_ROOT").is_ok() => {
            let cbc_root = env::var("CBC_ROOT").unwrap();
            println!("cargo:rustc-link-search={cbc_root}/lib");
            println!(r"cargo:rustc-link-lib=static=libCbc");
            println!(r"cargo:rustc-link-lib=static=libCbcSolver");
            println!(r"cargo:rustc-link-lib=static=libCgl");
            println!(r"cargo:rustc-link-lib=static=libClp");
            println!(r"cargo:rustc-link-lib=static=libCoinUtils");
            println!(r"cargo:rustc-link-lib=static=libOsi");
            println!(r"cargo:rustc-link-lib=static=libOsiClp");
        }
        Ok("windows") if env::var("CBC_ROOT").is_err() => {
            println!(r"cargo:warning=CBC_ROOT environment variable not found.")
        }
        _ => println!(r"cargo:warning=Unsupported target family."),
    }
}
