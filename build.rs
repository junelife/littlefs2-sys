use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = cc::Build::new();
    let builder = builder
        .flag("-std=c11")
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c");

    println!("cargo:rerun-if-changed=littlefs/lfs.c");
    println!("cargo:rerun-if-changed=littlefs/lfs_util.c");
    println!("cargo:rerun-if-changed=littlefs/lfs_util.h");

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    builder.compile("lfs-sys");

    let bindings = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .allowlist_type("lfs.*")
        .allowlist_function("lfs_.*")
        .allowlist_var("LFS_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
