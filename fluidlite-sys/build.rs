#[cfg(feature = "generate-bindings")]
mod source {
    pub const URL: &str = "https://github.com/katyo/{package}/archive/{version}.tar.gz";
    pub const VERSION: &str = "1.2.1";
}

fn main() {
    #[cfg(any(not(feature = "generate-bindings"), feature = "update-bindings"))]
    use std::path::PathBuf;
    use std::{env, path::Path};

    #[cfg(any(not(feature = "generate-bindings"), feature = "update-bindings"))]
    fn bindings_filename() -> String {
        format!(
            "{}-{}-{}.rs",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            env::var("CARGO_CFG_TARGET_OS").unwrap(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap()
        )
    }

    #[cfg(any(not(feature = "generate-bindings"), feature = "update-bindings"))]
    fn bindings_filepath(filename: &str) -> PathBuf {
        Path::new("src").join("bindings").join(filename)
    }

    #[cfg(not(feature = "generate-bindings"))]
    {
        let bindings_file = bindings_filename();

        if bindings_filepath(&bindings_file).is_file() {
            println!("cargo:rustc-env=FLUIDLITE_BINDINGS={}", bindings_file);
        } else {
            panic!("No prebuilt bindings. Try use `generate-bindings` feature.",);
        }
    }

    #[cfg(feature = "generate-bindings")]
    {
        let src = utils::Source::new(
            "fluidlite",
            env::var("FLUIDLITE_VERSION").unwrap_or_else(|_| source::VERSION.into()),
            env::var("FLUIDLITE_URL").unwrap_or_else(|_| source::URL.into()),
        );

        let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source").join(&src.version);

        utils::fetch_source(&src, &src_dir);

        let inc_dir = src_dir.join("include");
        let bindings = out_dir.join("bindings.rs");

        utils::generate_bindings(&inc_dir, &bindings);

        #[cfg(feature = "update-bindings")]
        {
            let out_path = bindings_filepath(&bindings_filename());
            utils::update_bindings(&bindings, &out_path);
        }
    }
}

#[cfg(feature = "generate-bindings")]
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

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_args(&[format!("-I{}", inc_dir.display())])
            .header(inc_dir.join("fluidlite.h").display().to_string())
            .whitelist_var("FLUID_.*")
            .whitelist_var("SEEK_.*")
            .whitelist_type("fluid_.*")
            .whitelist_function("fluid_.*")
            .whitelist_function("new_fluid_.*")
            .whitelist_function("delete_fluid_.*")
            .generate()
            .expect("Generated bindings.");

        bindings.write_to_file(out_file).expect("Written bindings.");
    }

    #[cfg(feature = "update-bindings")]
    pub fn update_bindings(bind_file: &Path, dest_file: &Path) {
        use std::{env, fs, io::Write};

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
}
