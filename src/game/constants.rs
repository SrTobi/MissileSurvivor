pub const GROUND_HEIGHT: f32 = 20.0;
pub const BUNKER_WIDTH: f32 = 30.0;
pub const BUNKER_HEIGHT: f32 = 20.0;
pub const MISSILE_SPEED: f32 = 100.0;
pub const EXPLOSION_MAX_RADIUS: f32 = 50.0;
pub const EXPLOSION_GROWTH_RATE: f32 = 200.0; // pixel/sec
pub const ENEMY_MISSILE_SPAWN_INTERVAL: f32 = 4.0;
pub const INITIAL_SPAWN_TIME: f32 = 2.0; // seconds

// Calculate enemy missile spawn interval based on elapsed time
pub fn get_enemy_missile_spawn_interval(elapsed_time: f32) -> f32 {
  // Start with a long interval and decrease over time
  // After 5 minutes, the interval will be 1/3 of the initial interval
  let min_interval = ENEMY_MISSILE_SPAWN_INTERVAL / 3.0;
  let time_to_min_interval = 300.0; // 5 minutes in seconds

  let interval_multiplier = 1.0 - (elapsed_time / time_to_min_interval) * (1.0 - min_interval / ENEMY_MISSILE_SPAWN_INTERVAL);
  (ENEMY_MISSILE_SPAWN_INTERVAL * interval_multiplier).max(min_interval)
}

// Calculate enemy missile speed based on elapsed time
pub fn get_enemy_missile_speed(elapsed_time: f32) -> f32 {
  // Start with a slow speed and increase over time
  // After 5 minutes, the speed will be 3 times the initial speed
  let base_speed = MISSILE_SPEED * 0.5; // Start at half the player missile speed
  let max_speed_multiplier = 3.0;
  let time_to_max_speed = 300.0; // 5 minutes in seconds

  let speed_multiplier = 1.0 + (elapsed_time / time_to_max_speed) * (max_speed_multiplier - 1.0);
  base_speed * speed_multiplier.min(max_speed_multiplier)
}
