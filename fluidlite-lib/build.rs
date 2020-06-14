mod source {
    pub const URL: &str = "https://github.com/katyo/{package}/archive/{version}.tar.gz";
    pub const VERSION: &str = "1.2.1";
}

fn main() {
    #[cfg(not(feature = "rustdoc"))]
    {
        use std::{env, path::Path};

        let src = utils::Source::new(
            "fluidlite",
            env::var("FLUIDLITE_VERSION").unwrap_or(source::VERSION.into()),
            env::var("FLUIDLITE_URL").unwrap_or(source::URL.into()),
        );

        let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source").join(&src.version);

        let bld_dir = out_dir.join("build").join(&src.version);

        utils::fetch_source(&src, &src_dir);

        utils::compile_library(&src_dir, &bld_dir);
    }
}

mod utils {
    use std::path::Path;

    pub struct Source {
        pub package: String,
        pub version: String,
        pub url: String,
    }

    impl Source {
        pub fn new(
            package: impl Into<String>,
            version: impl Into<String>,
            url: impl Into<String>,
        ) -> Self {
            Self {
                package: package.into(),
                version: version.into(),
                url: url.into(),
            }
        }

        pub fn url(&self) -> String {
            self.url
                .replace("{package}", &self.package)
                .replace("{version}", &self.version)
        }
    }

    pub fn fetch_source(src: &Source, out_dir: &Path) {
        use fetch_unroll::Fetch;

        if !out_dir.is_dir() {
            let src_url = src.url();

            eprintln!("Fetch fluidlite from {} to {}", src_url, out_dir.display());

            Fetch::from(src_url)
                .unroll()
                .strip_components(1)
                .to(out_dir)
                .expect("FluidLite sources should be fetched.");
        }
    }

    pub fn lib_file<S: AsRef<str>>(name: S, shared: bool) -> String {
        #[cfg(target_os = "windows")]
        {
            format!("{}.{}", name.as_ref(), if shared { "dll" } else { "lib" })
        }

        #[cfg(not(target_os = "windows"))]
        {
            format!("lib{}.{}", name.as_ref(), if shared { "so" } else { "a" })
        }
    }

    pub fn bool_flag(flag: bool) -> &'static str {
        if flag {
            "ON"
        } else {
            "OFF"
        }
    }

    pub fn compile_library(src_dir: &Path, out_dir: &Path) {
        use cmake::Config;

        let lib_dir = out_dir.join("lib");

        let lib_name = "fluidlite";

        if !lib_dir
            .join(lib_file(&lib_name, cfg!(feature = "shared")))
            .is_file()
        {
            std::fs::create_dir_all(out_dir).unwrap();

            Config::new(src_dir)
                //.define("FLUIDLITE_BUILD_SHARED", bool_flag(cfg!(feature = "shared")))
                //.define("FLUIDLITE_BUILD_STATIC", bool_flag(!cfg!(feature = "shared")))
                .define("ENABLE_SF3", bool_flag(cfg!(feature = "with-sf3")))
                .define("STB_VORBIS", bool_flag(cfg!(feature = "with-stb")))
                .define("CMAKE_C_COMPILER_WORKS", bool_flag(true))
                .define("CMAKE_CXX_COMPILER_WORKS", bool_flag(true))
                .always_configure(true)
                .very_verbose(true)
                .out_dir(out_dir)
                .build();
        }

        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        #[cfg(feature = "shared")]
        println!("cargo:rustc-link-lib={}", lib_name);

        #[cfg(not(feature = "shared"))]
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }
}
