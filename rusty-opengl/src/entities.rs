pub mod object;
pub mod texture;

pub trait Drawable {
    fn init(&mut self);
    fn draw(&self);
    fn set_vertices(&mut self, vertices: &Vec<f32>);
    fn recalculate(&mut self);
}
