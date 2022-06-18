mod shaders;

use crate::shaders::test_shader_program_compilation;
use rusty_opengl::config::GlfwConfig;

macro_rules! e2e_test {
    ($func:ident) => {
        let result;
        if $func() {
            result = "PASSED";
        } else {
            result = "FAILED";
        }
        println!("{} - {}", stringify!($func), result);
    };
}

fn main() {
    let glfw: GlfwConfig = Default::default();
    let (mut window, _events) = glfw.create_window(800, 600, "learn opengl");
    window.set_current();
    window.load_opengl_func_ptr();

    e2e_test!(test_shader_program_compilation);
}
