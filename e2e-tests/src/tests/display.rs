use crate::tools::images::save_screen_as_img_png;
use crate::tools::utilities::create_shader_program;
use rusty_opengl::config::set_background_color;
use rusty_opengl::config::GlfwConfig;
use rusty_opengl::config::Window;
use rusty_opengl::entities::object::Triangle2d;
use rusty_opengl::entities::object::Vertices;

pub fn test_draw_two_triangles(glfw: &mut GlfwConfig, window: &mut Window) -> bool {
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

    save_screen_as_img_png(window, "result_draw_two_triangles.png");

    // //flip image

    // let output = Command::new("python")
    //     .arg("./e2e-tests/src/prototype.py")
    //     .output()
    //     .expect("ls command failed to start");

    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // assert!(output.status.success());

    true
}
