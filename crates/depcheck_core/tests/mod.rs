use depckeck_core::check::{check_package, CheckResult};
use relative_path::RelativePathBuf;
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
                String::from("@package"),
                vec![
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@packageRoot"),
                vec![RelativePathBuf::from("src/rootFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubDir"),
                vec![RelativePathBuf::from("src/subDir/subDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubSubDir"),
                vec![RelativePathBuf::from(
                    "src/subDir/subSubDir/subSubDirFile.ts",
                )]
                .into_iter()
                .collect(),
            ),
            (
                String::from("react"),
                vec![
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
        ]),
        unused_dependencies: vec![String::from("unusedPackage")].into_iter().collect(),
        missing_dependencies: BTreeMap::from([(
            String::from("@package"),
            vec![
                RelativePathBuf::from("src/subDir/subDirFile.ts"),
                RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                RelativePathBuf::from("src/rootFile.ts"),
            ]
            .into_iter()
            .collect(),
        )]),
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_import_function_missing() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("import_function_missing");

    let actual = check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            vec![RelativePathBuf::from("index.js")]
                .into_iter()
                .collect(),
        )]),
        unused_dependencies: Default::default(),
        missing_dependencies: Default::default(),
    };

    assert_eq!(actual, expected);
}
