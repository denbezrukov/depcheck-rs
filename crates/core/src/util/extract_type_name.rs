use crate::util::is_core_module::is_core_module;
use regex::Regex;

pub fn extract_type_name(path: &str) -> String {
    if is_core_module(path) {
        return "@types/node".to_owned();
    }

    let organization_dependency = Regex::new(r"@(.*?)/(.*)").unwrap();

    let path = match organization_dependency.captures(path) {
        Some(captures) => {
            captures.get(1).unwrap().as_str().to_owned() + "__" + captures.get(2).unwrap().as_str()
        }
        None => path.to_owned(),
    };
    format!("@types/{path}")
}
