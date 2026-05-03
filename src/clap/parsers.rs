use std::path::PathBuf;

pub fn path_parser(path: &str) -> Result<PathBuf, String> {
    match shellexpand::full(path) {
        Ok(path) => {
            let path = PathBuf::from(&*path);
            Ok(path.canonicalize().unwrap_or(path))
        }
        Err(err) => Err(err.to_string()),
    }
}
