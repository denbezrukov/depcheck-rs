use depckeck_core::check::check_directory;
use depckeck_core::package::Package;
use std::env;
use std::path::PathBuf;

#[test]
fn test_package() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("package");

    let mut package_path = path.clone();
    package_path.push("package.json");
    let package = Package::from_path(package_path).unwrap();

    println!("{:#?}", package);

    check_directory(&path);
}
