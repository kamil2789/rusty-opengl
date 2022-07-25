use gl;
use glfw;
use glfw::Context;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct Window {
    window: glfw::Window,
    resolution: Rc<Resolution>,
}

pub struct Glfw {
    glfw: glfw::Glfw,
}

#[allow(dead_code)]
pub struct WindowEvents {
    events: Receiver<(f64, glfw::WindowEvent)>,
}

pub fn set_background_color(red: f32, green: f32, blue: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

impl Glfw {
    #[must_use]
    pub fn create_window(
        &self,
        resolution: Rc<Resolution>,
        window_name: &str,
    ) -> (Window, WindowEvents) {
        let (window, events) = self
            .glfw
            .create_window(
                resolution.width,
                resolution.height,
                window_name,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");
        (Window { window, resolution }, WindowEvents { events })
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}

impl Default for Glfw {
    fn default() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        Glfw { glfw }
    }
}

impl Window {
    pub fn set_current(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);
        self.window.set_framebuffer_size_polling(true);
    }

    pub fn load_opengl_func_ptr(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol).cast());
    }

    pub fn is_running_window(&self) -> bool {
        !self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn get_resolution(&self) -> Rc<Resolution> {
        self.resolution.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_initial_configuration() {
        let glfw: Glfw = Default::default();
        let resolution = Rc::new(Resolution {
            width: 800,
            height: 600,
        });
        let (mut window, _events) = glfw.create_window(resolution.clone(), "learn opengl");
        window.set_current();
        window.load_opengl_func_ptr();
    }
}
