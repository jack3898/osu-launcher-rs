use std::path::Path;

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}
