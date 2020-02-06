fn main() {
    #[cfg(not(feature = "rustdoc"))]
    {
        use std::{
            env,
            path::Path,
        };

        let src = utils::Source {
            repository: env::var("FLUIDLITE_REPOSITORY")
                .unwrap_or("https://github.com/katyo/fluidlite".into()),
            version: env::var("FLUIDLITE_VERSION")
                .unwrap_or("1.2.0".into()),
        };

        let out_dir = env::var("OUT_DIR")
            .expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source")
            .join(&src.version);

        let src_dir = utils::fetch_source(&src, &src_dir);

        utils::compile_library(&src_dir);
    }
}

mod utils {
    use std::path::{Path, PathBuf};

    pub struct Source {
        pub repository: String,
        pub version: String,
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) -> PathBuf {
        use std::fs::{metadata, create_dir_all};
        use fetch_unroll::fetch_unroll;

        if !metadata(&out_dir)
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
        {
            let src_url = format!("{repo}/archive/{ver}.tar.gz",
                                  repo = src.repository,
                                  ver = src.version);

            eprintln!("Fetch fluidlite from {} to {}",
                      src_url, out_dir.display());

            create_dir_all(&out_dir.parent().unwrap())
                .expect("Output directory should be created.");

            fetch_unroll(src_url, out_dir)
                .expect("FluidLite sources should be fetched.");
        }

        let mut sub_dirs = out_dir.read_dir()
            .expect("Sources directory should be readable.");

        match (sub_dirs.next(), sub_dirs.next()) {
            // Single subdirectory
            (Some(sub_dir), None) => {
                let sub_dir = sub_dir.unwrap().path();

                if sub_dir.is_dir() {
                    return sub_dir.to_owned();
                } else {
                    panic!("Invalid source contents");
                }
            },
            _ => out_dir.to_owned(),
        }
    }

    pub fn compile_library(src_dir: &Path) {
        use cmake::Config;

        let lib_name = "fluidlite";

        let library = Config::new(src_dir)
        //.define("FLUIDLITE_BUILD_SHARED", if cfg!(feature = "shared") { "1" } else { "0" })
        //.define("FLUIDLITE_BUILD_STATIC", if cfg!(feature = "shared") { "0" } else { "1" })
            .define("ENABLE_SF3", if cfg!(feature = "with-sf3") { "1" } else { "0" })
            .define("STB_VORBIS", if cfg!(feature = "with-stb") { "1" } else { "0" })
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
}
