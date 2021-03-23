use std::error::Error;

pub trait Draw {
    fn draw(&self, aspect_ratio: f32) -> Result<(), Box<dyn Error>>;
}
