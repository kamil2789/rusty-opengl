mod shaders;
mod utilities;

use crate::shaders::test_shader_program_compilation;
use rusty_opengl::config::GlfwConfig;

fn main() {
    let glfw: GlfwConfig = Default::default();
    let (mut window, _events) = glfw.create_window(800, 600, "learn opengl");
    window.set_current();
    window.load_opengl_func_ptr();

    e2e_test!(test_shader_program_compilation);
}
