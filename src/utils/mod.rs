use macroquad::math::{Rect, Vec2};

pub mod aligned_box;
pub mod color_ext;
pub mod coord;
pub mod diagnostics;
pub mod drawing;
pub mod graphics;
pub mod viewport;

pub use coord::Coord;

pub fn mouse_pos() -> Vec2 {
  let (x, y) = macroquad::input::mouse_position();
  Vec2::new(x, y)
}

pub fn rect_from_corners(p1: Vec2, p2: Vec2) -> Rect {
  let min = Vec2::new(p1.x.min(p2.x), p1.y.min(p2.y));
  let max = Vec2::new(p1.x.max(p2.x), p1.y.max(p2.y));

  Rect::new(min.x, min.y, max.x - min.x, max.y - min.y)
}

pub fn hash_of<T: std::hash::Hash>(value: &T) -> u64 {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};

  let mut hasher = DefaultHasher::new();
  value.hash(&mut hasher);
  hasher.finish()
}

pub fn vec2_to_seed(vec: Vec2) -> u64 {
  let x = vec.x.to_bits() as u64;
  let y = vec.y.to_bits() as u64;
  (x << 32) | y
}

pub fn floor_f32_to_i32(v: f32) -> i32 {
  let as_i32 = v as i32;
  if v < 0.0 { as_i32 - 1 } else { as_i32 }
}

pub fn ceil_f32_to_i32(v: f32) -> i32 {
  let as_i32 = v as i32;
  if v > 0.0 { as_i32 + 1 } else { as_i32 }
}
