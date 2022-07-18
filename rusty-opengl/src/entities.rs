pub mod object;
pub mod texture;

pub trait Drawable {
    fn init(&mut self);
    fn draw(&self);
}