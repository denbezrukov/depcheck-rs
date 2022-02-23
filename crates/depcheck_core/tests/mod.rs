use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::PathBuf;

use relative_path::RelativePathBuf;

use depckeck_core::check::{CheckResult, Checker};

#[derive(Default)]
struct ExpectedCheckResult<'a> {
    using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
    missing_dependencies: BTreeMap<&'a str, &'a HashSet<RelativePathBuf>>,
    unused_dependencies: HashSet<&'a str>,
    unused_dev_dependencies: HashSet<&'a str>,
}

fn assert_result(actual: CheckResult, expected: ExpectedCheckResult) {
    assert_eq!(actual.using_dependencies, expected.using_dependencies);
    // assert_eq!(
    //     actual.get_missing_dependencies(),
    //     expected.missing_dependencies
    // );
    // assert_eq!(
    //     actual.get_unused_dependencies(),
    //     expected.unused_dependencies
    // );
    // assert_eq!(
    //     actual.get_unused_dev_dependencies(),
    //     expected.unused_dev_dependencies
    // );
}

fn get_module_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));
    path.push("tests");
    path.push("fake_modules");
    path.push(name);
    path
}

#[test]
fn test_package() {
    let path = get_module_path("package");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let react_files = [
        RelativePathBuf::from("src/subDir/subDirFile.ts"),
        RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
        RelativePathBuf::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();
    let package_first_2_files = [
        RelativePathBuf::from("src/subDir/subDirFile.ts"),
        RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
        RelativePathBuf::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();

    let package_first_3_files = [
        RelativePathBuf::from("src/subDir/subDirFile.ts"),
        RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
        RelativePathBuf::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();

    let package_root_first_files = [RelativePathBuf::from("src/rootFile.ts")]
        .into_iter()
        .collect();
    let package_sub_first_files = [RelativePathBuf::from("src/subDir/subDirFile.ts")]
        .into_iter()
        .collect();

    let package_sub_sub_first = [RelativePathBuf::from(
        "src/subDir/subSubDir/subSubDirFile.ts",
    )]
    .into_iter()
    .collect();

    let missing_dependencies = BTreeMap::from([
        ("react", &react_files),
        ("@package/first2", &package_first_2_files),
        ("@package/first3", &package_first_3_files),
        ("@packageRoot/first1", &package_root_first_files),
        ("@packageSubDir/first1", &package_sub_first_files),
        ("@packageSubSubDir/first1", &package_sub_sub_first),
    ]);

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("@package/first2"),
                [
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                    RelativePathBuf::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@package/first3"),
                [
                    RelativePathBuf::from("src/rootFile.ts"),
                    RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    RelativePathBuf::from("src/subDir/subDirFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@packageRoot/first1"),
                [RelativePathBuf::from("src/rootFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubDir/first1"),
                [RelativePathBuf::from("src/subDir/subDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubSubDir/first1"),
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
        missing_dependencies,
        unused_dependencies: ["unusedPackage", "@package/unusedPackage"]
            .into_iter()
            .collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_import_function_missing() {
    let path = get_module_path("import_function_missing");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let anyone_files = [RelativePathBuf::from("index.js")].into_iter().collect();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        missing_dependencies: BTreeMap::from([("anyone", &anyone_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_import_function() {
    let path = get_module_path("import_function");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_import_function_webpack() {
    let path = get_module_path("import_function_webpack");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

//
// #[test]
// fn test_require_resolve_missing() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("require_resolve_missing");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([(
//             String::from("anyone"),
//             [RelativePathBuf::from("index.js")].into_iter().collect(),
//         )]),
//         missing_dependencies: BTreeMap::from([(
//             String::from("anyone"),
//             [RelativePathBuf::from("index.js")].into_iter().collect(),
//         )]),
//         ..Default::default()
//     };
//
//     // assert_eq!(actual, expected); //TODO
// }
//
// #[test]
// fn test_require_resolve() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("require_resolve");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([(
//             String::from("optimist"),
//             [RelativePathBuf::from("index.js")].into_iter().collect(),
//         )]),
//         ..Default::default()
//     };
//
//     //assert_eq!(actual, expected); //TODO
// }
//
#[test]
fn test_bad() {
    let path = get_module_path("bad");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["optimist"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bad_es6() {
    let path = get_module_path("bad_es6");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
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
        unused_dependencies: ["dont-find-me"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good() {
    let path = get_module_path("good");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("optimist"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("foo"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good_es6() {
    let path = get_module_path("good_es6");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("basic-import"),
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
        unused_dependencies: ["unsupported-syntax"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}
//
// #[test]
// fn test_gatsby() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("gatsby");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         unused_dependencies: [
//             String::from("gatsby-plugin-react-helmet"),
//             String::from("gatsby-plugin-sass"),
//         ]
//         .into_iter()
//         .collect(),
//         ..Default::default()
//     };
//
//     // assert_eq!(actual, expected);
// }

#[test]
fn test_good_es7() {
    let path = get_module_path("good_es7");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("ecmascript-rest-spread"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}
//
// #[test]
//fn test_good_es7_flow() {
// let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//     "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
// ));
// path.push("tests");
// path.push("fake_modules");
// path.push("good_es7_flow");
//
// let checker = Checker::default();
// let actual = checker.check_package(path).unwrap();
//
// let expected = CheckResult {
//     using_dependencies: BTreeMap::from([(
//         String::from("ecmascript-rest-spread"),
//         [RelativePathBuf::from("index.js")].into_iter().collect(),
//     )]),
//     ..Default::default()
//     };
//
//     // assert_eq!(actual, expected);
// }

#[test]
fn test_typescript() {
    let path = get_module_path("typescript");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("react"),
                [RelativePathBuf::from("component.tsx")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@types/react"),
                [RelativePathBuf::from("component.tsx")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@types/node"),
                [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
            ),
            (
                String::from("@types/org__org-pkg"),
                [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
            ),
            (
                String::from("@types/typeless-module"),
                [RelativePathBuf::from("typeOnly.ts")].into_iter().collect(),
            ),
            (
                String::from("@org/org-pkg"),
                [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
            ),
            (
                String::from("ts-dep-1"),
                [RelativePathBuf::from("index.ts")].into_iter().collect(),
            ),
            (
                String::from("ts-dep-2"),
                [RelativePathBuf::from("index.ts")].into_iter().collect(),
            ),
            (
                String::from("ts-dep-esnext"),
                [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
            ),
            (
                String::from("ts-dep-typedef"),
                [RelativePathBuf::from("typedef.d.ts")]
                    .into_iter()
                    .collect(),
            ),
        ]),
        ..Default::default()
    };

    println!("{:#?}", actual);

    assert_result(actual, expected);
}
