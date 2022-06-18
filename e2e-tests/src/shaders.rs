use crate::utilities::get_current_dir_name;
use rusty_opengl::shaders::shader_program::ShaderProgram;
use rusty_opengl::shaders::utils::read_src_from_file;
use std::path::Path;

pub fn test_shader_program_compilation() -> bool {
    let path_vertex_src = get_path_to_shaders() + "simplest.vert";
    let path_fragment_src = get_path_to_shaders() + "simplest.frag";

    let vertex_src = read_src_from_file(Path::new(&path_vertex_src)).unwrap();
    let fragment_src = read_src_from_file(Path::new(&path_fragment_src)).unwrap();
    let mut shader_program = ShaderProgram::new(&vertex_src, &fragment_src);

    shader_program.compile();
    shader_program.activate();

    true
}

fn get_path_to_shaders() -> String {
    if get_current_dir_name() == "e2e-tests" {
        String::from("shaders/")
    } else {
        String::from("e2e-tests/shaders/")
    }
}
