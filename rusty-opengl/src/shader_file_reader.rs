use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

pub fn read_src_from_file(path: &Path) -> Result<String, String> {
    let mut result = String::new();

    if path.is_file() {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path.to_str().unwrap())
            .unwrap();
        file.read_to_string(&mut result).unwrap();
        return Ok(result);
    }

    Err(format!(
        "File could not be opened, path: {}",
        path.to_str().unwrap()
    ))
}
