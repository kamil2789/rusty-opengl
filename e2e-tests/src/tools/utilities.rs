use rusty_opengl::shaders::shader_program::ShaderProgram;
use rusty_opengl::shaders::utils::read_src_from_file;
use std::env;
use std::path::Path;

#[cfg(windows)]
static DELIMETER: char = '\\';

#[cfg(unix)]
static DELIMETER: char = '/';

pub fn get_current_dir_name() -> String {
    let full_path = env::current_dir().unwrap();
    let (_, dir) = full_path.to_str().unwrap().rsplit_once(DELIMETER).unwrap();
    String::from(dir)
}

pub fn create_shader_program(vertex_name: &str, fragment_name: &str) -> ShaderProgram {
    let path_vertex_src = get_path_to_shaders() + vertex_name;
    let path_fragment_src = get_path_to_shaders() + fragment_name;

    let vertex_src = read_src_from_file(Path::new(&path_vertex_src)).unwrap();
    let fragment_src = read_src_from_file(Path::new(&path_fragment_src)).unwrap();
    ShaderProgram::new(&vertex_src, &fragment_src)
}

pub fn get_path_to_shaders() -> String {
    if get_current_dir_name() == "e2e-tests" {
        String::from("shaders/")
    } else {
        String::from("e2e-tests/shaders/")
    }
}

pub fn get_path_to_python_scripts() -> String {
    if get_current_dir_name() == "e2e-tests" {
        String::from("src/tools/python_scripts/")
    } else {
        String::from("e2e-tests/src/tools/python_scripts/")
    }
}
