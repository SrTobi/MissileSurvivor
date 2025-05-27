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
pub mod player;
pub mod star;

use bunker::Bunker;
use constants::*;
use explosion::{Explosion, ExplosionParams};
use missile::Missile;
use player::{Player, Skill};
use star::Star;

pub struct Game {
  bunkers: Vec<Bunker>,
  missiles: Vec<Missile>,
  explosions: Vec<Explosion>,
  stars: Vec<Star>,
  time_until_next_missile_spawn: f32,
  game_over: bool,
  view_rect: Rect,
  viewport: Viewport,
  game_time: f32,
  player: Player,
  level_ups_left: usize,
  // Skill selection menu state
  skill_options: Vec<Skill>,
  selected_skill_index: usize,
}

impl Game {
  // Calculate experience based on missile height
  // 0 at bunker level (y=280), 100 at top of screen (y=-300)
  fn calculate_experience_for_missile(&self, missile_pos: Vec2) -> f32 {
    // Normalize the y position between bunker level and top of screen
    let bunker_level = 280.0;
    let top_level = -300.0;
    let height_range = bunker_level - top_level;

    // Calculate normalized height (0.0 at bunker level, 1.0 at top)
    let normalized_height = (bunker_level - missile_pos.y) / height_range;

    // Scale to experience (0 at bunker level, 100 at top)
    normalized_height * 100.0
  }

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
      stars: Vec::new(),
      time_until_next_missile_spawn: INITIAL_SPAWN_TIME,
      game_over: false,
      view_rect,
      viewport,
      game_time: 0.0,
      player: Player::new(),
      level_ups_left: 0,
      skill_options: Vec::new(),
      selected_skill_index: 0,
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
    let start_pos = Vec2::new(start_x, -300.0);

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
                new_explosions.push(Explosion::new_default(bunker.pos));
              }
            }
          }
          // Player missile
          None => {
            // Check if missile reached target
            if missile.current_pos.distance(missile.target_pos) < 5.0 {
              missile.exploded = true;
              // Use player's skills for player missiles
              let params = ExplosionParams::from(&self.player);
              new_explosions.push(Explosion::new(missile.current_pos, params));
            }
          }
        }
      }
    }

    // Add all new explosions
    self.explosions.append(&mut new_explosions);
  }

  // Spawn a star at a random position above the bunkers
  fn spawn_star(&mut self) {
    let mut rng = rng();

    // Random position above the bunkers but below the top of the screen
    let x = rng.random_range(-350.0..350.0);
    let y = rng.random_range(-250.0..200.0); // Above bunkers, below top

    self.stars.push(Star::new(Vec2::new(x, y)));
  }

  fn update_explosions(&mut self, dt: f32) {
    // Collect positions for new explosions
    let mut new_explosions = Vec::new();
    // Collect positions of destroyed enemy missiles for experience
    let mut destroyed_enemy_missile_positions = Vec::new();
    // Track if any stars were hit
    let mut hit_stars: Vec<usize> = Vec::new();

    // Update existing explosions
    for explosion in &mut self.explosions {
      // Use the explosion's update method to handle growth mechanics
      explosion.update(dt);
    }

    // Check for chain reactions with missiles and stars
    for explosion in &self.explosions {
      // Check all missiles
      for missile in &mut self.missiles {
        if !missile.exploded && missile.current_pos.distance(explosion.pos) <= explosion.radius {
          missile.exploded = true;
          // Check if it's a player missile or an enemy missile
          if missile.is_player_missile() {
            // Player missile - use player's skills
            let params = ExplosionParams::from(&self.player);
            new_explosions.push(Explosion::new(missile.current_pos, params));
          } else {
            // Enemy missile - use default parameters
            new_explosions.push(Explosion::new_default(missile.current_pos));
            // Store position for experience calculation
            destroyed_enemy_missile_positions.push(missile.current_pos);
          }
        }
      }

      // Check if any stars are hit by this explosion
      self.stars.retain(|star| {
        if star.is_hit_by_explosion(explosion.pos, explosion.radius) {
          self.level_ups_left += 1;
          false
        } else {
          true
        }
      })
    }

    // Award experience for destroyed enemy missiles
    for pos in destroyed_enemy_missile_positions {
      let exp = self.calculate_experience_for_missile(pos);
      let new_stars = self.player.add_experience(exp);

      for _ in 0..new_stars {
        self.spawn_star()
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

  // Show the skill selection menu with two random skills
  fn show_skill_selection_menu(&mut self) {
    if self.skill_options.is_empty() && self.level_ups_left > 0 {
      // Get two random skills
      self.skill_options = Skill::random_subset(2);
      self.selected_skill_index = 0;
    }
  }

  // Handle player input for skill selection
  fn handle_skill_selection_input(&mut self) {
    if !self.is_skill_selection_active() {
      return;
    }

    // Get mouse position in world coordinates
    let mouse_pos = mouse_pos();
    let world_pos = self.viewport.vec2_to_view(mouse_pos);

    // Check if mouse is hovering over any skill option
    for (i, _) in self.skill_options.iter().enumerate() {
      let x_pos = -150.0 + i as f32 * 300.0;
      let y_pos = -50.0;
      let box_x = x_pos - 150.0;
      let box_y = y_pos - 30.0;
      let box_width = 300.0;
      let box_height = 200.0;

      // Check if mouse is inside this skill's box
      if world_pos.x >= box_x && world_pos.x <= box_x + box_width &&
         world_pos.y >= box_y && world_pos.y <= box_y + box_height {
        // Update selected skill index
        self.selected_skill_index = i;

        // Select skill with mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
          if let Some(selected_skill) = self.skill_options.get(self.selected_skill_index) {
            // Level up the selected skill
            self.player.level_up_skill(*selected_skill);
            self.level_ups_left -= 1;
            self.skill_options.clear();

            // If there are more level ups left, show the menu again
            if self.level_ups_left > 0 {
              self.show_skill_selection_menu();
            }
          }
        }

        break;
      }
    }
  }

  fn render(&self) {
    clear_background(BLACK);

    // Use the stored viewport
    self.viewport.set_as_camera();
    G::filled_rect(self.view_rect, color::DARKGRAY.mul(0.5));

    // Draw experience bar at the top of the screen
    let exp_progress = self.player.experience_progress();
    let exp_bar_width = 800.0 * exp_progress;
    let exp_bar_rect = Rect::new(-400.0, -300.0, exp_bar_width, 3.0);
    G::filled_rect(exp_bar_rect, color::BLUE);

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

    // Draw stars
    for star in &self.stars {
      if star.active {
        // Draw a blue star (a circle for now)
        G::circle(star.pos, star.radius, 1.0, color::BLUE);
      }
    }

    // Draw timer in top right corner
    let timer_text = Game::format_time(self.game_time);
    G::centered_text(&timer_text, 350.0, -280.0, 20.0, color::WHITE);

    // Draw game over text
    if self.game_over {
      G::centered_text("GAME OVER", 0.0, -20.0, 40.0, color::WHITE);
      G::centered_text("Press any key to restart", 0.0, 20.0, 20.0, color::WHITE);
    }

    // Draw skill selection menu if active
    if self.is_skill_selection_active() {
      // Draw semi-transparent background
      let menu_bg = Rect::new(-300.0, -200.0, 600.0, 400.0);
      G::filled_rect(menu_bg, color::BLACK.with_alpha(0.8));

      // Draw title
      G::centered_text("LEVEL UP!", 0.0, -150.0, 30.0, color::YELLOW);
      G::centered_text("Choose a skill to improve:", 0.0, -110.0, 20.0, color::WHITE);

      // Draw skill options
      for (i, skill) in self.skill_options.iter().enumerate() {
        let x_pos = -150.0 + i as f32 * 300.0;
        let y_pos = -50.0;
        let is_selected = i == self.selected_skill_index;

        // Draw selection box
        if is_selected {
          let box_x = x_pos - 150.0;
          let box_y = y_pos - 30.0;
          let box_width = 300.0;
          let box_height = 200.0;

          // Draw the four lines of the rectangle
          G::line(Vec2::new(box_x, box_y), Vec2::new(box_x + box_width, box_y), 2.0, color::YELLOW); // Top
          G::line(Vec2::new(box_x, box_y), Vec2::new(box_x, box_y + box_height), 2.0, color::YELLOW); // Left
          G::line(Vec2::new(box_x + box_width, box_y), Vec2::new(box_x + box_width, box_y + box_height), 2.0, color::YELLOW); // Right
          G::line(Vec2::new(box_x, box_y + box_height), Vec2::new(box_x + box_width, box_y + box_height), 2.0, color::YELLOW); // Bottom
        }

        // Draw skill name
        let color = if is_selected { color::YELLOW } else { color::WHITE };
        G::centered_text(skill.name(), x_pos, y_pos, 25.0, color);

        // Draw current level
        let level = self.player.get_skill_level(*skill);
        let level_text = format!("Current Level: {}", level);
        G::centered_text(&level_text, x_pos, y_pos + 40.0, 20.0, color);

        // Draw description
        G::centered_text(skill.description(), x_pos, y_pos + 80.0, 15.0, color);
      }

      // Draw instructions
      G::centered_text("Hover over a skill and click to select", 0.0, 150.0, 15.0, color::WHITE);
    }
  }

  fn is_skill_selection_active(&self) -> bool {
    self.level_ups_left > 0
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

    if self.game_over {
      if !get_keys_down().is_empty() {
        self.reset();
      }
    } else {
      // Show skill selection menu if there are level ups available
      self.show_skill_selection_menu();

      // Handle skill selection input
      if self.is_skill_selection_active() {
        self.handle_skill_selection_input();
      } else {
        // Update game time
        if !self.game_over {
          self.game_time += dt;
        }

        // Handle mouse click for firing missiles
        if is_mouse_button_pressed(MouseButton::Left) {
          let mouse_pos = mouse_pos();
          let world_pos = self.viewport.vec2_to_view(mouse_pos);

          if let Some(bunker_idx) = self.find_closest_active_bunker(world_pos) {
            let bunker = &mut self.bunkers[bunker_idx];
            bunker.firing = true;

            // Use player's missile speed skill
            let missile_speed = self.player.get_missile_speed();
            self.missiles.push(Missile::new(bunker.pos, world_pos, None, missile_speed));
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
    }

    self.render();

    set_default_camera();
    Diagnostics::render();
  }
}
