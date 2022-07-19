use gl::types::GLfloat;

use crate::entities::Drawable;

pub struct Vertices {
    vert: [f32; 12],
}

pub struct Reactangle {
    ebo: u32,
    vao: u32,
    vbo: u32,
    vertices: Vertices,
}

impl Vertices {
    const SIZE: usize = 12;

    #[must_use]
    pub fn new(data: [f32; 8]) -> Self {
        Vertices {
            vert: [
                data[0], data[1], 0.0, data[2], data[3], 0.0, data[4], data[5], 0.0, data[6],
                data[7], 0.0,
            ],
        }
    }
}

impl Reactangle {
    #[must_use]
    pub fn new(vertices: Vertices) -> Self {
        Reactangle {
            ebo: 0,
            vao: 0,
            vbo: 0,
            vertices,
        }
    }

    unsafe fn init_array_buffer(&mut self) {
        if self.vao == 0 || self.vbo == 0 || self.ebo == 0 {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);
            gl::BindVertexArray(self.vao);

            self.set_array_buffer();
            self.set_element_array_buffer();

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<GLfloat>()).try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    unsafe fn set_array_buffer(&mut self) {
        gl::BindBuffer(gl::ARRAY_BUFFER as u32, self.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER as u32,
            (Vertices::SIZE * std::mem::size_of::<GLfloat>())
                .try_into()
                .unwrap(),
            self.vertices.vert.as_ptr().cast::<std::ffi::c_void>(),
            gl::STATIC_DRAW,
        );
    }

    unsafe fn set_element_array_buffer(&mut self) {
        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER as u32, self.ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER as u32,
            (indices.len() * std::mem::size_of::<GLfloat>())
                .try_into()
                .unwrap(),
            indices.as_ptr().cast::<std::ffi::c_void>(),
            gl::STATIC_DRAW,
        );
    }
}

impl Drawable for Reactangle {
    fn init(&mut self) {
        unsafe {
            self.init_array_buffer();
        }
    }

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER as u32, self.ebo);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    fn set_vertices(&mut self, vertices: &Vec<f32>) {
        if vertices.len() >= 2 {
            self.vertices.vert[0] = vertices[0];
            self.vertices.vert[1] = vertices[1];
        }

        if vertices.len() >= 4 {
            self.vertices.vert[2] = vertices[2];
            self.vertices.vert[3] = vertices[3];
        }

        if vertices.len() >= 6 {
            self.vertices.vert[4] = vertices[4];
            self.vertices.vert[5] = vertices[5];
        }

        if vertices.len() >= 8 {
            self.vertices.vert[6] = vertices[6];
            self.vertices.vert[7] = vertices[7];
        }
    }

    fn recalculate(&mut self) {}
}

impl Drop for Reactangle {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
