// extern crate pkg_config;
use std::fmt::Display;
use std::path;
fn main() {
    // if std::env::var("DOCS_RS").is_err() {
    //     // do not probe for libsoxr when compiling at docs.rs
    //     if let Err(e) = pkg_config::probe_library("soxr") {
    //         match e {
    //         pkg_config::Error::Failure { .. } => panic! (
    //             "Pkg-config failed - usually this is because libsoxr development headers are not installed.\n\n\
    //             For Mac users using brew: brew install libsoxr\n\n\
    //             For Debian/Ubuntu users:\n# apt-get install libsoxr0-dev\n\n\
    //             pkg_config details:\n{}",
    //             e
    //         ),
    //         _ => panic!("{}", e)
    //     }
    //     }
    // }
    if !path::Path::new("soxr").exists() {
        println!("soxr is not found.");
        std::process::exit(1);
    }
    let soxr_build_dir = cmake::Config::new("soxr")
        .profile("Release")
        .define("WITH_CR32S", "ON")
        .define("WITH_CR64S", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WITH_OPENMP", "OFF")
        .build();
    link_soxr(soxr_build_dir.display());
}
fn link_soxr(soxr_build_dir: impl Display) {
    println!("cargo:info=Linking Soxr as static lib: {}", soxr_build_dir);
    println!("cargo:rustc-link-lib=static=soxr");
    println!("cargo:rustc-link-search=native={}/lib", soxr_build_dir);
}
