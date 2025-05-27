use frame::Frame;
use macroquad::{color, prelude::*};

use crate::{
  utils::{
    color_ext::ColorExt, diagnostics::Diagnostics, graphics::G, viewport::{View, Viewport, ViewportMode}
  }, AppState
};

pub mod frame;

pub struct Game {
  pub degree: f32,
}

impl Game {
  pub fn new() -> Box<Game> {
    Box::new(Game { degree: 0.0 })
  }

  fn render(&self) {
    let frame = Frame::get();
    clear_background(BLACK);

    let view_rect = Rect::new(-400.0, -300.0, 800., 600.);
    let viewport = Viewport::fit(view_rect, frame.screen_size, 0.0, ViewportMode::FitViewIntoScreen);
    viewport.set_as_camera();
    G::filled_rect(view_rect, color::DARKGRAY.mul(0.5));

    G::circle(Vec2::ZERO, 300., 1.0, color::WHITE);
    
    let to = Vec2::new(300.0 * self.degree.to_radians().cos(), 300.0 * self.degree.to_radians().sin());
    G::line(Vec2::ZERO, to, 1.0, color::RED);
  }
}

impl AppState for Game {
  fn update(&mut self) {
    Diagnostics::update();
    Frame::update();

    self.degree += Frame::get().t * 90.0;

    self.render();

    set_default_camera();
    Diagnostics::render();
  }
}
