use macroquad::prelude::{Color, Rect, Vec2};
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines, draw_triangle};
use macroquad::text::{draw_text, get_text_center};

pub struct G;

impl G {
  pub fn point(pos: Vec2, color: Color) {
    draw_triangle(pos, pos, pos, color);
  }

  pub fn circle(pos: Vec2, radius: f32, thickness: f32, color: Color) {
    draw_circle_lines(pos.x, pos.y, radius, thickness, color);
  }

  pub fn filled_circle(pos: Vec2, radius: f32, color: Color) {
    draw_circle(pos.x, pos.y, radius, color);
  }

  pub fn centered_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let center = get_text_center(text, None, font_size as u16, 1.0, 0.);
    draw_text(text, x - center.x, y - center.y, font_size, color);
  }

  pub fn line(p1: Vec2, p2: Vec2, thickness: f32, color: Color) {
    draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color);
  }

  pub fn lines(points: &[Vec2], thickness: f32, color: Color) {
    for i in 0..points.len() {
      let p1 = points[i];
      let p2 = points[(i + 1) % points.len()];
      G::line(p1, p2, thickness, color);
    }
  }

  pub fn filled_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
  }

  pub fn rect_outline(rect: Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
  }
}
