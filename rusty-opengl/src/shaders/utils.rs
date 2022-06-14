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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_src_from_file_no_file() {
        let result = read_src_from_file(Path::new("/nonExistedPath"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_src_from_file_exists() {
        let text = "Hello World file reader";
        let file_name = "file_reader_test.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        let write_result = writeln!(file, "{}", text);
        assert!(write_result.is_ok());

        let result = read_src_from_file(Path::new(file_name)).unwrap();
        assert_eq!(result.trim(), text);

        assert!(fs::remove_file(file_name).is_ok());
    }
}
