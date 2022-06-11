use glfw;
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

impl GlfwConfig {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        GlfwConfig { glfw }
    }

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
}

impl Window {
    pub fn set_current(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_initial_configuration() {
        let glfw = GlfwConfig::new();
        let (_window, _events) = glfw.create_window(800, 600, "learn opengl");
    }
}
