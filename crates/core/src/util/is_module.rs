use std::path::Path;

pub fn is_module(path: &Path) -> bool {
    let mut path = path.to_path_buf();
    path.push("package.json");
    path.is_file()
}
