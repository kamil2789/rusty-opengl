pub mod color;
pub mod texture;
pub mod vertices;
mod databuffer;

use crate::polygons::color::RGBA;
use crate::polygons::databuffer::DataBuffer;
use crate::polygons::vertices::{Vertices, VertexLocation};
use crate::shaders::shader_program::ShaderProgram;
use crate::shaders::utils::create_shader_program;
use crate::polygons::texture::Texture;

pub struct Polygon {
    vertices: Vertices,
    shader_program: ShaderProgram,
    data_buffer: DataBuffer,
    texture: Option<Texture>
}

pub struct PolygonBuilder {
    vertices: Vertices,
    color: Option<RGBA>,
    texture: Option<Texture>
}

impl Polygon {
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
            texture: None
        }
    }

    /// # Errors
    pub fn build(&mut self) -> Result<Polygon, String> {
        if self.color.is_some() {
            self.set_same_color_for_all_vertices();
        }

        let shader_program;
        if self.texture.is_some() {
            shader_program = create_shader_program("basic_texture.vert", "basic_texture.frag");
        }
        else {
            shader_program = create_shader_program("basic_colored.vert", "basic_colored.frag");
        }

        let mut result = Polygon {
            vertices: self.vertices.clone(),
            shader_program,
            data_buffer: DataBuffer::new(),
            texture: self.texture.clone()
        };

        if self.texture.is_some() {
            result.texture.as_mut().unwrap().set_options();
            result.texture.as_mut().unwrap().init();
            result.vertices.set_position(&[1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0], VertexLocation::Texture);
        }

        result.data_buffer.init(&result.vertices)?;
        if !result.shader_program.compile() {
            return Err(String::from("Shader program compilation error"));
        }

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

    fn set_same_color_for_all_vertices(&mut self) {
        self.vertices
            .set_one_color_for_all_vert(self.color.as_ref().unwrap());
    }
}

impl Default for PolygonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
