pub mod vertices;
pub mod texture;
pub mod color;
mod databuffer;

use crate::polygons::texture::Texture;
use crate::shaders::shader_program::ShaderProgram;
use crate::polygons::color::ColorRGBA;
use crate::polygons::vertices::Vertices;
use crate::polygons::databuffer::DataBuffer;
use crate::shaders::utils::create_shader_program;
use crate::polygons::vertices::VertexLocation;

pub struct Polygon {
    vertices: Vertices,
    shader_program: ShaderProgram,
    data_buffer: DataBuffer,
}

pub struct PolygonBuilder {
    vertices: Vertices,
    color: Option<ColorRGBA>
}

impl Polygon {
    pub fn init(&mut self) {

    }

    pub fn draw(&self) {
        self.shader_program.activate();
        self.data_buffer.draw();
    }

    pub fn update(&mut self) {

    }

    pub fn set_color(&mut self, color: &ColorRGBA) {

    }

    pub fn transform(&mut self) {

    }
}

//TODO IMPLEMENT ERROR HANDLING, CHANGE STRING TO ERR STRUCT
impl PolygonBuilder {

    pub fn new() -> Self {
        PolygonBuilder{vertices: Vertices::empty(), color: None}
    }

    pub fn build(&mut self) -> Result<Polygon, String> {
        if self.color.is_some() {
            self.set_same_color_for_all_vertices();
        }

        let shader_program = create_shader_program("basic_colored.vert", "basic_colored.frag");
        let mut result = Polygon{vertices: self.vertices.clone(), shader_program, data_buffer: DataBuffer::new()};
        result.data_buffer.init(&result.vertices)?;
        if result.shader_program.compile() == false {
            return Err(String::from("Shader program compilation error"));
        }

        Ok(result)
    }

    pub fn set_color(&mut self, color: ColorRGBA) {
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
        self.vertices.set_one_color_for_all_vert(self.color.as_ref().unwrap());
    }
}