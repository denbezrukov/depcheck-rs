use depckeck_core::check::{CheckResult, Checker};
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

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("@package"),
                [
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@packageRoot"),
                [RelativePathBuf::from("src/rootFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubDir"),
                [RelativePathBuf::from("src/subDir/subDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubSubDir"),
                [RelativePathBuf::from(
                    "src/subDir/subSubDir/subSubDirFile.ts",
                )]
                .into_iter()
                .collect(),
            ),
            (
                String::from("react"),
                [
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
        ]),
        unused_dependencies: [String::from("unusedPackage")].into_iter().collect(),
        missing_dependencies: BTreeMap::from([
            (
                String::from("react"),
                [
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@packageRoot"),
                [RelativePathBuf::from("src/rootFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubDir"),
                [RelativePathBuf::from("src/subDir/subDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubSubDir"),
                [RelativePathBuf::from(
                    "src/subDir/subSubDir/subSubDirFile.ts",
                )]
                .into_iter()
                .collect(),
            ),
        ]),
        ..Default::default()
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

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        missing_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_import_function() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("import_function");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_import_function_webpack() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("import_function_webpack");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_require_resolve_missing() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("require_resolve_missing");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        missing_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    // assert_eq!(actual, expected); //TODO
}

#[test]
fn test_require_resolve() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("require_resolve");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    //assert_eq!(actual, expected); //TODO
}

#[test]
fn test_bad() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("bad");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        unused_dependencies: [String::from("optimist")].into_iter().collect(),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_bad_es6() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push("bad_es6");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = CheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("find-me"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("default-export"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("default-member-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("member-alias-export"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("member-alias-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("member-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("mixed-default-star-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("mixed-member-alias-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("mixed-name-memeber-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("multiple-member-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("named-export"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("name-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("star-export"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("star-import"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        unused_dependencies: [String::from("dont-find-me")].into_iter().collect(),
        ..Default::default()
    };

    assert_eq!(actual, expected);
}
