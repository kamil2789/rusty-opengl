mod shaders;
mod utilities;

use crate::shaders::shader_factory;
use crate::shaders::test_shader_program_compilation;

use rusty_opengl::config::tmp_set_colors;
use rusty_opengl::config::GlfwConfig;
use rusty_opengl::entities::object::Triangle2d;
use rusty_opengl::entities::object::Vertices;

fn main() {
    let mut glfw: GlfwConfig = Default::default();
    let (mut window, _events) = glfw.create_window(800, 600, "learn opengl");
    window.set_current();
    window.load_opengl_func_ptr();

    e2e_test!(test_shader_program_compilation);

    let shader_program = shader_factory("simplest.vert", "simplest.frag");
    let vertices = Vertices::new([-0.1, -0.5, 0.5, -0.5, 0.0, 0.5]);
    let mut triangle = Triangle2d::new(vertices, Some(shader_program));
    triangle.init();

    let shader_program2 = shader_factory("simplest.vert", "simplestGreen.frag");
    let vertices = Vertices::new([-0.9, 0.0, -0.5, 0.5, -0.5, 0.0]);
    let mut triangle2 = Triangle2d::new(vertices, Some(shader_program2));
    triangle2.init();

    while window.is_running_window() {
        //process_events(&mut window, &events);
        tmp_set_colors();

        triangle.draw();
        triangle2.draw();
        window.swap_buffers();
        glfw.poll_events();
    }
}
