use macroquad::prelude::Vec2;

/// A star that appears when the player's experience bar is full.
/// The player can shoot the star with an explosion to level up.
pub struct Star {
    pub pos: Vec2,
    pub active: bool,
    pub radius: f32,
}

impl Star {
    /// Create a new star at the given position
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            active: true,
            radius: 3.0, // Default radius
        }
    }

    /// Check if the star is hit by an explosion
    pub fn is_hit_by_explosion(&self, explosion_pos: Vec2, explosion_radius: f32) -> bool {
        if !self.active {
            return false;
        }
        
        // Check if the explosion overlaps with the star
        self.pos.distance(explosion_pos) <= explosion_radius + self.radius
    }
}