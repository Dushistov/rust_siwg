use std::{env, path::Path, time::Instant};

use rust_swig::{DotNetConfig, LanguageConfig};

fn main() {
    env_logger::init();

    let now = Instant::now();

    let out_dir = env::var("OUT_DIR").unwrap();
    rust_swig_expand(
        Path::new("src/glue.rs.in"),
        &Path::new(&out_dir).join("glue.rs"),
    );
    let expand_time = now.elapsed();
    println!(
        "rust swig expand time: {}",
        expand_time.as_secs() as f64 + (expand_time.subsec_nanos() as f64) / 1_000_000_000.
    );
    println!("cargo:rerun-if-changed=src/glue.rs.in");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn rust_swig_expand(from: &Path, out: &Path) {
    println!("Run rust_swig_expand");
    let config = DotNetConfig::new("rust_swig_test_dotnet".to_owned(), "rust_swig_test_dotnet".into());
    let swig_gen = rust_swig::Generator::new(LanguageConfig::DotNetConfig(config));
    swig_gen.expand("rust_swig_test_dotnet_native", from, out);
}
