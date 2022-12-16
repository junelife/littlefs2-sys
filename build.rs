use embuild::cmd::Cmd;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = cc::Build::new();
    let target = env::var("TARGET")?;
    let builder = builder
        .flag("-std=c11")
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c");

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    builder.compile("lfs-sys");

    let mut builder = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .clang_arg(format!("--target={}", target))
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true);

    let target = env::var("TARGET").unwrap();

    if target == "xtensa-esp32s3-espidf" {
        let mut cmd = Cmd::new("xtensa-esp32s3-elf-ld");
        cmd.arg("--print-sysroot");
        let sysroot = cmd
            .stdout()
            .map(PathBuf::from)
            .expect("Failed to find sysroot");

        builder = builder.clang_arg(format!("--sysroot={}", sysroot.display()));
        // TODO: determine why it isn't sufficient to just set the sysroot.
        builder = builder.clang_arg(format!("-I{}", sysroot.join("include").display()))
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
