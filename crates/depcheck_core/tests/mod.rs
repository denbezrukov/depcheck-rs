use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::PathBuf;

use relative_path::RelativePathBuf;

use depckeck_core::check::{CheckResult, Checker};
use depckeck_core::options::CheckerOptions;
use pretty_assertions::{assert_eq};

#[derive(Default)]
struct ExpectedCheckResult<'a> {
    using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
    missing_dependencies: BTreeMap<&'a str, &'a HashSet<RelativePathBuf>>,
    unused_dependencies: HashSet<&'a str>,
    unused_dev_dependencies: HashSet<&'a str>,
}

fn assert_result(actual: CheckResult, expected: ExpectedCheckResult) {
    assert_eq!(actual.using_dependencies, expected.using_dependencies);
    assert_eq!(
        actual.get_missing_dependencies(),
        expected.missing_dependencies
    );
    assert_eq!(
        actual.get_unused_dependencies(),
        expected.unused_dependencies
    );
    assert_eq!(
        actual.get_unused_dev_dependencies(),
        expected.unused_dev_dependencies
    );
}

fn get_module_path(name: &str) -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map(|p| {
            p.canonicalize()
                .expect("failed to canonicalize `CARGO_MANIFEST_DIR`")
                .join("tests")
                .join("fake_modules")
                .join(name)
        })
        .unwrap_or_else(|err| panic!("failed to read `CARGO_MANIFEST_DIR`: {}", err))
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

#[test]
fn test_require_resolve_missing() {
    let path = get_module_path("require_resolve_missing");

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
fn test_require_resolve() {
    let path = get_module_path("require_resolve");

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

#[test]
fn test_gatsby() {
    let path = get_module_path("gatsby");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["gatsby-plugin-react-helmet", "gatsby-plugin-sass"]
            .into_iter()
            .collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

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

#[test]
#[ignore]
fn test_good_es7_flow() {
    let path = get_module_path("good_es7_flow");

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

    assert_result(actual, expected);
}

//
// {
// name: 'support SASS/SCSS syntax',
// module: 'sass',
// options: {},
// expected: {
// dependencies: ['unused-sass-dep'],
// devDependencies: [],
// missing: {
// '@test-dep/aFile': ['sass2.sass'],
// '@test-dep/aFile2': ['scss2.scss'],
// '@test-dep/aFile3': ['scss2.scss'],
// '@test-dep/aFile4': ['scss2.scss'],
// sass: ['scss2.scss'],
// },
// using: {
// 'sass-dep': ['sass.sass', 'sass2.sass'],
// 'sass-dep2': ['sass.sass', 'sass2.sass'],
// '@scss-deps/fonts': ['scss.scss'],
// 'scss-dep-2': ['scss.scss'],
// 'scss-dep-3': ['scss.scss'],
// 'scss-dep': ['scss.scss'],
// '@test-dep/aFile': ['sass2.sass'],
// '@test-dep/aFile2': ['scss2.scss'],
// '@test-dep/aFile3': ['scss2.scss'],
// '@test-dep/aFile4': ['scss2.scss'],
// sass: ['scss2.scss'],
// },
// },
// expectedErrorCode: -1,
// },

#[test]
#[ignore]
fn test_vue() {
    let path = get_module_path("vue");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("vue"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("vue-dep-1"),
                [RelativePathBuf::from("component.vue")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("vue-dep-2"),
                [RelativePathBuf::from("component.vue")]
                    .into_iter()
                    .collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
#[ignore]
fn test_vue3() {
    let path = get_module_path("vue3");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("vue"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("vue-dep-1"),
                [RelativePathBuf::from("component.vue")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("vue-dep-2"),
                [RelativePathBuf::from("component.vue")]
                    .into_iter()
                    .collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_missing() {
    let path = get_module_path("missing");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into_iter().collect();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        missing_dependencies: BTreeMap::from([("missing-dep", &missing_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_missing_nested() {
    let path = get_module_path("missing_nested");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into_iter().collect();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("outer-missing-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("used-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        missing_dependencies: BTreeMap::from([("outer-missing-dep", &missing_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_missing_peer_deps() {
    let path = get_module_path("missing_peer_deps");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into_iter().collect();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-this-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("peer-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("optional-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        missing_dependencies: BTreeMap::from([("missing-this-dep", &missing_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
#[ignore]
fn test_grunt() {
    let path = get_module_path("grunt");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("grunt-contrib-jshint"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
#[ignore]
fn test_grunt_tasks() {
    let path = get_module_path("grunt-tasks");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("grunt-contrib-jshint"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_dev() {
    let path = get_module_path("dev");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dev_dependencies: ["unused-dev-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([(
            String::from("used-dep"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_peer_dep() {
    let path = get_module_path("peer_dep");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("host"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("peer"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_peer_dep_nested() {
    let path = get_module_path("peer_dep_nested");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-nested-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("host"),
                [RelativePathBuf::from("nested/index.js")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("peer"),
                [RelativePathBuf::from("nested/index.js")]
                    .into_iter()
                    .collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_optional_dep() {
    let path = get_module_path("optional_dep");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into_iter().collect(),
        using_dependencies: BTreeMap::from([
            (
                String::from("host"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("optional"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_nested() {
    let path = get_module_path("nested");

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
fn test_empty_file() {
    let path = get_module_path("empty_file");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["empty-package"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_shebang() {
    let path = get_module_path("shebang");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["shebang"].into_iter().collect(),
        using_dependencies: BTreeMap::from([(
            String::from("shebang-script"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_empty_dep() {
    let path = get_module_path("empty_dep");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bin_js() {
    let path = get_module_path("bin_js");

    let options = CheckerOptions::default().with_ignore_bin_package(true);
    let checker = Checker::new(options);
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["nobin"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bin_js_ignore_bin_package_false() {
    let path = get_module_path("bin_js");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["anybin", "nobin"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good_ignore_bin_package_true() {
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
fn test_skip_missing_true() {
    let path = get_module_path("missing");

    let options = CheckerOptions::default().with_skip_missing(true);
    let checker = Checker::new(options);
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_skip_missing_false() {
    let path = get_module_path("missing");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into_iter().collect();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([("missing-dep", &missing_files)]),
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_nothing() {
    let path = get_module_path("require_nothing");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["require-nothing"].into_iter().collect(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_dynamic() {
    let path = get_module_path("require_dynamic");

    let checker = Checker::default();
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("dynamic"),
            [RelativePathBuf::from("index.js")].into_iter().collect(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_ignore_matches() {
    let path = get_module_path("bad");

    let options = CheckerOptions::default().with_ignore_matches(vec![String::from(r"o*")]);
    let checker = Checker::new(options);
    let actual = checker.check_package(path).unwrap();

    let expected = ExpectedCheckResult {
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_ignore_matches_for_missing() {
    let path = get_module_path("missing_ignore");

    let options = CheckerOptions::default().with_ignore_matches(vec![
        String::from(r"missing-ignore-[^n][^o][^t]"), /*r"!missing-ignore-not"*/ //TODO https://github.com/isaacs/minimatch
    ]);
    let checker = Checker::new(options);
    let actual = checker.check_package(path).unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into_iter().collect();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([
            ("missing-dep", &missing_files),
            ("missing-ignore-not", &missing_files),
        ]),
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("missing-ignore-dep"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
            (
                String::from("missing-ignore-not"),
                [RelativePathBuf::from("index.js")].into_iter().collect(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}
