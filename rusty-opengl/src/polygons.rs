pub mod color;
mod databuffer;
pub mod texture;
pub mod vertices;

use crate::polygons::color::RGBA;
use crate::polygons::databuffer::DataBuffer;
use crate::polygons::texture::Texture;
use crate::polygons::vertices::{VertexLocation, Vertices};
use crate::shaders::shader_program::ShaderProgram;
use crate::shaders::utils::create_shader_program;

pub struct Polygon {
    vertices: Vertices,
    shader_program: ShaderProgram,
    data_buffer: DataBuffer,
    texture: Option<Texture>,
}

pub struct PolygonBuilder {
    vertices: Vertices,
    color: Option<RGBA>,
    texture: Option<Texture>,
}

impl Polygon {
    /// # Panics
    pub fn draw(&self) {
        if self.texture.is_some() {
            self.texture.as_ref().unwrap().draw();
        }
        self.shader_program.activate();
        self.data_buffer.draw();
    }
}

//TODO IMPLEMENT ERROR HANDLING, CHANGE STRING TO ERR STRUCT

impl PolygonBuilder {
    #[must_use]
    pub fn new() -> Self {
        PolygonBuilder {
            vertices: Vertices::empty(),
            color: None,
            texture: None,
        }
    }

    /// # Errors
    pub fn build(&mut self) -> Result<Polygon, String> {
        self.prepare_vertices();
        let shader_program = self.create_shader();
        let mut result = Polygon {
            vertices: self.vertices.clone(),
            shader_program,
            data_buffer: DataBuffer::new(),
            texture: self.texture.take(),
        };

        PolygonBuilder::init_polygon(&mut result)?;
        Ok(result)
    }

    pub fn set_color(&mut self, color: RGBA) {
        self.color = Some(color);
    }

    pub fn set_vertices(&mut self, vertices: Vertices) {
        self.vertices = vertices;
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = Some(texture);
    }

    fn prepare_vertices(&mut self) {
        if self.color.is_some() {
            self.vertices
                .set_one_color_for_all_vert(self.color.as_ref().unwrap());
        } else {
            self.vertices
                .set_one_color_for_all_vert(&RGBA::from_hex(0xFF_FF_FF_FF));
        }

        if self.texture.is_some() && !self.vertices.is_texture() {
            if self.vertices.is_triangle() {
                self.vertices
                    .set_position(&[1.0, 1.0, 1.0, 0.0, 0.0, 0.0], VertexLocation::Texture);
            } else if self.vertices.is_reactangle() {
                self.vertices.set_position(
                    &[1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                    VertexLocation::Texture,
                );
            }
        }
    }

    fn create_shader(&self) -> ShaderProgram {
        if self.texture.is_some() {
            create_shader_program("basic_texture.vert", "basic_texture.frag")
        } else {
            create_shader_program("basic_colored.vert", "basic_colored.frag")
        }
    }

    fn init_polygon(polygon: &mut Polygon) -> Result<(), String> {
        if polygon.texture.is_some() {
            polygon.texture.as_mut().unwrap().generate_mipmap();
        }

        polygon.data_buffer.init(&polygon.vertices)?;
        if !polygon.shader_program.compile() {
            return Err(String::from("Shader program compilation error"));
        }

        Ok(())
    }
}

impl Default for PolygonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
