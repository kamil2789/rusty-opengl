use gl::types::GLfloat;

use crate::shaders::shader_program::ShaderProgram;

pub struct Vertices {
    vert: [f32; 9],
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

pub struct Triangle2d {
    vao: u32,
    vbo: u32,
    pub vertices: Vertices,
    pub shader: Option<ShaderProgram>,
}

impl Triangle2d {
    #[must_use]
    pub fn new(vertices: Vertices, shader: Option<ShaderProgram>) -> Self {
        Triangle2d {
            vao: 0,
            vbo: 0,
            vertices,
            shader,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.init_array_buffer();
            self.prepare_shader();
        }
    }

    pub fn draw(&self) {
        if self.shader.is_some() {
            if let Some(shader_ref) = self.shader.as_ref() {
                shader_ref.activate();
            }
        }
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

    unsafe fn prepare_shader(&mut self) {
        if self.shader.is_some() {
            self.shader.as_mut().unwrap().compile();
        }
    }
}
