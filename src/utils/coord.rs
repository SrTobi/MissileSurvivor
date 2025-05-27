use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
  pub x: i32,
  pub y: i32,
}

impl Coord {
  pub const ZERO: Self = Self::new(0, 0);
  pub const ONE: Self = Self::new(0, 0);

  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub const fn to_vec2(self) -> Vec2 {
    Vec2::new(self.x as f32, self.y as f32)
  }
}

impl Into<Vec2> for Coord {
  fn into(self) -> Vec2 {
    Vec2::new(self.x as f32, self.y as f32)
  }
}

impl std::ops::Add for Coord {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
impl std::ops::Sub for Coord {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
impl std::ops::Mul for Coord {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
    }
  }
}
impl std::ops::Div for Coord {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    Self {
      x: self.x / other.x,
      y: self.y / other.y,
    }
  }
}

impl std::ops::AddAssign for Coord {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl std::ops::SubAssign for Coord {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}
impl std::ops::MulAssign for Coord {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
  }
}

impl std::ops::DivAssign for Coord {
  fn div_assign(&mut self, other: Self) {
    self.x /= other.x;
    self.y /= other.y;
  }
}

impl std::ops::Mul<i32> for Coord {
  type Output = Self;

  fn mul(self, other: i32) -> Self::Output {
    Self {
      x: self.x * other,
      y: self.y * other,
    }
  }
}

impl std::ops::Div<i32> for Coord {
  type Output = Self;

  fn div(self, other: i32) -> Self::Output {
    Self {
      x: self.x / other,
      y: self.y / other,
    }
  }
}

impl std::ops::AddAssign<i32> for Coord {
  fn add_assign(&mut self, other: i32) {
    self.x += other;
    self.y += other;
  }
}

impl std::ops::SubAssign<i32> for Coord {
  fn sub_assign(&mut self, other: i32) {
    self.x -= other;
    self.y -= other;
  }
}
