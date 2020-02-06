fn main() {
    #[cfg(feature = "generate-bindings")]
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

        let inc_dir = src_dir.join("include");
        let bindings = out_dir.join("bindings.rs");

        utils::generate_bindings(&inc_dir, &bindings);
    }
}

#[cfg(feature = "generate-bindings")]
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

    pub fn generate_bindings(inc_dir: &Path, out_file: &Path) {
        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_args(&[
                format!("-I{}", inc_dir.display()),
            ])
            .header(inc_dir.join("fluidlite.h").display().to_string())
            .generate()
            .expect("Genrated bindings.");

        bindings
            .write_to_file(out_file)
            .expect("Written bindings.");
    }
}
