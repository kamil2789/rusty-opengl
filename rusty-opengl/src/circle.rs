use crate::color::RGBA;
use crate::config::Resolution;
use crate::shaders::shader_program::ShaderProgram;
use crate::shaders::utils::create_shader_program;
use std::rc::Rc;

pub struct Circle {
    center: (f32, f32),
    radius_width: f32,
    radius_height: f32,
    color: RGBA,
    segments: u16,
    circle_data_buffer: Option<CircleDataBuffer>,
}

struct CircleDataBuffer {
    vao: u32,
    vbo: u32,
    segments: u16,
    shader: ShaderProgram,
}

impl Circle {
    const DEFAULT_SEGMENTS: u16 = 32;

    #[must_use]
    pub fn new(center: (f32, f32), mut radius: f32, color: RGBA, segments: Option<u16>) -> Self {
        let segments_value = segments.unwrap_or(Circle::DEFAULT_SEGMENTS);
        radius = Circle::normalized_radius(radius);

        Circle {
            center,
            radius_width: radius,
            radius_height: radius,
            color,
            segments: segments_value,
            circle_data_buffer: None,
        }
    }

    pub fn draw(&self) {
        if self.circle_data_buffer.is_some() {
            self.circle_data_buffer.as_ref().unwrap().draw();
        }
    }

    pub fn init(&mut self) {
        if self.circle_data_buffer.is_none() {
            self.circle_data_buffer = Some(CircleDataBuffer::new(self.segments));
            let raw_data = self.calculate_raw_data();
            self.circle_data_buffer
                .as_mut()
                .unwrap()
                .init(&raw_data, &self.color);
        }
    }

    pub fn adjust_radius(&mut self, resolution: Rc<Resolution>) {
        if resolution.width == resolution.height {
            return;
        }

        match resolution.width > resolution.height {
            true => {
                self.radius_width =
                    resolution.height as f32 * self.radius_width / resolution.width as f32
            }
            false => {
                self.radius_height =
                    resolution.width as f32 * self.radius_height / resolution.height as f32
            }
        };
    }

    fn calculate_raw_data(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(100);
        result = self.add_color(result);
        result = self.add_center_point(result);
        result = self.add_triangles(result);
        result
    }

    fn add_color(&self, mut buffer: Vec<f32>) -> Vec<f32> {
        buffer.extend_from_slice(&self.color.get_as_normalized_f32());
        buffer
    }

    fn add_center_point(&self, mut buffer: Vec<f32>) -> Vec<f32> {
        let center = self.normalized_center_coordinates();
        buffer.append(&mut vec![center.0, center.1, 0.0]);
        buffer
    }

    fn normalized_center_coordinates(&self) -> (f32, f32) {
        let mut result = self.center;
        if self.center.0 > 1_f32 {
            result.0 = 1_f32;
        }
        if self.center.1 > 1_f32 {
            result.1 = 1_f32;
        }
        if self.center.0 < -1_f32 {
            result.0 = -1_f32;
        }
        if self.center.1 < -1_f32 {
            result.1 = -1_f32;
        }

        result
    }

    fn normalized_radius(radius: f32) -> f32 {
        if radius < 0_f32 {
            return 0_f32;
        } else if radius > 1_f32 {
            return 1_f32;
        }

        radius
    }

    fn add_triangles(&self, mut buffer: Vec<f32>) -> Vec<f32> {
        if self.segments == 0 {
            return buffer;
        }

        let angle = 360_f32 / f32::from(self.segments);
        let mut current_angle = angle;
        for _ in 0..self.segments {
            let x = self.radius_width * f32::sin(f32::to_radians(current_angle));
            let y = self.radius_height * f32::cos(f32::to_radians(current_angle));

            buffer.append(&mut vec![x, y, 0.0]);
            current_angle += angle;
        }
        buffer
    }
}

impl CircleDataBuffer {
    pub fn new(segments: u16) -> Self {
        let shader = create_shader_program("colored_circle.vert", "basic_colored.frag");
        CircleDataBuffer {
            vao: 0,
            vbo: 0,
            shader,
            segments,
        }
    }

    pub fn init(&mut self, data: &[f32], color: &RGBA) {
        self.compile_shader();
        self.generate_buffers();
        self.bind();
        CircleDataBuffer::create_buffer_array(data);
        CircleDataBuffer::set_position_attribute_ptr();
        //TODO ADD AS IMPROVEMENT
        //Do not know why it's now working :( Going to set color via uniform variable
        //CircleDataBuffer::set_color_attribute_ptr();
        self.set_color_via_uniform(color);
        CircleDataBuffer::unbind();
    }

    pub fn draw(&self) {
        unsafe {
            self.shader.activate();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 1, i32::from(self.segments));
            ShaderProgram::deactivate();
        }
    }

    fn compile_shader(&mut self) {
        self.shader.compile();
    }

    fn create_buffer_array(data: &[f32]) {
        let size = data.len() * std::mem::size_of::<f32>();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size.try_into().unwrap(),
                data.as_ptr().cast::<std::ffi::c_void>(),
                gl::STATIC_DRAW,
            );
        }
    }

    fn generate_buffers(&mut self) {
        unsafe {
            if self.vao == 0 || self.vbo == 0 {
                gl::GenVertexArrays(1, &mut self.vao);
                gl::GenBuffers(1, &mut self.vbo);
            }
        }
    }

    fn set_position_attribute_ptr() {
        let offset = 4 * std::mem::size_of::<f32>();
        let stride = (3 * std::mem::size_of::<f32>()) as i32;
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(0);
        }
    }

    #[allow(dead_code)]
    fn set_color_attribute_ptr() {
        unsafe {
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);
        }
    }

    fn set_color_via_uniform(&self, color: &RGBA) -> bool {
        self.shader.set_uniform4f_variable("circleColor", color)
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for CircleDataBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_raw_data_no_segments() {
        let circle = Circle::new(
            (0.5_f32, 0.4_f32),
            0.2_f32,
            RGBA::from_hex(0x00_00_FF_FF),
            Some(0),
        );
        let raw_data = circle.calculate_raw_data();
        let expected = vec![0_f32, 0_f32, 1_f32, 1_f32, 0.5_f32, 0.4_f32, 0_f32];
        assert_eq!(expected, raw_data);
    }

    #[test]
    fn test_calculate_raw_data_six_segments() {
        let circle = Circle::new(
            (0.5_f32, 0.4_f32),
            0.2_f32,
            RGBA::from_hex(0x00_00_FF_FF),
            Some(6),
        );
        let raw_data = circle.calculate_raw_data();
        let triangles = 6;
        let expected_len = triangles * 3 + 7;
        assert_eq!(expected_len, raw_data.len());
    }

    #[test]
    fn test_calculate_raw_data_default_segments() {
        let circle = Circle::new(
            (0.5_f32, 0.4_f32),
            0.2_f32,
            RGBA::from_hex(0x00_00_FF_FF),
            None,
        );
        let raw_data = circle.calculate_raw_data();
        let expected_len = Circle::DEFAULT_SEGMENTS as usize * 3 + 7;
        assert_eq!(expected_len, raw_data.len());
    }
}
