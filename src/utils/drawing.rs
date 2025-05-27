use macroquad::{
  color::Color,
  math::Vec2,
  text::{draw_text, measure_text},
};

pub fn draw_centered_text(text: &str, pos: Vec2, fontsize: f32, color: Color) {
  let text_size = measure_text(text, None, fontsize as _, 1.0);
  draw_text(text, pos.x - text_size.width / 2., pos.y - text_size.height / 2., fontsize, color);
}
