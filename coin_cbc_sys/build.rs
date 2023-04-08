extern crate pkg_config;
use std::env;

fn main() {
    let _ = pkg_config::probe_library("cbc");
    if cfg!(windows) {
        match env::var("CBC_ROOT") {
            Ok(cbc_root) => {
                println!("cargo:rustc-link-search={cbc_root}\\lib");
                println!(r"cargo:rustc-link-lib=static=libCbcSolver");
                println!(r"cargo:rustc-link-lib=static=libCbc");
                println!(r"cargo:rustc-link-lib=static=libCgl");
                println!(r"cargo:rustc-link-lib=static=libCoinUtils");
                println!(r"cargo:rustc-link-lib=static=libClp");
                println!(r"cargo:rustc-link-lib=static=libOsi");
                println!(r"cargo:rustc-link-lib=static=libOsiClp");
                ()
            }
            _ => {
                println!(r"cargo:warning=CBC_ROOT environment variable not found.");
            }
        }
    }
}
