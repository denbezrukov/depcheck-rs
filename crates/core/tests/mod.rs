use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::PathBuf;

use depckeck_rs_core::checker::Checker;
use depckeck_rs_core::checker_result::CheckerResult;
use depckeck_rs_core::config::Config;
use pretty_assertions::assert_eq;

#[derive(Default)]
struct ExpectedCheckResult<'a> {
    using_dependencies: BTreeMap<String, HashSet<String>>,
    missing_dependencies: BTreeMap<&'a str, &'a HashSet<String>>,
    unused_dependencies: HashSet<&'a str>,
    unused_dev_dependencies: HashSet<&'a str>,
}

fn assert_result(actual: CheckerResult, expected: ExpectedCheckResult) {
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
        String::from("src/subDir/subDirFile.ts"),
        String::from("src/subDir/subSubDir/subSubDirFile.ts"),
        String::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();
    let package_first_2_files = [
        String::from("src/subDir/subDirFile.ts"),
        String::from("src/subDir/subSubDir/subSubDirFile.ts"),
        String::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();

    let package_first_3_files = [
        String::from("src/subDir/subDirFile.ts"),
        String::from("src/subDir/subSubDir/subSubDirFile.ts"),
        String::from("src/rootFile.ts"),
    ]
    .into_iter()
    .collect();

    let package_root_first_files = [String::from("src/rootFile.ts")].into_iter().collect();
    let package_sub_first_files = [String::from("src/subDir/subDirFile.ts")]
        .into_iter()
        .collect();

    let package_sub_sub_first = [String::from("src/subDir/subSubDir/subSubDirFile.ts")]
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
                    String::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    String::from("src/subDir/subDirFile.ts"),
                    String::from("src/rootFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@package/first3"),
                [
                    String::from("src/rootFile.ts"),
                    String::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    String::from("src/subDir/subDirFile.ts"),
                ]
                .into_iter()
                .collect(),
            ),
            (
                String::from("@packageRoot/first1"),
                [String::from("src/rootFile.ts")].into_iter().collect(),
            ),
            (
                String::from("@packageSubDir/first1"),
                [String::from("src/subDir/subDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("@packageSubSubDir/first1"),
                [String::from("src/subDir/subSubDir/subSubDirFile.ts")]
                    .into_iter()
                    .collect(),
            ),
            (
                String::from("react"),
                [
                    String::from("src/subDir/subSubDir/subSubDirFile.ts"),
                    String::from("src/subDir/subDirFile.ts"),
                    String::from("src/rootFile.ts"),
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

    let anyone_files = [String::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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

    let anyone_files = [String::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("anyone"),
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            (String::from("find-me"), [String::from("index.js")].into()),
            (
                String::from("default-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("default-member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-alias-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-alias-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-default-star-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-member-alias-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-name-memeber-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("multiple-member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("named-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("star-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("star-import"),
                [String::from("index.js")].into(),
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
            (String::from("optimist"), [String::from("index.js")].into()),
            (String::from("foo"), [String::from("index.js")].into()),
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
                [String::from("index.js")].into(),
            ),
            (
                String::from("default-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("default-member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-alias-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-alias-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-default-star-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-member-alias-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("mixed-name-memeber-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("multiple-member-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("named-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("star-export"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("star-import"),
                [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
                [String::from("component.tsx")].into_iter().collect(),
            ),
            (
                String::from("@types/react"),
                [String::from("component.tsx")].into_iter().collect(),
            ),
            (
                String::from("@types/node"),
                [String::from("esnext.ts")].into(),
            ),
            (
                String::from("@types/org__org-pkg"),
                [String::from("esnext.ts")].into(),
            ),
            (
                String::from("@types/typeless-module"),
                [String::from("typeOnly.ts")].into(),
            ),
            (
                String::from("@org/org-pkg"),
                [String::from("esnext.ts")].into(),
            ),
            (String::from("ts-dep-1"), [String::from("index.ts")].into()),
            (String::from("ts-dep-2"), [String::from("index.ts")].into()),
            (
                String::from("ts-dep-esnext"),
                [String::from("esnext.ts")].into(),
            ),
            (
                String::from("ts-dep-typedef"),
                [String::from("typedef.d.ts")].into_iter().collect(),
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
            (String::from("vue"), [String::from("index.js")].into()),
            (
                String::from("vue-dep-1"),
                [String::from("component.vue")].into_iter().collect(),
            ),
            (
                String::from("vue-dep-2"),
                [String::from("component.vue")].into_iter().collect(),
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
            (String::from("vue"), [String::from("index.js")].into()),
            (
                String::from("vue-dep-1"),
                [String::from("component.vue")].into_iter().collect(),
            ),
            (
                String::from("vue-dep-2"),
                [String::from("component.vue")].into_iter().collect(),
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

    let missing_files = [String::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [String::from("index.js")].into(),
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

    let missing_files = [String::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("outer-missing-dep"),
                [String::from("index.js")].into(),
            ),
            (String::from("used-dep"), [String::from("index.js")].into()),
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

    let missing_files = [String::from("index.js")].into();
    let expected = ExpectedCheckResult {
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-this-dep"),
                [String::from("index.js")].into(),
            ),
            (String::from("peer-dep"), [String::from("index.js")].into()),
            (
                String::from("optional-dep"),
                [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            (String::from("host"), [String::from("index.js")].into()),
            (String::from("peer"), [String::from("index.js")].into()),
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
                [String::from("nested/index.js")].into_iter().collect(),
            ),
            (
                String::from("peer"),
                [String::from("nested/index.js")].into_iter().collect(),
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
            (String::from("host"), [String::from("index.js")].into()),
            (String::from("optional"), [String::from("index.js")].into()),
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
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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
            (String::from("optimist"), [String::from("index.js")].into()),
            (String::from("foo"), [String::from("index.js")].into()),
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
            [String::from("index.js")].into(),
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

    let missing_files = [String::from("index.js")].into();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([("missing-dep", &missing_files)]),
        using_dependencies: BTreeMap::from([(
            String::from("missing-dep"),
            [String::from("index.js")].into(),
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
            [String::from("index.js")].into(),
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

    let missing_files = [String::from("index.js")].into();

    let expected = ExpectedCheckResult {
        missing_dependencies: BTreeMap::from([
            ("missing-dep", &missing_files),
            ("missing-ignore-not", &missing_files),
        ]),
        using_dependencies: BTreeMap::from([
            (
                String::from("missing-dep"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("missing-ignore-dep"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("missing-ignore-not"),
                [String::from("index.js")].into(),
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
            [String::from("index.jsx")].into(),
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
            (String::from("react"), [String::from("index.js")].into()),
            (String::from("jsx-as-js"), [String::from("index.js")].into()),
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
                [String::from("index.js")].into(),
            ),
            (
                String::from("@secondowner/package"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("@org/parent"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("name-import"),
                [String::from("index.js")].into(),
            ),
            (
                String::from("child-import"),
                [String::from("index.js")].into(),
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
            [String::from("index.tsx")].into(),
        )]),
        ..Default::default()
    };

    assert_result(actual, expected);
}

#[test]
fn test_depcheckignore() {
    let path = get_module_path("depcheckignore");

    let config = Config::new(path).with_ignore_path(Some(PathBuf::from(".depcheckignore")));
    let checker = Checker::new(config);
    let actual = checker.check_package().unwrap();

    let missing_files = [String::from("used.js")].into();

    let expected = ExpectedCheckResult {
        unused_dev_dependencies: ["debug"].into(),
        missing_dependencies: BTreeMap::from([("react", &missing_files)]),
        using_dependencies: BTreeMap::from([
            (String::from("lodash"), [String::from("used.js")].into()),
            (String::from("react"), [String::from("used.js")].into()),
        ]),
        ..Default::default()
    };

    assert_result(actual, expected);
}
