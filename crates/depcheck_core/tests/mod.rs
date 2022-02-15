use depckeck_core::check::{CheckDerive, CheckResult, Checker};
use depckeck_core::package::Package;
use relative_path::RelativePathBuf;
use std::collections::{BTreeMap, HashSet};
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
        package: Package {
            name: "package".to_string(),
            version: "1.0.0".to_string(),
            dependencies: BTreeMap::from([
                (String::from("@package"), String::from("1.0.0")),
                (String::from("unusedPackage"), String::from("2.0.0")),
            ]),
            dev_dependencies: Default::default(),
            peer_dependencies: Default::default(),
            bundled_dependencies: Default::default(),
            optional_dependencies: Default::default(),
        },
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
        ..Default::default()
    };

    assert_eq!(actual, expected);

    let actual_derive = actual.get_result();
    let react_files = [
        RelativePathBuf::from("src/subDir/subDirFile.ts"),
        RelativePathBuf::from("src/subDir/subSubDir/subSubDirFile.ts"),
        RelativePathBuf::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();

    let package_root_files = [RelativePathBuf::from("src/rootFile.ts")]
        .into_iter()
        .collect();

    let package_sub_dir_files = [RelativePathBuf::from("src/subDir/subDirFile.ts")]
        .into_iter()
        .collect();

    let package_sub_Ssub_dir_files = [RelativePathBuf::from(
        "src/subDir/subSubDir/subSubDirFile.ts",
    )]
    .into_iter()
    .collect();

    let expected_derive = CheckDerive {
        unused_dependencies: ["unusedPackage"].into_iter().collect(),
        missing_dependencies: BTreeMap::from([
            ("react", &react_files),
            ("@packageRoot", &package_root_files),
            ("@packageSubDir", &package_sub_dir_files),
            ("@packageSubSubDir", &package_sub_Ssub_dir_files),
        ]),
        ..Default::default()
    };

    assert_eq!(actual_derive, expected_derive);
}
//
// #[test]
// fn test_import_function_missing() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("import_function_missing");
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
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_import_function() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("import_function");
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
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_import_function_webpack() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("import_function_webpack");
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
//     assert_eq!(actual, expected);
// }
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
// #[test]
// fn test_bad() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("bad");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         unused_dependencies: [String::from("optimist")].into_iter().collect(),
//         ..Default::default()
//     };
//
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_bad_es6() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("bad_es6");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([
//             (
//                 String::from("find-me"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("default-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("default-member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-alias-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-alias-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-default-star-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-member-alias-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-name-memeber-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("multiple-member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("named-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("name-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("star-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("star-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//         ]),
//         unused_dependencies: [String::from("dont-find-me")].into_iter().collect(),
//         ..Default::default()
//     };
//
//     assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_good() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("good");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([
//             (
//                 String::from("optimist"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("foo"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//         ]),
//         ..Default::default()
//     };
//
//     // assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_good_es6() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("good_es6");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([
//             (
//                 String::from("basic-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("default-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("default-member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-alias-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-alias-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-default-star-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-member-alias-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("mixed-name-memeber-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("multiple-member-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("named-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("name-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("star-export"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//             (
//                 String::from("star-import"),
//                 [RelativePathBuf::from("index.js")].into_iter().collect(),
//             ),
//         ]),
//         unused_dependencies: [String::from("unsupported-syntax")].into_iter().collect(),
//         ..Default::default()
//     };
//
//     assert_eq!(actual, expected);
// }
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
//
// #[test]
// fn test_good_es7() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("good_es7");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         using_dependencies: BTreeMap::from([(
//             String::from("ecmascript-rest-spread"),
//             [RelativePathBuf::from("index.js")].into_iter().collect(),
//         )]),
//         ..Default::default()
//     };
//
//     assert_eq!(actual, expected);
// }
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
//
// #[test]
// fn test_typescript() {
//     let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
//         "test requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
//     ));
//     path.push("tests");
//     path.push("fake_modules");
//     path.push("typescript");
//
//     let checker = Checker::default();
//     let actual = checker.check_package(path).unwrap();
//
//     let expected = CheckResult {
//         unused_dependencies: [String::from("unused-dep")].into_iter().collect(),
//         using_dependencies: BTreeMap::from([
//             (
//                 String::from("react"),
//                 [RelativePathBuf::from("component.tsx")]
//                     .into_iter()
//                     .collect(),
//             ),
//             (
//                 String::from("@types/react"),
//                 [RelativePathBuf::from("component.tsx")]
//                     .into_iter()
//                     .collect(),
//             ),
//             (
//                 String::from("@types/node"),
//                 [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("@types/org__org-pkg"),
//                 [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("@types/typeless-module"),
//                 [RelativePathBuf::from("typeOnly.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("@org/org-pkg"),
//                 [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("ts-dep-1"),
//                 [RelativePathBuf::from("index.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("ts-dep-2"),
//                 [RelativePathBuf::from("index.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("ts-dep-esnext"),
//                 [RelativePathBuf::from("esnext.ts")].into_iter().collect(),
//             ),
//             (
//                 String::from("ts-dep-typedef"),
//                 [RelativePathBuf::from("typedef.d.ts")]
//                     .into_iter()
//                     .collect(),
//             ),
//         ]),
//         ..Default::default()
//     };
//
//     // assert_eq!(actual, expected);
// }
