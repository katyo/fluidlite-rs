use std::{
    env,
    path::Path,
};

use serde::Deserialize;

fn main() {
    #[cfg(not(feature = "rustdoc"))]
    {
        let out_dir = env::var("OUT_DIR")
            .expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source");

        fetch_source(&src_dir);

        compile_library(&src_dir);
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    metadata: Metadata,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    source: Source,
}

#[derive(Debug, Deserialize)]
struct Source {
    git: String,
    rev: String,
}

fn source_info() -> Source {
    use std::{
        fs::File,
        io::Read,
    };

    use toml::from_slice;

    let manifest_path = Path::new(
        &env::var("CARGO_MANIFEST_DIR")
            .expect("The CARGO_MANIFEST_DIR is set by cargo.")
    ).join("Cargo.toml");

    let mut manifest_data = Vec::new();

    File::open(manifest_path)
        .expect("Manifest file is existing")
        .read_to_end(&mut manifest_data)
        .expect("Manifest file is readable");

    let manifest: Manifest = from_slice(&manifest_data)
        .expect("Valid metadata in manifest");

    manifest.package.metadata.source
}

fn check_source(out_dir: &Path, src: &Source) -> Option<()> {
    use git2::Repository;

    let repo = Repository::open(out_dir).ok()?;

    let remote = repo.find_remote("origin").ok()?;
    remote.url().filter(|url| url == &src.git)?;

    let rev = repo.revparse_single(&src.rev).ok()?;
    repo.checkout_tree(&rev, None).ok()?;

    Some(())
}

fn clone_source(out_dir: &Path, src: &Source) {
    use git2::Repository;

    let repo = Repository::clone(&src.git, out_dir)
        .expect("Fluidlite git repository should exist");

    let rev = repo.revparse_single(&src.rev)
        .expect("Fluidlite git revision should exist");

    let rev = repo.revparse_single(&src.rev)
        .or_else(|_| {
            let mut remote = repo.find_remote("origin")?;

            remote.fetch(&[&src.rev],
                         Some(FetchOptions::new()
                              .update_fetchhead(true)
                              .download_tags(AutotagOption::All)),
                         Some("pull"))?;

            repo.revparse_single(&format!("origin/{}", src.rev))
        })
        .expect("Fluidlite git revision should exist");

    repo.checkout_tree(&rev, None)
        .expect("Fluidlite checkout should be done");
}

fn fetch_source(out_dir: &Path) {
    use std::fs::remove_dir_all;

    let mut src = source_info();

    if let Ok(val) = env::var("FLUIDLITE_GIT") {
        src.git = val;
    }

    if let Ok(val) = env::var("FLUIDLITE_REV") {
        src.rev = val;
    }

    if check_source(out_dir, &src).is_some() {
        return;
    }

    let _ = remove_dir_all(out_dir);

    eprintln!("Fetch fluidlite from {}#{} to {}",
              src.git, src.rev, out_dir.display());

    clone_source(out_dir, &src);
}

fn compile_library(src_dir: &Path) {
    use cmake::Config;

    let lib_name = "fluidlite";

    let library = Config::new(src_dir)
        //.define("FLUIDLITE_BUILD_SHARED", if cfg!(feature = "shared") { "1" } else { "0" })
        //.define("FLUIDLITE_BUILD_STATIC", if cfg!(feature = "shared") { "0" } else { "1" })
        .define("DISABLE_SF3", if cfg!(feature = "with-sf3") { "OFF" } else { "ON" })
        .define("CMAKE_C_COMPILER_WORKS", "1")
        .define("CMAKE_CXX_COMPILER_WORKS", "1")

        .always_configure(true)
        .very_verbose(true)
        .build_target("all")
        .build();

    let lib_dir = library.join("build");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    #[cfg(feature = "shared")]
    println!("cargo:rustc-link-lib={}", lib_name);

    #[cfg(not(feature = "shared"))]
    println!("cargo:rustc-link-lib=static={}", lib_name);
}
