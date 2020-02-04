fn main() {
    #[cfg(feature = "generate-bindings")]
    {
        use std::{
            env,
            path::Path,
        };

        let out_dir = env::var("OUT_DIR")
            .expect("The OUT_DIR is set by cargo.");

        let out_dir = Path::new(&out_dir);

        let src_dir = out_dir.join("source");

        utils::fetch_source(&src_dir);

        let inc_dir = src_dir.join("include");
        let bindings = out_dir.join("bindings.rs");

        utils::generate_bindings(&inc_dir, &bindings);
    }
}

#[cfg(feature = "generate-bindings")]
mod utils {
    use std::path::Path;

    use serde::Deserialize;

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
            env,
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
        use git2::{Repository, FetchOptions, AutotagOption};

        let repo = Repository::clone(&src.git, out_dir)
            .expect("Fluidlite git repository should exist");

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

    pub fn fetch_source(out_dir: &Path) {
        use std::{
            env,
            fs::remove_dir_all,
        };

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
