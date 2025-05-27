use extend::ext;
use macroquad::color;

#[ext(name = ColorExt)]
pub impl color::Color {
  fn mul(&self, other: f32) -> color::Color {
    color::Color {
      r: self.r * other,
      g: self.g * other,
      b: self.b * other,
      a: self.a,
    }
  }

  fn with_alpha(&self, alpha: f32) -> color::Color {
    color::Color {
      r: self.r,
      g: self.g,
      b: self.b,
      a: alpha,
    }
  }
}
