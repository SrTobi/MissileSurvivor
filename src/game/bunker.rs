use macroquad::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct Bunker {
  pub pos: Vec2,
  pub active: bool,
  pub firing: bool,
}

impl Bunker {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      active: true,
      firing: false,
    }
  }

  pub fn reset(&mut self) {
    self.active = true;
    self.firing = false;
  }
}