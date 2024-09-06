#[cfg(test)]
#[test]
fn debug() {
    let t = trybuild::TestCases::new();

    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::PathBuf::from(cargo_manifest_dir);
    let path = path.join("integration/src/main.rs");
    let path = path.as_path();
    t.pass(path);
}
