use crate::game::constants::{EXPLOSION_GROWTH_RATE, EXPLOSION_MAX_RADIUS, MISSILE_SPEED};

/// Player struct that holds the levels of the skills of the player
pub struct Player {
    // Skill levels
    player_level: u32,
    experience: f32,
    explosion_speed_level: u32,
    explosion_after_glow_level: u32,
    explosion_radius_level: u32,
    missile_speed_level: u32,
}

impl Player {
    /// Create a new player with all skills at level 0
    pub fn new() -> Self {
        Self {
            player_level: 0,
            experience: 0.0,
            explosion_speed_level: 0,
            explosion_after_glow_level: 0,
            explosion_radius_level: 0,
            missile_speed_level: 0,
        }
    }

    /// Get the explosion growth rate multiplier based on the explosion speed skill level
    /// Explosions grow 20% faster per level
    pub fn get_explosion_growth_rate(&self) -> f32 {
        EXPLOSION_GROWTH_RATE * (1.0 + 0.2 * self.explosion_speed_level as f32)
    }

    /// Get the explosion static duration multiplier based on the explosion after glow skill level
    /// Static duration grows by 20% per level
    pub fn get_explosion_static_duration(&self) -> f32 {
        0.05 * (1.0 + 0.2 * self.explosion_after_glow_level as f32) // Base value 0.05 from Explosion::new
    }

    /// Get the explosion max radius multiplier based on the explosion radius skill level
    /// Max radius is 20% bigger per level
    pub fn get_explosion_max_radius(&self) -> f32 {
        EXPLOSION_MAX_RADIUS * (1.0 + 0.2 * self.explosion_radius_level as f32)
    }

    /// Get the missile speed multiplier based on the missile speed skill level
    /// Missiles are 20% faster per level
    pub fn get_missile_speed(&self) -> f32 {
        MISSILE_SPEED * (1.0 + 0.2 * self.missile_speed_level as f32)
    }

    pub fn player_level(&self) -> u32 {
        self.player_level
    }

    /// Calculate the experience required for the next level
    /// First level requires 50 experience, each subsequent level requires 10% more
    pub fn experience_required_for_next_level(&self) -> f32 {
        50.0 * (1.0 + 0.1 * self.player_level as f32)
    }

    /// Add experience to the player
    /// Returns true if the player has enough experience to level up
    pub fn add_experience(&mut self, amount: f32) -> usize {
        self.experience += amount;
        
        let mut stars = 0;
        while self.experience >= self.experience_required_for_next_level() {
            stars += 1;
            self.experience -= self.experience_required_for_next_level();
        }
        stars
    }

    /// Level up the player
    /// This will reset the experience counter and increment the player level
    pub fn level_up(&mut self) {
        self.player_level += 1;
    }

    /// Get the current experience
    pub fn experience(&self) -> f32 {
        self.experience
    }

    /// Get the experience progress as a percentage (0.0 - 1.0)
    pub fn experience_progress(&self) -> f32 {
        self.experience / self.experience_required_for_next_level()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
