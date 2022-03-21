use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::PathBuf;

use relative_path::RelativePathBuf;

use depckeck_core::checker::{CheckResult, Checker};
use depckeck_core::config::Config;
use pretty_assertions::assert_eq;

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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let anyone_files = [RelativePathBuf::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        missing_dependencies: BTreeMap::from([("anyone", &anyone_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_import_function() {
    let path = get_module_path("import_function");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_import_function_webpack() {
    let path = get_module_path("import_function_webpack");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_resolve_missing() {
    let path = get_module_path("require_resolve_missing");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let anyone_files = [RelativePathBuf::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        missing_dependencies: BTreeMap::from([("anyone", &anyone_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_resolve() {
    let path = get_module_path("require_resolve");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bad() {
    let path = get_module_path("bad");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["optimist"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bad_es6() {
    let path = get_module_path("bad_es6");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("find-me"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("default-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("default-member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-alias-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-alias-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-default-star-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-member-alias-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-name-memeber-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("multiple-member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("named-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("star-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("star-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        unused_dependencies: ["dont-find-me"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good() {
    let path = get_module_path("good");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("optimist"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("foo"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good_es6() {
    let path = get_module_path("good_es6");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("basic-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("default-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("default-member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-alias-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-alias-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-default-star-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-member-alias-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("mixed-name-memeber-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("multiple-member-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("named-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("star-export"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("star-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        unused_dependencies: ["unsupported-syntax"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_gatsby() {
    let path = get_module_path("gatsby");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("ecmascript-rest-spread"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
#[ignore]
fn test_good_es7_flow() {
    let path = get_module_path("good_es7_flow");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("ecmascript-rest-spread"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_typescript() {
    let path = get_module_path("typescript");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into(),
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
                [RelativePathBuf::from("esnext.ts")].into(),
            ),
            (
                String::from("@types/org__org-pkg"),
                [RelativePathBuf::from("esnext.ts")].into(),
            ),
            (
                String::from("@types/typeless-module"),
                [RelativePathBuf::from("typeOnly.ts")].into(),
            ),
            (
                String::from("@org/org-pkg"),
                [RelativePathBuf::from("esnext.ts")].into(),
            ),
            (
                String::from("ts-dep-1"),
                [RelativePathBuf::from("index.ts")].into(),
            ),
            (
                String::from("ts-dep-2"),
                [RelativePathBuf::from("index.ts")].into(),
            ),
            (
                String::from("ts-dep-esnext"),
                [RelativePathBuf::from("esnext.ts")].into(),
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
// config: {},
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into(),
        using_dependencies: BTreeMap::from([
            (
                String::from("vue"),
                [RelativePathBuf::from("index.js")].into(),
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into(),
        using_dependencies: BTreeMap::from([
            (
                String::from("vue"),
                [RelativePathBuf::from("index.js")].into(),
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        missing_dependencies: BTreeMap::from([("missing-dep", &missing_files)]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_missing_nested() {
    let path = get_module_path("missing_nested");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("outer-missing-dep"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("used-dep"),
                [RelativePathBuf::from("index.js")].into(),
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-this-dep"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("peer-dep"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("optional-dep"),
                [RelativePathBuf::from("index.js")].into(),
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("grunt-contrib-jshint"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
#[ignore]
fn test_grunt_tasks() {
    let path = get_module_path("grunt-tasks");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("grunt-contrib-jshint"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_dev() {
    let path = get_module_path("dev");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dev_dependencies: ["unused-dev-dep"].into(),
        using_dependencies: BTreeMap::from([(
            String::from("used-dep"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_peer_dep() {
    let path = get_module_path("peer_dep");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into(),
        using_dependencies: BTreeMap::from([
            (
                String::from("host"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("peer"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_peer_dep_nested() {
    let path = get_module_path("peer_dep_nested");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-nested-dep"].into(),
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

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["unused-dep"].into(),
        using_dependencies: BTreeMap::from([
            (
                String::from("host"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("optional"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_nested() {
    let path = get_module_path("nested");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("optimist"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_empty_file() {
    let path = get_module_path("empty_file");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["empty-package"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_shebang() {
    let path = get_module_path("shebang");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["shebang"].into(),
        using_dependencies: BTreeMap::from([(
            String::from("shebang-script"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_empty_dep() {
    let path = get_module_path("empty_dep");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bin_js() {
    let path = get_module_path("bin_js");

    let config = Config::new(path).with_ignore_bin_package(true);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["nobin"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_bin_js_ignore_bin_package_false() {
    let path = get_module_path("bin_js");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["anybin", "nobin"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_good_ignore_bin_package_true() {
    let path = get_module_path("good");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("optimist"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("foo"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_skip_missing_true() {
    let path = get_module_path("missing");

    let config = Config::new(path).with_skip_missing(true);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_skip_missing_false() {
    let path = get_module_path("missing");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([("missing-dep", &missing_files)]),
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_nothing() {
    let path = get_module_path("require_nothing");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["require-nothing"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_require_dynamic() {
    let path = get_module_path("require_dynamic");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("dynamic"),
            [RelativePathBuf::from("index.js")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_ignore_matches() {
    let path = get_module_path("bad");

    let config = Config::new(path).with_ignore_matches(vec![String::from(r"o*")]);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_ignore_matches_for_missing() {
    let path = get_module_path("missing_ignore");

    let config = Config::new(path).with_ignore_matches(vec![
        String::from(r"missing-ignore-[^n][^o][^t]"), /*r"!missing-ignore-not"*/ //TODO https://github.com/isaacs/minimatch
    ]);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("index.js")].into();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([
            ("missing-dep", &missing_files),
            ("missing-ignore-not", &missing_files),
        ]),
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-dep"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("missing-ignore-dep"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("missing-ignore-not"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_jsx() {
    let path = get_module_path("jsx");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("react"),
            [RelativePathBuf::from("index.jsx")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_jsx_js() {
    let path = get_module_path("jsx_js");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("react"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("jsx-as-js"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_scoped_module() {
    let path = get_module_path("scoped_module");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["@unused/package"].into(),
        using_dependencies: BTreeMap::from([
            (
                String::from("@owner/package"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("@secondowner/package"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("@org/parent"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
            (
                String::from("child-import"),
                [RelativePathBuf::from("index.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_ignore_number() {
    let path = get_module_path("ignore_number");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        unused_dependencies: ["number"].into(),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_decorators() {
    let path = get_module_path("decorators");

    let config = Config::new(path);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("mobx"),
            [RelativePathBuf::from("index.tsx")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_depcheckignore() {
    let path = get_module_path("depcheckignore");

    let config = Config::new(path).with_read_depcheckignore(true);
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [RelativePathBuf::from("used.js")].into();

    let expected = ExpectedCheckResult {
        unused_dev_dependencies: ["debug"].into(),
        missing_dependencies: BTreeMap::from([("react", &missing_files)]),
        using_dependencies: BTreeMap::from([
            (
                String::from("lodash"),
                [RelativePathBuf::from("used.js")].into(),
            ),
            (
                String::from("react"),
                [RelativePathBuf::from("used.js")].into(),
            ),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}
