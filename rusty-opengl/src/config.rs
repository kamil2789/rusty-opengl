use glfw::Context;
use std::sync::mpsc::Receiver;

pub struct Window {
    window: glfw::Window,
}

pub struct GlfwConfig {
    glfw: glfw::Glfw,
}

#[allow(dead_code)]
pub struct WindowEvents {
    events: Receiver<(f64, glfw::WindowEvent)>,
}

pub fn tmp_set_colors() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

impl GlfwConfig {
    pub fn create_window(
        &self,
        width: u32,
        height: u32,
        window_name: &str,
    ) -> (Window, WindowEvents) {
        let (window, events) = self
            .glfw
            .create_window(width, height, window_name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        (Window { window }, WindowEvents { events })
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}

impl Default for GlfwConfig {
    fn default() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        GlfwConfig { glfw }
    }
}

impl Window {
    pub fn set_current(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
    }

    pub fn load_opengl_func_ptr(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    pub fn is_running_window(&self) -> bool {
        !self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_initial_configuration() {
        let glfw: GlfwConfig = Default::default();
        let (mut window, _events) = glfw.create_window(800, 600, "learn opengl");
        window.set_current();
        window.load_opengl_func_ptr();
    }
}
