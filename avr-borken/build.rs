use std::env;
use std::path::PathBuf;
use bindgen;
use bindgen::Builder;

pub trait BlocklistFileMulti {
    fn blocklist_file_multi<T: AsRef<str>, I: IntoIterator<Item = T>>(self, iter: I) -> Self;
}

impl BlocklistFileMulti for bindgen::Builder {
    fn blocklist_file_multi<T: AsRef<str>, I: IntoIterator<Item = T>>(self, iter: I) -> Self {
        let mut builder = self;
        for file in iter {
            println!("# blocking file {}", file.as_ref());
            builder = builder.blocklist_file(file)
        }
        builder
    }
}

fn generate_bindings_generic<T: AsRef<str>, I: IntoIterator<Item = T>>(header: &str, out_basename: &str, excludes: I, malfunction:bool) {
    println!("cargo:rerun-if-changed={}", header);
    let bindings = make_builder_for(header)
        .blocklist_file_multi(excludes)
        .clang_args(if malfunction {
            vec!["-DMALFUNCTION=1"]
        } else {
            vec![]
        })
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_out_file = out_path.join(out_basename);
    bindings
        .write_to_file(bindings_out_file)
        .expect("Couldn't write bindings!");
}

fn make_builder_for(header: &str) -> Builder {
    bindgen::Builder::default()
        .header(header)
        .clang_args(&[
            "-x",
            "c++",
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
}

fn main()
{
    let blank : [&str;0] = [];
    generate_bindings_generic("src-cpp/mixed.h", "mixed.rs", &blank, false);
    generate_bindings_generic("src-cpp/alpha.h", "alpha.rs", &blank, false);
    generate_bindings_generic("src-cpp/alpha.h", "alpha-minus.rs", vec!["src-cpp/base.h"], false);
    generate_bindings_generic("src-cpp/alpha.h", "alpha-wrong.rs", vec!["src-cpp/base.h"], true);
}