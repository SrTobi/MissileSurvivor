use macroquad::prelude::Vec2;
use crate::game::constants::{EXPLOSION_MAX_RADIUS, EXPLOSION_GROWTH_RATE};
use crate::game::player::Player;

#[derive(PartialEq)]
pub enum ExplosionPhase {
    Linear,    // Phase 1: Linear growth until 80% of max radius
    Decelerate, // Phase 2: Decreasing speed until max radius
    Static     // Phase 3: Stay at maximum radius for a short time
}

/// Parameters for creating an explosion
pub struct ExplosionParams {
    pub max_radius: f32,
    pub growth_rate: f32,
    pub static_duration: f32,
}

impl Default for ExplosionParams {
    fn default() -> Self {
        Self {
            max_radius: EXPLOSION_MAX_RADIUS,
            growth_rate: EXPLOSION_GROWTH_RATE,
            static_duration: 0.05, // Default static duration from Explosion::new
        }
    }
}

impl From<&Player> for ExplosionParams {
    fn from(player: &Player) -> Self {
        Self {
            max_radius: player.get_explosion_max_radius(),
            growth_rate: player.get_explosion_growth_rate(),
            static_duration: player.get_explosion_static_duration(),
        }
    }
}

pub struct Explosion {
  pub pos: Vec2,
  pub radius: f32,
  pub max_radius: f32,
  pub phase: ExplosionPhase,
  pub phase_transition_radius: f32, // 80% of max_radius
  pub initial_growth_rate: f32,   // The constant growth rate in phase 1
  pub current_growth_rate: f32,   // The current growth rate (changes in phase 2)
  pub static_duration: f32,       // How long the explosion stays at max radius
  pub static_time_elapsed: f32,   // Time elapsed in the static phase
}

impl Explosion {
  pub fn new(pos: Vec2, params: ExplosionParams) -> Explosion {
    let max_radius = params.max_radius;
    let phase_transition_radius = max_radius * 0.8;
    let initial_growth_rate = params.growth_rate;
    let static_duration = params.static_duration;

    Explosion {
      pos,
      radius: 0.0,
      max_radius,
      phase: ExplosionPhase::Linear,
      phase_transition_radius,
      initial_growth_rate,
      current_growth_rate: initial_growth_rate,
      static_duration,
      static_time_elapsed: 0.0,
    }
  }

  // Convenience method that uses default parameters
  pub fn new_default(pos: Vec2) -> Explosion {
    Self::new(pos, ExplosionParams::default())
  }

  pub fn update(&mut self, dt: f32) {
    match self.phase {
      // Phase 1: Linear growth until 80% of max radius
      ExplosionPhase::Linear => {
        // Linear growth at constant rate
        self.radius += self.current_growth_rate * dt;

        // Check if we need to transition to phase 2
        if self.radius >= self.phase_transition_radius {
          self.phase = ExplosionPhase::Decelerate;

          // Calculate how much we overshot the transition point
          let overshoot = self.radius - self.phase_transition_radius;
          self.radius = self.phase_transition_radius;

          // Apply the remaining growth with phase 2 logic
          // We want phase 2 to take the same time as phase 1
          // Since phase 2 covers 20% of the radius and phase 1 covers 80%,
          // we need to adjust the growth rate to make phase 2 take the same time

          // Calculate the initial deceleration for phase 2
          // We want to start at the same speed as phase 1 and gradually slow down
          let remaining_distance = self.max_radius - self.phase_transition_radius;
          let initial_deceleration = self.initial_growth_rate * self.initial_growth_rate / (2.0 * remaining_distance);

          // Apply the remaining growth with deceleration
          if overshoot > 0.0 {
            // Calculate how much to grow in phase 2 with the overshoot time
            let overshoot_time = overshoot / self.initial_growth_rate;
            let phase2_growth = self.initial_growth_rate * overshoot_time - 
                               0.5 * initial_deceleration * overshoot_time * overshoot_time;
            self.radius += phase2_growth.min(remaining_distance);

            // Update current growth rate for next frame
            self.current_growth_rate = (self.initial_growth_rate - initial_deceleration * overshoot_time)
                                       .max(0.0);
          }
        }
      },
      // Phase 2: Decreasing speed until max radius
      ExplosionPhase::Decelerate => {
        // Calculate remaining distance
        let remaining_distance = self.max_radius - self.radius;

        if remaining_distance > 0.1 {
          // Calculate deceleration needed to reach max_radius with decreasing speed
          let deceleration = self.current_growth_rate * self.current_growth_rate / (2.0 * remaining_distance);

          // Calculate distance to grow this frame with deceleration
          let growth = self.current_growth_rate * dt - 0.5 * deceleration * dt * dt;

          // Apply growth, ensuring we don't exceed max_radius
          self.radius += growth.min(remaining_distance);

          // Update current growth rate for next frame
          self.current_growth_rate = (self.current_growth_rate - deceleration * dt).max(0.0);
        } else {
          self.phase = ExplosionPhase::Static;
          self.radius = self.max_radius; // Ensure we don't exceed max radius
          self.static_time_elapsed = 0.0; // Reset static phase timer
        }
      },
      // Phase 3: Stay at maximum radius for a short time
      ExplosionPhase::Static => {
        // Update elapsed time in static phase
        self.static_time_elapsed += dt;
        // No need to update radius as it's already at max
      }
    }
  }

  // Check if the explosion has completed all phases
  pub fn has_ended(&self) -> bool {
    // Only end if we're in the Static phase and have stayed there for the required duration
    self.phase == ExplosionPhase::Static && self.static_time_elapsed >= self.static_duration
  }
}
