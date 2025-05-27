use macroquad::prelude::Vec2;
use crate::game::constants::{MISSILE_SPEED, get_enemy_missile_speed};

pub struct Missile {
  pub start_pos: Vec2,
  pub target_pos: Vec2,
  pub current_pos: Vec2,
  pub direction: Vec2,
  pub target_bunker_idx: Option<usize>,
  pub exploded: bool,
  pub speed: f32,
}

impl Missile {
  pub fn new(start_pos: Vec2, target_pos: Vec2, target_bunker_idx: Option<usize>, speed: f32) -> Self {
    Self {
      start_pos,
      target_pos,
      current_pos: start_pos,
      direction: (target_pos - start_pos).normalize(),
      target_bunker_idx,
      exploded: false,
      speed,
    }
  }
  
  pub fn is_player_missile(&self) -> bool {
    self.target_bunker_idx.is_none()
  }
}
