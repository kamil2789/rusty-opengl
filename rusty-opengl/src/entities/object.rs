use crate::entities::texture::Texture;
use crate::entities::Drawable;
use crate::shaders::shader_program::Color;
use crate::shaders::shader_program::ShaderProgram;

pub struct Object {
    pub polygon: Box<dyn Drawable>,
    pub shader: Option<ShaderProgram>,
    pub texture: Option<Texture>,
}

impl Object {
    #[must_use]
    pub fn new(
        polygon: Box<dyn Drawable>,
        shader: Option<ShaderProgram>,
        texture: Option<Texture>,
    ) -> Self {
        Object {
            polygon,
            shader,
            texture,
        }
    }

    pub fn set_uniform_var(&self, name: &str, color: &Color) {
        if self.shader.is_some() {
            let _result = self
                .shader
                .as_ref()
                .unwrap()
                .set_uniform4f_variable(name, &color);
        }
    }
}

impl Drawable for Object {
    fn init(&mut self) {
        self.polygon.init();
        if self.shader.is_some() {
            self.shader.as_mut().unwrap().compile();
        }

        if self.texture.is_some() {
            self.texture.as_mut().unwrap().init();
        }
    }

    fn draw(&self) {
        if self.shader.is_some() {
            if let Some(shader_ref) = self.shader.as_ref() {
                shader_ref.activate();
            }
        }

        if self.texture.is_some() {
            if let Some(texture_ref) = self.shader.as_ref() {
                texture_ref.activate();
            }
        }

        self.polygon.draw();
    }

    fn set_vertices(&mut self, vertices: &Vec<f32>) {
        self.polygon.set_vertices(&vertices);
    }

    fn recalculate(&mut self) {
        self.polygon.recalculate();
    }
}
