use crate::tools::images::check_images_equality;
use rusty_opengl::config::{set_background_color, Glfw, Window};
use rusty_opengl::polygons::color::RGBA;
use rusty_opengl::polygons::texture::{Filtering, Texture, Wrapping};
use rusty_opengl::polygons::vertices::Vertices;
use rusty_opengl::polygons::PolygonBuilder;
use std::path::Path;

pub fn test_draw_two_triangles(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(
        vec![-0.1, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
        vec![],
        vec![],
    );
    pol_builder.set_vertices(vertices);
    pol_builder.set_color(RGBA::from_hex(0xFF_A5_00_FF));
    let orange_triangle = pol_builder.build().unwrap();

    let vertices_second = Vertices::new(
        vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0],
        vec![],
        vec![],
    );
    pol_builder.set_vertices(vertices_second);
    pol_builder.set_color(RGBA::from_hex(0x00_FF_00_FF));

    let green_triangle = pol_builder.build().unwrap();

    set_background_color(0.2, 0.4, 0.6);
    orange_triangle.draw();
    green_triangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    let image_name = "draw_two_triangles.png";
    check_images_equality(window, image_name)
}

pub fn test_draw_reactangle(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(
        vec![
            0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
        ],
        vec![],
        vec![],
    );
    pol_builder.set_vertices(vertices);
    pol_builder.set_color(RGBA::from_hex(0x00_00_FF_FF));
    let reactangle = pol_builder.build().unwrap();

    set_background_color(0.1, 0.2, 0.2);

    reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    check_images_equality(window, "blue_reactangle.png")
}

pub fn test_draw_reactangle_with_texture(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(
        vec![
            0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
        ],
        vec![],
        vec![],
    );

    let mut texture = Texture::new(Path::new("e2e-tests/assets/texture/container.jpg"));
    texture.set_default();
    pol_builder.set_vertices(vertices);
    pol_builder.set_texture(texture);
    let reactangle = pol_builder.build().unwrap();

    set_background_color(0.1, 0.2, 0.2);

    reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    check_images_equality(window, "reactangle_with_texture.png")
}

pub fn test_draw_red_reactangle_with_texture(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(
        vec![
            0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
        ],
        vec![],
        vec![],
    );

    let texture = Texture::new(Path::new("e2e-tests/assets/texture/container.jpg"));
    texture.set_filtering(Filtering::Nearest);
    texture.set_wrapping(Wrapping::Repeat);
    pol_builder.set_vertices(vertices);
    pol_builder.set_color(RGBA::from_hex(0xFF_00_00_FF));
    pol_builder.set_texture(texture);
    let reactangle = pol_builder.build().unwrap();

    set_background_color(0.1, 0.2, 0.2);

    reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    check_images_equality(window, "red_reactangle_with_texture.png")
}

pub fn test_draw_triangle_with_texture_only_vertices(glfw: &mut Glfw, window: &mut Window) -> bool {
    let mut pol_builder = PolygonBuilder::new();
    let vertices = Vertices::new(
        vec![0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0],
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    );

    let texture = Texture::new(Path::new("e2e-tests/assets/texture/container.jpg"));
    texture.set_filtering(Filtering::Nearest);
    texture.set_wrapping(Wrapping::Repeat);
    pol_builder.set_vertices(vertices);
    pol_builder.set_texture(texture);
    let reactangle = pol_builder.build().unwrap();

    set_background_color(0.1, 0.2, 0.2);

    reactangle.draw();
    window.swap_buffers();
    glfw.poll_events();

    check_images_equality(window, "triangle_with_texture_only_vertices.png")
}
