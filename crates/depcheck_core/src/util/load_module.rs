use crate::package::{self, Package};
use std::path::Path;

pub fn load_module(path: &Path) -> package::Result<Package> {
    let package_path = path.join("package.json");
    Package::from_path(package_path)
}
