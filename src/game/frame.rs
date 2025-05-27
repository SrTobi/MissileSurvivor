use macroquad::prelude::Vec2;
use macroquad::time::{get_fps, get_frame_time};
use macroquad::window::{screen_height, screen_width};

use crate::utils::diagnostics::Diagnostics;

static mut FRAME: Frame = Frame::initial();

#[derive(Debug, Clone, Copy, Default)]
pub struct Frame {
  pub t: f32,

  pub screen_size: Vec2,
}

impl Frame {
  const fn initial() -> Self {
    Self {
      t: 0.,
      screen_size: Vec2::new(800., 600.),
    }
  }

  pub fn get() -> Self {
    unsafe { FRAME }
  }

  pub fn update() {
    let mut frame = Self::get();
    frame.t = get_frame_time();
    frame.screen_size = Vec2::new(screen_width(), screen_height());
    unsafe {
      FRAME = frame;
    }

    let diag = Diagnostics::get();
    diag.report_number("frame/time", frame.t * 1000., "ms");
    diag.report_number("frame/fps", get_fps(), "");
  }
}
