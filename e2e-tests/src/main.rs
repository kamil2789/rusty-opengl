use rusty_opengl::shader_file_reader::read_src_from_file;
use std::path::Path;

fn main() {
    let result = read_src_from_file(Path::new("e2e-tests/shaders/simplest.frag"));
    if result.is_ok() {
        println!("{}", result.unwrap());
    } else {
        println!("{}", result.unwrap_err());
    }
}
