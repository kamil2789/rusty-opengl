use crate::tools::images::check_images_equality;
use crate::tools::utilities::create_shader_program;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::Glfw;
use rusty_opengl::config::Window;
use rusty_opengl::entities::object::Triangle2d;
use rusty_opengl::entities::object::Vertices;
use rusty_opengl::shaders::shader_program::Color;

pub fn test_draw_two_triangles(glfw: &mut Glfw, window: &mut Window) -> bool {
    let right_shader_program = create_shader_program("simplest.vert", "simplest.frag");
    let right_vertices = Vertices::new([-0.1, -0.5, 0.5, -0.5, 0.0, 0.5]);

    let mut right_triangle = Triangle2d::new(right_vertices, Some(right_shader_program));
    right_triangle.init();

    let left_shader_program = create_shader_program("simplest.vert", "simplestGreen.frag");
    let left_vertices = Vertices::new([-0.9, 0.0, -0.5, 0.5, -0.5, 0.0]);
    let mut left_triangle = Triangle2d::new(left_vertices, Some(left_shader_program));
    left_triangle.init();

    set_background_color(0.2, 0.4, 0.6);

    right_triangle.draw();
    left_triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "draw_two_triangles.png";
    check_images_equality(window, image_name)
}

pub fn test_draw_triangle_with_color_from_uniform(glfw: &mut Glfw, window: &mut Window) -> bool {
    let shader_program = create_shader_program("simplest.vert", "uniform.frag");
    let vertices = Vertices::new([-0.1, -0.5, 0.5, -0.5, 0.0, 0.5]);

    let mut triangle = Triangle2d::new(vertices, Some(shader_program));
    triangle.init();

    let set_result = triangle.shader.as_ref().unwrap().set_uniform4f_variable(
        "ourColor",
        &Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    );
    assert!(set_result);

    set_background_color(0.2, 0.4, 0.6);

    triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "red_triangle_uniform.png";
    check_images_equality(window, image_name)
}
