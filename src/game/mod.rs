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

pub mod bunker;
pub mod constants;
pub mod explosion;
pub mod frame;
pub mod missile;

use bunker::Bunker;
use constants::*;
use explosion::Explosion;
use missile::Missile;

pub struct Game {
  bunkers: Vec<Bunker>,
  missiles: Vec<Missile>,
  explosions: Vec<Explosion>,
  time_until_next_missile_spawn: f32,
  game_over: bool,
  view_rect: Rect,
  viewport: Viewport,
  game_time: f32,
}

impl Game {
  pub fn new() -> Box<Game> {
    // Create three bunkers at the bottom of the screen
    let bunkers = vec![
      Bunker::new(Vec2::new(-200.0, 280.0 - BUNKER_HEIGHT)),
      Bunker::new(Vec2::new(0.0, 280.0 - BUNKER_HEIGHT)),
      Bunker::new(Vec2::new(200.0, 280.0 - BUNKER_HEIGHT)),
    ];

    let (viewport, view_rect) = Self::create_viewport();

    Box::new(Game {
      bunkers,
      missiles: Vec::new(),
      explosions: Vec::new(),
      time_until_next_missile_spawn: INITIAL_SPAWN_TIME,
      game_over: false,
      view_rect,
      viewport,
      game_time: 0.0,
    })
  }

  fn reset(&mut self) {
    *self = *Game::new();
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

      // Calculate enemy missile speed based on elapsed time
      let speed = get_enemy_missile_speed(self.game_time);

      self
        .missiles
        .push(Missile::new(start_pos, target_pos, Some(target_bunker_idx), speed));
    }
  }

  fn update_missiles(&mut self, dt: f32) {
    let mut new_explosions = Vec::new();

    // Update all missiles
    for missile in &mut self.missiles {
      if !missile.exploded {
        missile.current_pos += missile.direction * missile.speed * dt;

        match missile.target_bunker_idx {
          // Enemy missile
          Some(target_bunker_idx) => {
            // Check if missile hit its target bunker
            if target_bunker_idx < self.bunkers.len() {
              let bunker = &mut self.bunkers[target_bunker_idx];

              if missile.current_pos.distance(bunker.pos) < 2.0 {
                missile.exploded = true;
                bunker.active = false;
                new_explosions.push(Explosion::new(bunker.pos));
              }
            }
          }
          // Player missile
          None => {
            // Check if missile reached target
            if missile.current_pos.distance(missile.target_pos) < 5.0 {
              missile.exploded = true;
              new_explosions.push(Explosion::new(missile.current_pos));
            }
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
      // Use the explosion's update method to handle growth mechanics
      explosion.update(dt);
    }

    // Check for chain reactions with missiles
    for explosion in &self.explosions {
      // Check all missiles
      for missile in &mut self.missiles {
        if !missile.exploded && missile.current_pos.distance(explosion.pos) <= explosion.radius {
          missile.exploded = true;
          new_explosions.push(Explosion::new(missile.current_pos));
        }
      }
    }

    // Add all new explosions
    self.explosions.append(&mut new_explosions);

    // Remove finished explosions
    self.explosions.retain(|e| !e.has_ended());

    // Remove exploded missiles
    self.missiles.retain(|m| !m.exploded);

    // Update bunker firing status
    for bunker in &mut self.bunkers {
      if bunker.firing {
        let still_firing = self
          .missiles
          .iter()
          .any(|m| m.target_bunker_idx.is_none() && !m.exploded && m.start_pos.distance(bunker.pos) < 5.0);
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

    // Draw all missiles
    for missile in &self.missiles {
      if !missile.exploded {
        G::line(missile.start_pos, missile.current_pos, 1.0, color::WHITE);
      }
    }

    // Draw explosions
    for explosion in &self.explosions {
      G::circle(explosion.pos, explosion.radius, 1.0, color::WHITE);
    }

    // Draw timer in top right corner
    let timer_text = Game::format_time(self.game_time);
    G::centered_text(&timer_text, 350.0, -280.0, 20.0, color::WHITE);

    // Draw game over text
    if self.game_over {
      G::centered_text("GAME OVER", 0.0, -20.0, 40.0, color::WHITE);
      G::centered_text("Press any key to restart", 0.0, 20.0, 20.0, color::WHITE);
    }
  }

  // Format elapsed time as mm:ss
  fn format_time(seconds: f32) -> String {
    let total_seconds = seconds as u32;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
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

    // Update game time
    if !self.game_over {
      self.game_time += dt;
    }

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

          self.missiles.push(Missile::new(bunker.pos, world_pos, None, MISSILE_SPEED));
        }
      }

      // Spawn enemy missiles
      self.time_until_next_missile_spawn -= dt;

      if self.time_until_next_missile_spawn <= 0.0 {
        self.spawn_enemy_missile();

        // Get base spawn interval
        let spawn_interval = get_enemy_missile_spawn_interval(self.game_time);

        // Randomize by ±50%
        let mut rng = rng();
        let random_factor = rng.random_range(0.5..1.5);
        self.time_until_next_missile_spawn = spawn_interval * random_factor;
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
