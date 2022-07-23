use crate::entities::Drawable;
use crate::polygons::Vertices;
use gl::types::GLfloat;

pub struct Triangle {
    vao: u32,
    vbo: u32,
    vertices: Vertices,
}

impl Triangle {
    #[must_use]
    pub fn new(vertices: Vertices) -> Self {
        Triangle {
            vao: 0,
            vbo: 0,
            vertices,
        }
    }

    fn init_array_buffer(&mut self) {
        self.generate_buffers();
        self.bind();
        self.buffer_data();
        Triangle::set_attribute_ptr();
        Triangle::unbind();
    }

    fn generate_buffers(&mut self) {
        unsafe {
            if self.vao == 0 || self.vbo == 0 {
                gl::GenVertexArrays(1, &mut self.vao);
                gl::GenBuffers(1, &mut self.vbo);
            }
        }
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

    fn buffer_data(&self) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.sum_capacity() * std::mem::size_of::<GLfloat>())
                    .try_into()
                    .unwrap(),
                self.vertices
                    .create_single_vertices_array()
                    .unwrap()
                    .as_ptr()
                    .cast::<std::ffi::c_void>(),
                gl::STATIC_DRAW,
            );
        }
    }

    fn set_attribute_ptr() {
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<GLfloat>()).try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }
    }

    fn set_position_attribute_ptr(&self) {
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                self.vertices.get_stride().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }
    }

    fn set_color_attribute_ptr(&self) {
        unsafe {
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                self.vertices.get_stride().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
        }
    }

    fn set_texture_attribute_ptr(&self) {
        unsafe {
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                self.vertices.get_stride().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(2);
        }
    }
}

impl Drawable for Triangle {
    fn init(&mut self) {
        self.init_array_buffer();
    }

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn set_vertices(&mut self, vertices: &[f32]) {
        unimplemented!();
    }

    fn recalculate(&mut self) {
        self.bind();
        self.buffer_data();
        Triangle::unbind();
    }
}

impl Drop for Triangle {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
