use crate::polygons::vertices::Vertices;
use std::ptr;

pub struct DataBuffer {
    vao: u32,
    vbo: u32,
    ebo: u32,
    stride: i32,
}

impl DataBuffer {
    #[must_use]
    pub fn new() -> Self {
        DataBuffer {
            vao: 0,
            vbo: 0,
            ebo: 0,
            stride: 0,
        }
    }

    pub fn init(&mut self, data: &Vertices) -> Result<(), String> {
        self.generate_buffers();
        self.bind();
        DataBuffer::init_buffer(data)?;
        if data.is_reactangle() {
            self.generate_ebo_buffer();
            self.set_attribute_element_array_buffer();
        }

        self.set_attribute_ptr(data.get_stride());
        DataBuffer::unbind();

        Ok(())
    }

    fn generate_buffers(&mut self) {
        unsafe {
            if self.vao == 0 || self.vbo == 0 {
                gl::GenVertexArrays(1, &mut self.vao);
                gl::GenBuffers(1, &mut self.vbo);
            }
        }
    }

    fn generate_ebo_buffer(&mut self) {
        unsafe {
            if self.ebo == 0 {
                gl::GenBuffers(1, &mut self.ebo);
            }
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    fn init_buffer(vertices: &Vertices) -> Result<(), String> {
        let size = vertices.sum_capacity() * std::mem::size_of::<f32>();
        let raw_data = vertices.create_single_vertices_array();
        if raw_data.is_none() {
            return Err(String::from("Invalid vertices structure"));
        }

        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size.try_into().unwrap(),
                raw_data.unwrap().as_ptr().cast::<std::ffi::c_void>(),
                gl::STATIC_DRAW,
            );
        }

        Ok(())
    }

    fn set_attribute_ptr(&mut self, stride: i32) {
        self.stride = stride;
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            let ptr = 3 * std::mem::size_of::<f32>();
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                ptr as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(1);

            if stride == 9 {
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (6 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
                );
                gl::EnableVertexAttribArray(2);
            }
        }
    }

    fn set_attribute_element_array_buffer(&mut self) {
        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER as u32, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER as u32,
                (indices.len() * std::mem::size_of::<f32>())
                    .try_into()
                    .unwrap(),
                indices.as_ptr().cast::<std::ffi::c_void>(),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if self.ebo > 0 {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER as u32, self.ebo);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }

    fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for DataBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
