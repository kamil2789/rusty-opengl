use rusty_opengl::shaders::shader_program::ShaderProgram;
use rusty_opengl::shaders::utils::read_src_from_file;
use std::path::Path;

pub fn test_shader_program_compilation() -> bool {
    let vertex_src = read_src_from_file(Path::new("e2e-tests/shaders/simplest.vert")).unwrap();
    let fragment_src = read_src_from_file(Path::new("e2e-tests/shaders/simplest.frag")).unwrap();
    let mut shader_program = ShaderProgram::new(&vertex_src, &fragment_src);

    shader_program.compile();
    shader_program.activate();

    true
}
