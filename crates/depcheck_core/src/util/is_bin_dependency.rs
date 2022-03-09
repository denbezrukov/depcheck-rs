use std::path::Path;

use crate::util::load_module::load_module;

pub fn is_bin_dependency(directory: &Path, dependency: &str) -> bool {
    let dependency_module = load_module(&directory.join("node_modules").join(dependency));

    match dependency_module {
        Ok(dependency_module) => dependency_module.bin.is_some(),
        Err(_) => false,
    }
}
