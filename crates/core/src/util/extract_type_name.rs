use crate::util::is_core_module::is_core_module;
use regex::Regex;

/// Get a DefinitelyTyped dependency for a given dependency.
pub fn extract_type_name(dependency: &str) -> String {
    if is_core_module(dependency) {
        return "@types/node".to_owned();
    }

    let organization_dependency = Regex::new(r"@(.*?)/(.*)").unwrap();

    let path = match organization_dependency.captures(dependency) {
        Some(captures) => {
            captures.get(1).unwrap().as_str().to_owned() + "__" + captures.get(2).unwrap().as_str()
        }
        None => dependency.to_owned(),
    };
    format!("@types/{path}")
}
