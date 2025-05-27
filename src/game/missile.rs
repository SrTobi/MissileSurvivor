use macroquad::prelude::Vec2;

pub struct Missile {
  pub start_pos: Vec2,
  pub target_pos: Vec2,
  pub current_pos: Vec2,
  pub direction: Vec2,
  pub target_bunker_idx: Option<usize>,
  pub exploded: bool,
}

impl Missile {
  pub fn new(start_pos: Vec2, target_pos: Vec2, target_bunker_idx: Option<usize>) -> Self {
    Self {
      start_pos,
      target_pos,
      current_pos: start_pos,
      direction: (target_pos - start_pos).normalize(),
      target_bunker_idx,
      exploded: false,
    }
  }
}