use crate::tools::utilities::create_shader_program;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::Glfw;
use rusty_opengl::config::Window;
use rusty_opengl::entities::object::Object;
use rusty_opengl::entities::Drawable;
use rusty_opengl::polygons::triangle::Triangle;
use rusty_opengl::polygons::triangle::Vertices;
use rusty_opengl::shaders::shader_program::Color;

pub fn moving_triangle(glfw: &mut Glfw, window: &mut Window) {
    let shader_program = create_shader_program("simplest.vert", "uniform.frag");
    let vertices = Vertices::new([-0.1, -0.3, 0.5, -0.5, 0.0, 0.2]);
    let triangle = Triangle::new(vertices);
    let mut red_triangle = Object::new(Box::new(triangle), Some(shader_program), None);
    red_triangle.init();

    red_triangle.set_uniform_var(
        "ourColor",
        &Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    );

    let mut vertices_in_move = vec![-0.3, -0.3, 0.5, -0.5, 0.0, 0.0];
    let mut is_rising = false;
    while window.is_running_window() {
        set_background_color(0.2, 0.4, 0.6);
        red_triangle.set_vertices(&vertices_in_move);
        red_triangle.recalculate();
        red_triangle.draw();
        window.swap_buffers();
        glfw.poll_events();

        if is_rising {
            vertices_in_move[0] += 0.01;
        } else {
            vertices_in_move[0] -= 0.01;
        }

        if vertices_in_move[0] > 0.99 {
            is_rising = false;
        } else if vertices_in_move[0] < -0.99 {
            is_rising = true;
        }
    }
}
