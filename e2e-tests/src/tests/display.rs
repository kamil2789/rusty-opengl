use crate::tools::images::check_images_equality;
use crate::tools::utilities::create_shader_program;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::Glfw;
use rusty_opengl::config::Window;
use rusty_opengl::entities::object::Object;
use rusty_opengl::entities::Drawable;
use rusty_opengl::polygons::reactangle;
use rusty_opengl::polygons::reactangle::Reactangle;
use rusty_opengl::polygons::triangle;
use rusty_opengl::polygons::triangle::Triangle;
use rusty_opengl::shaders::shader_program::Color;

pub fn test_draw_two_triangles(glfw: &mut Glfw, window: &mut Window) -> bool {
    let right_shader_program = create_shader_program("simplest.vert", "simplest.frag");
    let right_vertices = triangle::Vertices::new([-0.1, -0.5, 0.5, -0.5, 0.0, 0.5]);

    let right_triangle = Triangle::new(right_vertices);
    let mut orange_triangle =
        Object::new(Box::new(right_triangle), Some(right_shader_program), None);

    let left_shader_program = create_shader_program("simplest.vert", "simplestGreen.frag");
    let left_vertices = triangle::Vertices::new([-0.9, 0.0, -0.5, 0.5, -0.5, 0.0]);
    let left_triangle = Triangle::new(left_vertices);

    let mut green_triangle = Object::new(Box::new(left_triangle), Some(left_shader_program), None);

    orange_triangle.init();
    green_triangle.init();

    set_background_color(0.2, 0.4, 0.6);

    orange_triangle.draw();
    green_triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "draw_two_triangles.png";
    check_images_equality(window, image_name)
}

pub fn test_draw_triangle_with_color_from_uniform(glfw: &mut Glfw, window: &mut Window) -> bool {
    let shader_program = create_shader_program("simplest.vert", "uniform.frag");
    let vertices = triangle::Vertices::new([-0.1, -0.5, 0.5, -0.5, 0.0, 0.5]);

    let triangle = Triangle::new(vertices);
    let mut red_triangle = Object::new(Box::new(triangle), Some(shader_program), None);
    red_triangle.init();

    let set_result = red_triangle
        .shader
        .as_ref()
        .unwrap()
        .set_uniform4f_variable(
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

    red_triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "red_triangle_uniform.png";
    check_images_equality(window, image_name)
}

pub fn test_draw_reactangle(glfw: &mut Glfw, window: &mut Window) -> bool {
    let shader_program = create_shader_program("simplest.vert", "uniform.frag");
    let vertices = reactangle::Vertices::new([0.5, 0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5]);

    let reactangle = Reactangle::new(vertices);
    let mut blue_reactangle = Object::new(Box::new(reactangle), Some(shader_program), None);
    blue_reactangle.init();

    let set_result = blue_reactangle
        .shader
        .as_ref()
        .unwrap()
        .set_uniform4f_variable(
            "ourColor",
            &Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
        );
    assert!(set_result);

    set_background_color(0.1, 0.2, 0.2);

    blue_reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "blue_reactangle.png";
    check_images_equality(window, image_name)
}
