use macroquad::math::{Rect, Vec2};

use super::{Coord, ceil_f32_to_i32, floor_f32_to_i32};

#[derive(Clone, Copy, Debug)]
pub struct AlignedBoxIt {
  cur: Vec2,
  cur_cooord: Coord,
  start: Vec2,
  start_coord: Coord,
  end: Vec2,
  box_size: Vec2,
}

impl AlignedBoxIt {
  pub fn new(rect: Rect, box_size: Vec2) -> Self {
    let start_coord = Self::top_left_box(rect.point(), box_size);
    let start = start_coord.to_vec2() * box_size + box_size / 2.;
    let end = Self::bottom_right_box(rect.point() + rect.size(), box_size).to_vec2() * box_size - box_size / 2.;
    let result = Self {
      start,
      cur_cooord: start_coord,
      start_coord,
      cur: start,
      box_size,
      end,
    };
    result
  }

  pub fn top_left_box(pos: Vec2, box_size: Vec2) -> Coord {
    Coord::new(floor_f32_to_i32(pos.x / box_size.x), floor_f32_to_i32(pos.y / box_size.y))
  }

  pub fn bottom_right_box(pos: Vec2, box_size: Vec2) -> Coord {
    Coord::new(ceil_f32_to_i32(pos.x / box_size.x), ceil_f32_to_i32(pos.y / box_size.y))
  }

  pub fn expand(mut self, box_number: u32) -> Self {
    let offset = self.box_size * box_number as f32;
    self.start -= offset;
    self.start_coord -= Coord::ONE * box_number as i32;
    self.end += offset;
    self.cur = self.start;
    self.cur_cooord = self.start_coord;
    self
  }
}

impl Iterator for AlignedBoxIt {
  type Item = (Vec2, Coord);

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur.x > self.end.x {
      self.cur.x = self.start.x;
      self.cur_cooord.x = self.start_coord.x;
      self.cur.y += self.box_size.y;
      self.cur_cooord.y += 1;
      if self.cur.y > self.end.y {
        return None;
      }
    }

    let result = Some((self.cur, self.cur_cooord));
    self.cur.x += self.box_size.x;
    self.cur_cooord.x += 1;
    result
  }
}
