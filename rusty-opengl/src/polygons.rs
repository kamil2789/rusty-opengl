pub mod color;
mod databuffer;
pub mod texture;
pub mod vertices;

use crate::polygons::color::RGBA;
use crate::polygons::databuffer::DataBuffer;
use crate::polygons::vertices::Vertices;
use crate::shaders::shader_program::ShaderProgram;
use crate::shaders::utils::create_shader_program;

pub struct Polygon {
    vertices: Vertices,
    shader_program: ShaderProgram,
    data_buffer: DataBuffer,
}

pub struct PolygonBuilder {
    vertices: Vertices,
    color: Option<RGBA>,
}

impl Polygon {
    //pub fn init(&mut self) {}

    pub fn draw(&self) {
        self.shader_program.activate();
        self.data_buffer.draw();
    }

    // pub fn update(&mut self) {}

    //pub fn set_color(&mut self, _color: &ColorRGBA) {}

    //pub fn transform(&mut self) {}
}

//TODO IMPLEMENT ERROR HANDLING, CHANGE STRING TO ERR STRUCT

impl PolygonBuilder {
    #[must_use]
    pub fn new() -> Self {
        PolygonBuilder {
            vertices: Vertices::empty(),
            color: None,
        }
    }

    /// # Errors
    pub fn build(&mut self) -> Result<Polygon, String> {
        if self.color.is_some() {
            self.set_same_color_for_all_vertices();
        }

        let shader_program = create_shader_program("basic_colored.vert", "basic_colored.frag");
        let mut result = Polygon {
            vertices: self.vertices.clone(),
            shader_program,
            data_buffer: DataBuffer::new(),
        };
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

    /* NOT SUPPORTED
    pub fn set_texture(&mut self, texture: Texture) {
        self.polygon.texture = Some(texture);
    }
    */

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
