use crate::utils::mouse_pos;
use crate::{
  AppState,
  utils::{
    color_ext::ColorExt,
    diagnostics::Diagnostics,
    graphics::G,
    viewport::{View, Viewport, ViewportMode},
  },
};
use ::rand::Rng;
use ::rand::rng;
use frame::Frame;
use macroquad::{color, prelude::*};
use rand_distr::Exp;
use std::{process, time::Duration};

pub mod frame;

const GROUND_HEIGHT: f32 = 20.0;
const BUNKER_WIDTH: f32 = 30.0;
const BUNKER_HEIGHT: f32 = 20.0;
const MISSILE_SPEED: f32 = 150.0;
const EXPLOSION_MAX_RADIUS: f32 = 50.0;
const EXPLOSION_GROWTH_RATE: f32 = 200.0;
const ENEMY_MISSILE_SPAWN_INTERVAL: f32 = 1.0;

#[derive(Clone, Copy)]
struct Bunker {
  pos: Vec2,
  active: bool,
  firing: bool,
}

struct PlayerMissile {
  start_pos: Vec2,
  target_pos: Vec2,
  current_pos: Vec2,
  direction: Vec2,
  exploded: bool,
}

struct EnemyMissile {
  start_pos: Vec2,
  target_pos: Vec2,
  current_pos: Vec2,
  direction: Vec2,
  target_bunker_idx: usize,
  exploded: bool,
}

struct Explosion {
  pos: Vec2,
  radius: f32,
  max_radius: f32,
}

impl Explosion {
  fn new(pos: Vec2) -> Explosion {
    Explosion {
      pos,
      radius: 0.0,
      max_radius: EXPLOSION_MAX_RADIUS,
    }
  }
}

pub struct Game {
  bunkers: Vec<Bunker>,
  player_missiles: Vec<PlayerMissile>,
  enemy_missiles: Vec<EnemyMissile>,
  explosions: Vec<Explosion>,
  time_since_last_spawn: f32,
  game_over: bool,
  view_rect: Rect,
  viewport: Viewport,
}

impl Game {
  pub fn new() -> Box<Game> {
    // Create three bunkers at the bottom of the screen
    let bunkers = vec![
      Bunker {
        pos: Vec2::new(-200.0, 280.0 - BUNKER_HEIGHT),
        active: true,
        firing: false,
      },
      Bunker {
        pos: Vec2::new(0.0, 280.0 - BUNKER_HEIGHT),
        active: true,
        firing: false,
      },
      Bunker {
        pos: Vec2::new(200.0, 280.0 - BUNKER_HEIGHT),
        active: true,
        firing: false,
      },
    ];

    let (viewport, view_rect) = Self::create_viewport();

    Box::new(Game {
      bunkers,
      player_missiles: Vec::new(),
      enemy_missiles: Vec::new(),
      explosions: Vec::new(),
      time_since_last_spawn: 0.0,
      game_over: false,
      view_rect,
      viewport,
    })
  }

  fn reset(&mut self) {
    // Reset bunkers
    for bunker in &mut self.bunkers {
      bunker.active = true;
      bunker.firing = false;
    }

    // Clear missiles and explosions
    self.player_missiles.clear();
    self.enemy_missiles.clear();
    self.explosions.clear();

    // Reset timers and flags
    self.time_since_last_spawn = 0.0;
    self.game_over = false;
  }

  fn create_viewport() -> (Viewport, Rect) {
    let view_rect = Rect::new(-400.0, -300.0, 800., 600.);
    let screen_size = Frame::get().screen_size;
    let viewport = Viewport::fit(view_rect, screen_size, 0.0, ViewportMode::FitViewIntoScreen);
    (viewport, view_rect)
  }

  fn find_closest_active_bunker(&self, target_pos: Vec2) -> Option<usize> {
    let mut closest_idx = None;
    let mut closest_dist = f32::MAX;

    for (idx, bunker) in self.bunkers.iter().enumerate() {
      if bunker.active && !bunker.firing {
        let dist = bunker.pos.distance(target_pos);
        if dist < closest_dist {
          closest_dist = dist;
          closest_idx = Some(idx);
        }
      }
    }

    closest_idx
  }

  fn spawn_enemy_missile(&mut self) {
    let mut rng = rng();

    // Random position at the top of the screen
    let start_x = rng.random_range(-380.0..380.0);
    let start_pos = Vec2::new(start_x, -280.0);

    // Target one of the bunkers
    let active_bunkers: Vec<(usize, &Bunker)> = self.bunkers.iter().enumerate().filter(|(_, b)| b.active).collect();

    if !active_bunkers.is_empty() {
      let random_idx = rng.random_range(0..active_bunkers.len());
      let (target_bunker_idx, target_bunker) = active_bunkers[random_idx];
      let target_pos = target_bunker.pos;

      // Calculate direction vector
      let direction = (target_pos - start_pos).normalize();

      self.enemy_missiles.push(EnemyMissile {
        start_pos,
        target_pos,
        current_pos: start_pos,
        direction,
        target_bunker_idx,
        exploded: false,
      });
    }
  }

  fn update_missiles(&mut self, dt: f32) {
    let mut new_explosions = Vec::new();

    // Update player missiles
    for missile in &mut self.player_missiles {
      if !missile.exploded {
        // Use stored direction
        missile.current_pos += missile.direction * MISSILE_SPEED * dt;

        // Check if missile reached target
        if missile.current_pos.distance(missile.target_pos) < 5.0 {
          missile.exploded = true;
          new_explosions.push(Explosion::new(missile.current_pos));
        }
      }
    }

    // Update enemy missiles
    for missile in &mut self.enemy_missiles {
      if !missile.exploded {
        // Use stored direction
        missile.current_pos += missile.direction * MISSILE_SPEED * dt;

        // Check if missile hit its target bunker
        if missile.target_bunker_idx < self.bunkers.len() {
          let bunker = &mut self.bunkers[missile.target_bunker_idx];

          if missile.current_pos.distance(bunker.pos) < 2.0 {
            missile.exploded = true;
            bunker.active = false;
            new_explosions.push(Explosion::new(bunker.pos));
          }
        }
      }
    }

    // Add all new explosions
    self.explosions.append(&mut new_explosions);
  }

  fn update_explosions(&mut self, dt: f32) {
    // Collect positions for new explosions
    let mut new_explosions = Vec::new();

    // Update existing explosions
    for explosion in &mut self.explosions {
      explosion.radius += EXPLOSION_GROWTH_RATE * dt;
    }

    // Check for chain reactions with missiles
    for explosion in &self.explosions {
      // Check player missiles
      for missile in &mut self.player_missiles {
        if !missile.exploded && missile.current_pos.distance(explosion.pos) <= explosion.radius {
          missile.exploded = true;
          new_explosions.push(Explosion::new(missile.current_pos));
        }
      }

      // Check enemy missiles
      for missile in &mut self.enemy_missiles {
        if !missile.exploded && missile.current_pos.distance(explosion.pos) <= explosion.radius {
          missile.exploded = true;
          new_explosions.push(Explosion::new(missile.current_pos));
        }
      }
    }

    // Add all new explosions
    self.explosions.append(&mut new_explosions);

    // Remove finished explosions
    self.explosions.retain(|e| e.radius < e.max_radius);

    // Remove exploded missiles
    self.player_missiles.retain(|m| !m.exploded);
    self.enemy_missiles.retain(|m| !m.exploded);

    // Update bunker firing status
    for bunker in &mut self.bunkers {
      if bunker.firing {
        let still_firing = self
          .player_missiles
          .iter()
          .any(|m| !m.exploded && m.start_pos.distance(bunker.pos) < 5.0);
        bunker.firing = still_firing;
      }
    }
  }

  fn check_game_over(&mut self) {
    // Game is over if all bunkers are destroyed
    if self.bunkers.iter().all(|b| !b.active) {
      self.game_over = true;
    }
  }

  fn render(&self) {
    clear_background(BLACK);

    // Use the stored viewport
    self.viewport.set_as_camera();
    G::filled_rect(self.view_rect, color::DARKGRAY.mul(0.5));

    // Draw ground
    let ground_rect = Rect::new(-400.0, 280.0, 800.0, GROUND_HEIGHT);
    G::filled_rect(ground_rect, color::YELLOW);

    // Draw bunkers
    for bunker in &self.bunkers {
      if bunker.active {
        let bunker_rect = Rect::new(bunker.pos.x - BUNKER_WIDTH / 2.0, bunker.pos.y, BUNKER_WIDTH, BUNKER_HEIGHT);
        G::filled_rect(bunker_rect, color::YELLOW);
      }
    }

    // Draw player missiles
    for missile in &self.player_missiles {
      if !missile.exploded {
        G::line(missile.start_pos, missile.current_pos, 1.0, color::WHITE);
      }
    }

    // Draw enemy missiles
    for missile in &self.enemy_missiles {
      if !missile.exploded {
        G::line(missile.start_pos, missile.current_pos, 1.0, color::WHITE);
      }
    }

    // Draw explosions
    for explosion in &self.explosions {
      G::circle(explosion.pos, explosion.radius, 1.0, color::WHITE);
    }

    // Draw game over text
    if self.game_over {
      G::centered_text("GAME OVER", 0.0, -20.0, 40.0, color::WHITE);
      G::centered_text("Press any key to restart", 0.0, 20.0, 20.0, color::WHITE);
    }
  }
}

impl AppState for Game {
  fn update(&mut self) {
    if is_key_down(KeyCode::Escape) {
      process::exit(0);
    }

    Diagnostics::update();
    Frame::update();

    // Update viewport with current screen size
    let (viewport, view_rect) = Self::create_viewport();
    self.viewport = viewport;
    self.view_rect = view_rect;

    let dt = Frame::get().t;

    if self.game_over {
      if !get_keys_down().is_empty() {
        self.reset();
      }
    } else {
      // Handle mouse click for firing missiles
      if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_pos();
        let world_pos = self.viewport.vec2_to_view(mouse_pos);

        if let Some(bunker_idx) = self.find_closest_active_bunker(world_pos) {
          let bunker = &mut self.bunkers[bunker_idx];
          bunker.firing = true;

          // Calculate direction vector
          let direction = (world_pos - bunker.pos).normalize();

          self.player_missiles.push(PlayerMissile {
            start_pos: bunker.pos,
            target_pos: world_pos,
            current_pos: bunker.pos,
            direction,
            exploded: false,
          });
        }
      }

      // Spawn enemy missiles
      self.time_since_last_spawn += dt;
      if self.time_since_last_spawn >= ENEMY_MISSILE_SPAWN_INTERVAL {
        self.spawn_enemy_missile();
        self.time_since_last_spawn = 0.0;
      }

      // Update game state
      self.update_missiles(dt);
      self.update_explosions(dt);
      self.check_game_over();
    }

    self.render();

    set_default_camera();
    Diagnostics::render();
  }
}
