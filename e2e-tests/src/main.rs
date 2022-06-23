mod tests;
mod tools;

use crate::tests::shaders::test_shader_program_compilation;
use rusty_opengl::config::GlfwConfig;
use rusty_opengl::entities::object::Triangle2d;
use rusty_opengl::entities::object::Vertices;
use std::process::Command;

fn main() {
    let mut glfw: GlfwConfig = Default::default();
    let (mut window, _events) = glfw.create_window(900, 600, "learn opengl");
    window.set_current();
    window.load_opengl_func_ptr();

    e2e_test!(test_shader_program_compilation);

    //process_events(&mut window, &events);
}
