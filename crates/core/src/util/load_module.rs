use crate::package::Package;
use std::path::Path;

pub fn load_module(path: &Path) -> eyre::Result<Package> {
    let package_path = path.join("package.json");
    Package::from_path(package_path)
}
