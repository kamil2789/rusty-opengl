//mod programs;
mod tests;
mod tools;

//use crate::programs::moving_triangle;
use crate::tests::display::test_draw_reactangle;
//use crate::tests::display::test_draw_triangle_with_color_from_uniform;
use crate::tests::display::test_draw_reactangle_with_texture;
use crate::tests::display::test_draw_red_reactangle_with_texture;
use crate::tests::display::test_draw_triangle_with_texture_only_vertices;
use crate::tests::display::test_draw_two_triangles;
use crate::tests::shaders::test_shader_program_compilation;
use rusty_opengl::config::Glfw;

fn main() {
    let mut glfw: Glfw = Glfw::default();
    let (mut window, _events) = glfw.create_window(900, 600, "learn opengl");
    window.set_current();
    window.load_opengl_func_ptr();

    e2e_test!(test_shader_program_compilation);
    e2e_test!(test_draw_two_triangles &mut glfw, &mut window);
    e2e_test!(test_draw_reactangle &mut glfw, &mut window);
    e2e_test!(test_draw_reactangle_with_texture &mut glfw, &mut window);
    e2e_test!(test_draw_red_reactangle_with_texture &mut glfw, &mut window);
    e2e_test!(test_draw_triangle_with_texture_only_vertices &mut glfw, &mut window);
    //moving_triangle(&mut glfw, &mut window);

    //process_events(&mut window, &events);
}
