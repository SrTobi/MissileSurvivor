use macroquad::{
  math::{vec2, FloatExt},
  prelude::{set_camera, Camera2D, Rect, Vec2},
};
use rand_distr::num_traits::Zero;
use crate::game::frame::Frame;

#[derive(Clone, Copy, Debug)]
pub struct View {
  pub pos: Vec2,
  pub zoom: f32,
  pub rotation: f32,
}

impl Default for View {
  fn default() -> Self {
    Self {
      pos: Vec2::ZERO,
      zoom: 1.0,
      rotation: 0.0,
    }
  }
}

impl View {
  pub fn new(pos: Vec2, zoom: f32, rotation: f32) -> Self {
    Self { pos, zoom, rotation }
  }

  pub fn at(mut self, pos: Vec2) -> Self {
    self.pos = pos;
    self
  }

  pub fn zoomed(mut self, zoom: f32) -> Self {
    self.zoom = zoom;
    self
  }

  pub fn rotated(mut self, rotation: f32) -> Self {
    self.rotation = rotation;
    self
  }
}

impl Into<Viewport> for View {
  fn into(self) -> Viewport {
    Viewport::make(self)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewportMode {
  FitScreenIntoView,
  FitViewIntoScreen,
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
  center: Vec2,
  screen_size: Vec2,
  view_size: Vec2,
  rotation: f32,
}

impl Viewport {
  pub fn new(view: View, screen_size: Vec2, mode: ViewportMode) -> Self {
    let aspect = screen_size.x / screen_size.y;
    let fit_screen_into_view = mode == ViewportMode::FitScreenIntoView;
    let view_size = if (aspect > 1.) == fit_screen_into_view {
      vec2(aspect, 1.)
    } else {
      vec2(1., 1. / aspect)
    };
    let view_size = view_size / view.zoom;

    Self {
      center: view.pos,
      screen_size,
      view_size,
      rotation: view.rotation,
    }
  }

  pub fn make(view: View) -> Self {
    let frame = Frame::get();
    Self::new(view, frame.screen_size, ViewportMode::FitViewIntoScreen)
  }

  pub fn make_parallax(view: View, z_level: f32) -> Self {
    let zoom = view.zoom;
    let level_zoom = zoom * z_level.powf(zoom);
    Viewport::make(view.zoomed(level_zoom))
  }

  pub fn make_normal(view: View) -> Self {
    let frame = Frame::get();
    Self {
      center: view.pos,
      rotation: view.rotation,
      screen_size: frame.screen_size,
      view_size: frame.screen_size,
    }
  }

  pub fn fit(view_rect: Rect, screen_size: Vec2, rotation: f32, mode: ViewportMode) -> Self {
    let screen_aspect = screen_size.x / screen_size.y;
    let view_aspect = view_rect.w / view_rect.h;
    let fit_view_into_screen = mode == ViewportMode::FitViewIntoScreen;
    let aspect_mul = if (screen_aspect > view_aspect) == fit_view_into_screen {
      vec2(screen_aspect / view_aspect, 1.)
    } else {
      vec2(1., view_aspect / screen_aspect)
    };
    let view_size = aspect_mul * view_rect.size();

    Self {
      center: view_rect.center(),
      rotation,
      screen_size,
      view_size,
    }
  }

  pub fn pos(&self) -> Vec2 {
    self.center
  }

  pub fn rotation(&self) -> f32 {
    self.rotation
  }

  pub fn screen_size(&self) -> Vec2 {
    self.screen_size
  }

  pub fn view_size(&self) -> Vec2 {
    self.view_size
  }

  pub fn screen_rect(&self) -> Rect {
    Rect::new(
      -self.screen_size.x / 2.,
      -self.screen_size.y / 2.,
      self.screen_size.x,
      self.screen_size.y,
    )
  }

  pub fn project_factor(&self) -> f32 {
    self.screen_size.x / self.view_size.x
  }

  pub fn is_in_view(&self, pos: Vec2, radius: f32) -> bool {
    let view_rect = self.view_rect();
    pos.x + radius > view_rect.left()
      && pos.x - radius < view_rect.right()
      && pos.y + radius > view_rect.top()
      && pos.y - radius < view_rect.bottom()
  }

  pub fn f32_to_screen(&self, len: f32) -> f32 {
    len * self.project_factor()
  }

  pub fn f32_to_view(&self, len: f32) -> f32 {
    len / self.project_factor()
  }

  pub fn vec2_to_screen(&self, pos: Vec2) -> Vec2 {
    assert!(self.rotation.is_zero());
    let view_size = self.view_size();
    let rel_pos = (pos - self.center) / (view_size / 2.);
    let half_screen_size = self.screen_size / 2.;
    half_screen_size + rel_pos * half_screen_size
  }

  pub fn vec2_to_view(&self, pos: Vec2) -> Vec2 {
    assert!(self.rotation.is_zero());
    let view_size = self.view_size();
    let half_screen_size = self.screen_size / 2.;
    let rel_pos = (pos - half_screen_size) / half_screen_size;
    self.center + rel_pos * (view_size / 2.)
  }

  pub fn view_rect(&self) -> Rect {
    let view_size: Vec2 = self.view_size();
    let rot = self.rotation.to_radians();
    let sin = rot.sin().abs();
    let cos = rot.cos().abs();
    let view_size = vec2(
      view_size.x * cos + view_size.y * sin,
      view_size.x * sin + view_size.y * cos,
    );
    let view_pos = self.center - view_size / 2.;
    Rect::new(view_pos.x, view_pos.y, view_size.x, view_size.y)
  }

  pub fn set_as_camera(&self) {
    let view_size = self.view_size();

    set_camera(&Camera2D {
      target: self.center,
      zoom: vec2(1. / view_size.x * 2., 1. / view_size.y * 2.),
      offset: vec2(0., 0.),
      rotation: self.rotation,

      render_target: None,
      viewport: None,
    });
  }
}
