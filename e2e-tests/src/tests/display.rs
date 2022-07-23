use crate::tools::images::check_images_equality;
use crate::tools::utilities::create_shader_program;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::Glfw;
use rusty_opengl::config::Window;
use rusty_opengl::polygons::PolygonBuilder;
use rusty_opengl::polygons::vertices::Vertices;
use rusty_opengl::polygons::color::ColorRGBA;

pub fn test_draw_two_triangles(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(vec![-0.1, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0], vec![], vec![]);
    pol_builder.set_vertices(vertices);
    pol_builder.set_color(ColorRGBA::from_hex(0xFFA500FF));
    let orange_triangle = pol_builder.build().unwrap();

    let vertices_second = Vertices::new(vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0], vec![], vec![]);
    pol_builder.set_vertices(vertices_second);
    pol_builder.set_color(ColorRGBA::from_hex(0x00FF00FF));

    let green_triangle = pol_builder.build().unwrap();

    set_background_color(0.2, 0.4, 0.6);
    orange_triangle.draw();
    green_triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "draw_two_triangles.png";
    check_images_equality(window, image_name)
}

/* NOT SUPPORTED uniform variable
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

    red_triangle.render();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "red_triangle_uniform.png";
    check_images_equality(window, image_name)
}
*/

pub fn test_draw_reactangle(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(vec![0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0], vec![], vec![]);
    pol_builder.set_vertices(vertices);
    pol_builder.set_color(ColorRGBA::from_hex(0x0000FFFF));
    let reactangle = pol_builder.build().unwrap();

    set_background_color(0.1, 0.2, 0.2);

    reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "blue_reactangle.png";
    check_images_equality(window, image_name)
}

/*
pub fn test_draw_reactangle_with_texture(glfw: &mut Glfw, window: &mut Window) -> bool {
    let shader_program = create_shader_program("simpleTexture.vert", "simpleTexture.frag");
    let vertices = reactangle::Vertices::new([0.5, 0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5]);

    let reactangle = Reactangle::new(vertices);
    let mut blue_reactangle = Object::new(Box::new(reactangle), Some(shader_program), None);
    blue_reactangle.init();

    set_background_color(0.1, 0.2, 0.2);

    blue_reactangle.render();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "blue_reactangle.png";
    check_images_equality(window, image_name)
}
*/