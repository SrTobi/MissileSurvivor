use macroquad::prelude::Vec2;
use crate::game::constants::EXPLOSION_MAX_RADIUS;

pub struct Explosion {
  pub pos: Vec2,
  pub radius: f32,
  pub max_radius: f32,
}

impl Explosion {
  pub fn new(pos: Vec2) -> Explosion {
    Explosion {
      pos,
      radius: 0.0,
      max_radius: EXPLOSION_MAX_RADIUS,
    }
  }
}