use depckeck_core::check::{check_package, CheckResult};
use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

#[test]
fn test_package() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("package");

    let actual = check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([
            (
                PathBuf::from("src\rootFile.ts"),
                vec![
                    String::from("react"),
                    String::from("@packageRoot"),
                    String::from("@package"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                PathBuf::from("src/subDir/subDirFile.ts"),
                vec![
                    String::from("react"),
                    String::from("@packageRoot"),
                    String::from("@package"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                PathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                vec![
                    String::from("react"),
                    String::from("@packageSubSubDir"),
                    String::from("@package"),
                ]
                .into_iter()
                .collect(),
            ),
        ]),
        unused_dependencies: vec![String::from("unusedPackage")].into_iter().collect(),
        missing_dependencies: vec![
            String::from("@packageSubSubDir"),
            String::from("react"),
            String::from("@packageRoot"),
            String::from("@packageSubDir"),
        ]
        .into_iter()
        .collect(),
    };

    assert_eq!(actual, expected);
}
