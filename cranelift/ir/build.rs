// Build script.
//
// This program is run by Cargo when building cranelift-ir. It is used to generate Rust code from
// the language definitions in the cranelift-ir/meta directory.
//
// Environment:
//
// OUT_DIR
//     Directory where generated files should be placed.
//
// TARGET
//     Target triple provided by Cargo.
//
// The build script expects to be run from the directory where this build.rs file lives. The
// current directory is used to find the sources.

use cranelift_ir_meta as meta;

use std::env;
use std::io::Read;
use std::process;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set");
    let target_triple = env::var("TARGET").expect("The TARGET environment variable must be set");

    println!("cargo:rerun-if-changed=build.rs");

    if let Err(err) = meta::generate(&out_dir, None) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    if env::var("CRANELIFT_VERBOSE").is_ok() {
        println!(
            "cargo:warning=Build step took {:?}.",
            Instant::now() - start_time
        );
        println!("cargo:warning=Generated files are in {}", out_dir);
    }

    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let mut cmd = std::process::Command::new("git");
    cmd.arg("rev-parse")
        .arg("HEAD")
        .stdout(std::process::Stdio::piped())
        .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap());
    let version = if let Ok(mut child) = cmd.spawn() {
        let mut git_rev = String::new();
        child
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut git_rev)
            .unwrap();
        let status = child.wait().unwrap();
        if status.success() {
            let git_rev = git_rev.trim().chars().take(9).collect::<String>();
            format!("{}-{}", pkg_version, git_rev)
        } else {
            // not a git repo
            pkg_version
        }
    } else {
        // git not available
        pkg_version
    };
    std::fs::write(
        std::path::Path::new(&out_dir).join("version.rs"),
        format!(
            "/// Version number of this crate. \n\
            pub const VERSION: &str = \"{}\";",
            version
        ),
    )
    .unwrap();
}
