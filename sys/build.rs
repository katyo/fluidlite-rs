use std::path::Path;

const LIB_NAME: &str = "fluidlite";

fn main() {
    use std::env;

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filename() -> String {
        format!(
            "{}-{}-{}.rs",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        )
    }

    #[cfg(any(not(feature = "bindgen"), feature = "update-bindings"))]
    fn bindings_filepath(filename: &str) -> impl AsRef<Path> {
        Path::new("src").join("bindings").join(filename)
    }

    #[cfg(not(feature = "bindgen"))]
    {
        let bindings_file = bindings_filename();

        if bindings_filepath(&bindings_file).as_ref().is_file() {
            println!("cargo:rustc-env=FLUIDLITE_BINDINGS={}", bindings_file);
        } else {
            panic!("No prebuilt bindings. Try use `bindgen` feature.",);
        }
    }

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");
    let out_dir = Path::new(&out_dir);

    let src_dir = Path::new("lib");

    #[cfg(feature = "bindgen")]
    {
        let inc_dirs = try_find_library_inc_dirs().unwrap_or_else(|| vec![src_dir.join("include")]);

        let bindings = out_dir.join("bindings.rs");

        generate_bindings(inc_dirs, &bindings);

        #[cfg(feature = "update-bindings")]
        {
            let out_path = bindings_filepath(&bindings_filename());
            update_bindings(&bindings, &out_path);
        }
    }

    if !try_find_and_use_library() {
        let lib_dir = out_dir;
        //if !has_lib_file(lib_dir, LIB_NAME, cfg!(feature = "shared")) {
        build_library(src_dir, lib_dir);
        //}

        add_lib_path(lib_dir);
        add_lib(LIB_NAME, !cfg!(not(feature = "shared")));
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings<P: AsRef<Path>>(
    inc_dirs: impl IntoIterator<Item = P>,
    out_file: impl AsRef<Path>,
) {
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_args(
            inc_dirs
                .into_iter()
                .map(|dir| format!("-I{}", dir.as_ref().display())),
        )
        .header_contents("library.h", "#include <fluidlite.h>")
        .allowlist_var("FLUID_.*")
        .allowlist_var("SEEK_.*")
        .allowlist_type("fluid_.*")
        .allowlist_function("fluid_.*")
        .allowlist_function("new_fluid_.*")
        .allowlist_function("delete_fluid_.*")
        .generate()
        .expect("Generated bindings.");

    bindings.write_to_file(out_file).expect("Written bindings.");
}

#[cfg(feature = "update-bindings")]
fn update_bindings(bind_file: impl AsRef<Path>, dest_file: impl AsRef<Path>) {
    use std::{env, fs, io::Write};

    let dest_file = dest_file.as_ref();

    fs::create_dir_all(&dest_file.parent().unwrap()).unwrap();
    fs::copy(&bind_file, &dest_file).unwrap();

    if let Ok(github_env) = env::var("GITHUB_ENV") {
        let mut env_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(github_env)
            .unwrap();
        writeln!(
            env_file,
            "FLUIDLITE_SYS_BINDINGS_FILE={}",
            dest_file.display()
        )
        .unwrap();
    }
}

/*
fn lib_file(name: impl AsRef<str>, shared: bool) -> String {
    #[cfg(target_os = "windows")]
    {
        format!("{}.{}", name.as_ref(), if shared { "dll" } else { "lib" })
    }

    #[cfg(not(target_os = "windows"))]
    {
        format!("lib{}.{}", name.as_ref(), if shared { "so" } else { "a" })
    }
}

fn has_lib_file(lib_dir: impl AsRef<Path>, name: impl AsRef<str>, shared: bool) -> bool {
    lib_dir.as_ref().join(lib_file(name, shared)).is_file()
}
*/

fn add_lib(name: &str, static_: bool) {
    println!(
        "cargo:rustc-link-lib={}{}",
        if static_ { "static=" } else { "" },
        name
    );
}

fn add_lib_path(path: impl AsRef<Path>) {
    println!("cargo:rustc-link-search={}", path.as_ref().display());
}

#[cfg(feature = "pkg-config")]
fn rust_use_pkg(pkg: &pkg_config::Library) {
    for path in &pkg.link_paths {
        add_lib_path(path);
    }
    for lib in &pkg.libs {
        add_lib(lib, cfg!(feature = "static"));
    }
}

#[cfg(feature = "pkg-config")]
fn cc_use_pkg(build: &mut cc::Build, pkg: &pkg_config::Library) {
    for (k, v) in &pkg.defines {
        if let Some(v) = v {
            build.define(k, v.as_ref());
        } else {
            build.define(k, None);
        }
    }
    build.includes(&pkg.include_paths);
    rust_use_pkg(pkg);
}

#[cfg(feature = "pkg-config")]
fn find_pkgs<S: AsRef<str>, V: AsRef<str>>(
    libs: impl IntoIterator<Item = (S, V)>,
) -> Option<Vec<pkg_config::Library>> {
    libs.into_iter()
        .map(|(name, version)| {
            pkg_config::Config::new()
                .atleast_version(version.as_ref())
                .probe(name.as_ref())
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

#[allow(unused)]
#[cfg(not(feature = "pkg-config"))]
fn try_find_and_use_pkgs<S: AsRef<str>, V: AsRef<str>>(
    _build: &mut cc::Build,
    _libs: impl IntoIterator<Item = (S, V)>,
) -> bool {
    false
}

#[allow(unused)]
#[cfg(feature = "pkg-config")]
fn try_find_and_use_pkgs<S: AsRef<str>, V: AsRef<str>>(
    build: &mut cc::Build,
    libs: impl IntoIterator<Item = (S, V)>,
) -> bool {
    find_pkgs(libs)
        .map(|pkgs| {
            for pkg in &pkgs {
                cc_use_pkg(build, pkg);
            }
            true
        })
        .unwrap_or(false)
}

#[cfg(feature = "pkg-config")]
fn find_library() -> Option<pkg_config::Library> {
    // try find system-wide library
    pkg_config::Config::new()
        .atleast_version("1.2.1")
        .probe(LIB_NAME)
        .ok()
}

fn try_find_and_use_library() -> bool {
    #[cfg(any(feature = "builtin", not(feature = "pkg-config")))]
    {
        false
    }

    #[cfg(all(not(feature = "builtin"), feature = "pkg-config"))]
    {
        find_library()
            .map(|pkg| {
                // Use installed system-wide package
                rust_use_pkg(&pkg);
                true
            })
            .unwrap_or(false)
    }
}

#[cfg(feature = "bindgen")]
fn try_find_library_inc_dirs() -> Option<Vec<std::path::PathBuf>> {
    #[cfg(not(feature = "pkg-config"))]
    {
        None
    }

    #[cfg(feature = "pkg-config")]
    {
        find_library().map(|pkg| pkg.include_paths)
    }
}

fn check_header(out_dir: &Path, build: &mut cc::Build, name: &str, required: bool) {
    use std::fs;

    let compiled = {
        let tmp_src = out_dir.join(format!("check_header_{}.c", name));
        fs::write(&tmp_src, format!("#include \"{}\"", name)).unwrap();

        let mut build = build.clone();
        build.file(&tmp_src);

        let tmp_obj = format!("check_header_{}.o", name);
        build.try_compile(&tmp_obj).is_ok()
    };

    if compiled {
        let tmp_def = format!(
            "HAVE_{}",
            name.to_ascii_uppercase()
                .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
        );
        eprintln!("#define {} 1", &tmp_def);
        build.define(&tmp_def, Some("1"));
    } else if required {
        panic!("The required header \"{}\" does not found.", name);
    } else {
        eprintln!("The header \"{}\" does not found.", name);
    }
}

fn build_library(src_dir: &Path, lib_dir: &Path) {
    let mut build = cc::Build::new();

    build.out_dir(lib_dir);
    build.flag_if_supported("-std=c99");

    for header in &[
        "string.h", "stdlib.h", "stdio.h", "math.h", "stdarg.h", "fcntl.h", "limits.h",
    ] {
        check_header(lib_dir, &mut build, header, true);
    }

    build.include(src_dir.join("include"));
    build.files(
        [
            "fluid_init.c",
            "fluid_chan.c",
            "fluid_chorus.c",
            "fluid_conv.c",
            "fluid_defsfont.c",
            "fluid_dsp_float.c",
            "fluid_gen.c",
            "fluid_hash.c",
            "fluid_list.c",
            "fluid_mod.c",
            "fluid_ramsfont.c",
            "fluid_rev.c",
            "fluid_settings.c",
            "fluid_synth.c",
            "fluid_sys.c",
            "fluid_tuning.c",
            "fluid_voice.c",
        ]
        .iter()
        .map(|src| src_dir.join("src").join(src)),
    );

    #[cfg(feature = "with-sf3")]
    {
        #[cfg(feature = "with-stb")]
        {
            build.define("SF3_SUPPORT", "SF3_STB_VORBIS");
            build.include(src_dir.join("stb"));
            build.file(src_dir.join("stb").join("stb_vorbis.c"));
        }

        #[cfg(not(feature = "with-stb"))]
        {
            build.define("SF3_SUPPORT", "SF3_XIPH_VORBIS");

            if !try_find_and_use_pkgs(&mut build, [("vorbis", "1.3.5"), ("vorbisfile", "1.3.5")]) {
                // use shipped libvorbis sources
                let src_dir = src_dir.join("libvorbis-1.3.5");

                build.include(src_dir.join("include"));
                build.include(src_dir.join("lib"));
                build.files(
                    [
                        "vorbisenc.c",
                        "info.c",
                        "analysis.c",
                        "bitrate.c",
                        "block.c",
                        "codebook.c",
                        "envelope.c",
                        "floor0.c",
                        "floor1.c",
                        "lookup.c",
                        "lpc.c",
                        "lsp.c",
                        "mapping0.c",
                        "mdct.c",
                        "psy.c",
                        "registry.c",
                        "res0.c",
                        "sharedbook.c",
                        "smallft.c",
                        "vorbisfile.c",
                        "window.c",
                        "synthesis.c",
                    ]
                    .iter()
                    .map(|src| src_dir.join("lib").join(src)),
                );
            }

            if !try_find_and_use_pkgs(&mut build, [("ogg", "1.3.2")]) {
                // use shipped libogg sources
                let src_dir = src_dir.join("libogg-1.3.2");

                build.include(src_dir.join("include"));
                build.files(
                    ["bitwise.c", "framing.c"]
                        .iter()
                        .map(|src| src_dir.join("src").join(src)),
                );
            }
        }
    }

    #[cfg(feature = "shared")]
    build.shared_flag(true);

    #[cfg(feature = "static")]
    build.static_flag(true);

    build.compile(LIB_NAME);
}
