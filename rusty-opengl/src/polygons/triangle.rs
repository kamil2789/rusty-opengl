use gl::types::GLfloat;

use crate::entities::Drawable;

pub struct Vertices {
    vert: [f32; 9],
}

pub struct Triangle {
    vao: u32,
    vbo: u32,
    pub vertices: Vertices,
}

impl Vertices {
    const SIZE: usize = 9;

    #[must_use]
    pub fn new(data: [f32; 6]) -> Self {
        Vertices {
            vert: [
                data[0], data[1], 0.0, data[2], data[3], 0.0, data[4], data[5], 0.0,
            ],
        }
    }
}

impl Triangle {
    #[must_use]
    pub fn new(vertices: Vertices) -> Self {
        Triangle {
            vao: 0,
            vbo: 0,
            vertices
        }
    }

    unsafe fn init_array_buffer(&mut self) {
        if self.vao == 0 || self.vbo == 0 {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (Vertices::SIZE * std::mem::size_of::<GLfloat>())
                    .try_into()
                    .unwrap(),
                self.vertices.vert.as_ptr().cast::<std::ffi::c_void>(),
                gl::STATIC_DRAW,
            );
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
            gl::BindVertexArray(0);
        }
    }
}

impl Drawable for Triangle {
    fn init(&mut self) {
        unsafe {
            self.init_array_buffer();
        }
    }

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
