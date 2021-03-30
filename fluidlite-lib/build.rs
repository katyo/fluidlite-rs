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
            env::var("FLUIDLITE_VERSION").unwrap_or_else(|_| source::VERSION.into()),
            env::var("FLUIDLITE_URL").unwrap_or_else(|_| source::URL.into()),
        );

        let out_dir = env::var("OUT_DIR").expect("The OUT_DIR is set by cargo.");
        let out_dir = Path::new(&out_dir);
        let src_dir = out_dir.join("source").join(&src.version);

        utils::fetch_source(&src, &src_dir);
        utils::compile_library(&src_dir, &out_dir);
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

    fn add_lib(name: &str, static_: bool) {
        println!(
            "cargo:rustc-link-lib={}{}",
            if static_ { "static=" } else { "" },
            name
        );
    }

    pub fn compile_library(src_dir: &Path, lib_dir: &Path) {
        let lib_name = "fluidlite";

        if !lib_dir
            .join(lib_file(&lib_name, cfg!(feature = "shared")))
            .is_file()
        {
            let mut build = cc::Build::new();

            build.flag_if_supported("-std=c99");

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
                    fn use_library(build: &mut cc::Build, pkg: &pkg_config::Library) {
                        for (k, v) in pkg.defines {
                            build.define(k, v);
                        }
                        build.includes(&pkg.include_paths);
                        for path in &pkg.link_paths {
                            println!("cargo:rustc-link-search={}", path);
                        }
                        for lib in &pkg.libs {
                            add_lib(lib, cfg!(feature = "static"));
                        }
                    }

                    build.define("SF3_SUPPORT", "SF3_XIPH_VORBIS");

                    if let (Some(vorbis), Some(vorbisfile)) = (
                        pkg_config::Config::new()
                            .atleast_version("1.3.5")
                            .probe("vorbis"),
                        pkg_config::Config::new()
                            .atleast_version("1.3.5")
                            .probe("vorbisfile"),
                    ) {
                        use_library(&mut build, &vorbis);
                        use_library(&mut build, &vorbisfile);
                    } else {
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

                    if let Some(ogg) = pkg_config::Config::new()
                        .atleast_version("1.3.2")
                        .probe("ogg")
                    {
                        use_library(&mut build, &ogg);
                    } else {
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

            build.compile("fluidlite");
        }

        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        add_lib(lib_name, !cfg!(not(feature = "shared")));
    }
}
