use crate::tools::images::save_screen_as_img_png;
use crate::tools::utilities::create_shader_program;
use crate::tools::utilities::get_path_to_python_scripts;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::Glfw;
use rusty_opengl::config::Window;
use rusty_opengl::entities::object::Triangle2d;
use rusty_opengl::entities::object::Vertices;
use std::process::Command;

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
    save_screen_as_img_png(window, &(String::from("result_") + image_name));

    let output = Command::new("python")
        .arg(get_path_to_python_scripts() + "compare_images.py")
        .arg(image_name)
        .output()
        .expect("python command failed to start");

    if !output.status.success() {
        println!("ERROR: {}", String::from_utf8_lossy(&output.stderr));
    }

    output.status.success()
}
