use regex::{Regex, Captures};

pub fn extract_package_name(path: &str) -> Option<String> {
    let scope_pattern = Regex::new(r"^(?:(@[^/]+)[/]+)([^/]+)[/]?").unwrap();
    let base_pattern = Regex::new(r"^([^/]+)[/]?").unwrap();
    let scope_pattern_test = Regex::new(r"^@").unwrap();

    if scope_pattern_test.is_match(path) {
        let captures = scope_pattern.captures(path)?;

        return match (captures.get(1), captures.get(2)) {
            (Some(first), Some(second)) => Some(first.as_str().to_string() + "/" + second.as_str()),
            _ => None,
        };
    } else {
        let captures = base_pattern.captures(path)?;
        captures.get(1).map(|v| v.as_str().to_string())
    }
}

pub fn extract_types_name(path: &str) -> String {
    let organization_dependency = Regex::new(r"@(.*?)/(.*)").unwrap();

    let path = match organization_dependency.captures(path) {
        Some(captures) => {
            captures.get(1).unwrap().as_str().to_string() + "__" + captures.get(2).unwrap().as_str()
        }
        None => {
            path.to_string()
        }
    };

    "@types/".to_string() + path.as_str()
}

#[cfg(test)]
mod tests {
    use super::extract_package_name;

    #[test]
    fn gets_the_package_name_for_a_require_statement() {
        assert_eq!(extract_package_name(""), None);
        assert_eq!(
            extract_package_name("tape/index.js"),
            Some(String::from("tape"))
        );
        assert_eq!(extract_package_name("tape/"), Some(String::from("tape")));
        assert_eq!(extract_package_name("tape"), Some(String::from("tape")));
        assert_eq!(
            extract_package_name("tape/foo/bar/index.js"),
            Some(String::from("tape"))
        );
        assert_eq!(
            extract_package_name("tape/foo/bar/index"),
            Some(String::from("tape"))
        );
        assert_eq!(
            extract_package_name("tape/foo/bar/"),
            Some(String::from("tape"))
        );
        assert_eq!(
            extract_package_name("tape/foo/bar"),
            Some(String::from("tape"))
        );
        assert_eq!(
            extract_package_name("tape///foo/bar"),
            Some(String::from("tape"))
        );

        assert_eq!(
            extract_package_name("@user/home"),
            Some(String::from("@user/home"))
        );
        assert_eq!(
            extract_package_name("@user/home/"),
            Some(String::from("@user/home"))
        );
        assert_eq!(
            extract_package_name("@user/home/foo.js"),
            Some(String::from("@user/home"))
        );
        assert_eq!(
            extract_package_name("@user//foobar"),
            Some(String::from("@user/foobar"))
        );
        assert_eq!(extract_package_name("@user"), None);
        assert_eq!(extract_package_name("@user/"), None);
        assert_eq!(extract_package_name("@user//"), None);
    }
}
