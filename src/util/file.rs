use std::path::Path;

pub fn path_exists(path: &str) -> bool {
    if path.trim() == "" {
        return false;
    }

    Path::new(path).exists()
}

pub fn file_exists(path: &str) -> bool {
    if path.trim() == "" {
        return false;
    }

    Path::new(path).is_file()
}
