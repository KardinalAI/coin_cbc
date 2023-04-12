extern crate pkg_config;
use std::env;

fn main() {
    let _ = pkg_config::probe_library("cbc");
    if cfg!(windows) {
        match env::var("CBC_ROOT") {
            Ok(cbc_root) => {
                println!("cargo:rustc-link-search={cbc_root}\\lib");
                println!(r"cargo:rustc-link-lib=static=Cbc");
                println!(r"cargo:rustc-link-lib=static=CbcSolver");
                println!(r"cargo:rustc-link-lib=static=Cgl");
                println!(r"cargo:rustc-link-lib=static=Clp");
                println!(r"cargo:rustc-link-lib=static=CoinUtils");
                println!(r"cargo:rustc-link-lib=static=Osi");
                println!(r"cargo:rustc-link-lib=static=OsiClp");
                println!(r"cargo:rustc-link-lib=static=zlib");
                ()
            }
            _ => {
                println!(r"cargo:warning=CBC_ROOT environment variable not found.");
            }
        }
    }
}
