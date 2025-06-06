#![allow(dead_code, unused_imports)]

use std::{mem::transmute, sync::Mutex};

use env_logger::Env;
use macroquad::prelude::*;

mod game;
mod utils;

trait AppState {
  fn update(&mut self);
}

fn set_state(state: Box<dyn AppState>) {
  let mut next_state = NEXT_STATE.lock().unwrap();
  *next_state = Some(unsafe { transmute::<Box<dyn AppState>, Box<dyn AppState + Send>>(state) });
}

lazy_static::lazy_static! {
    static ref NEXT_STATE: Mutex<Option<Box<dyn AppState + Send>>> = Mutex::new(None);
}

#[macroquad::main("MissileSurvivor")]
async fn main() {
  env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .parse_env(Env::new())
    .init();

  let mut state: Box<dyn AppState> = game::Game::new();

  loop {
    if let Some(next_state) = NEXT_STATE.lock().unwrap().take() {
      state = next_state;
    }
    state.update();

    next_frame().await;
  }
}

/*
const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}


fn make_game_camera() -> Camera2D {
    let w = screen_width();
    let h = screen_height();

    let zoom = if w > h {
        vec2(h / w, 1.0)
    } else {
        vec2(1.0, w / h)
    };

    Camera2D {
        zoom: zoom / 200.0,
        ..Camera2D::default()
    }
}

#[macroquad::main("gravity.io")]
async fn main() {
    /*let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rot: 0.,
        vel: Vec2::new(0., 0.),
    };

    let mut bullets = Vec::new();
    let mut last_shot = get_time();
    let mut asteroids = Vec::new();
    let mut gameover = false;

    let mut screen_center;*/

    let mut pos = vec2(0.0, -100.0);
    let mut vel: Vec2 = vec2(32., 0.0);
    let mut acc: Vec2 = vec2(0.0, 0.0);
    let sun_mass = 100000.0;
    let planet_mass = 100.0;

    let steps_per_second = 40;
    let time_per_step: f32 = 1.0 / steps_per_second as f32;
    let mut time_until_next = time_per_step;

    let mut path = VecDeque::new();
    let mut step = 0;
    let path_len = 1000;

    loop {
        let camera = make_game_camera();
        clear_background(LIGHTGRAY);
        push_camera_state();
        set_camera(&camera);

        let speed_up = if is_key_down(KeyCode::Space) {
            50.0
        } else {
            1.0
        };

        let (mx, my) = mouse_position();
        let mpos = vec2(mx, my);
        let mpos = camera.screen_to_world(mpos);
        draw_line(pos.x, pos.y, mpos.x, mpos.y, 0.4, WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            let diff = (mpos - pos).normalize();
            vel += diff;
        }

        let frame_time = get_frame_time() as f32 * speed_up;
        time_until_next -= frame_time;
        while time_until_next <= 0.0 {
            time_until_next += time_per_step;
            step += 1;

            let force = (-pos).normalize() * sun_mass * planet_mass / pos.length().powi(2);
            acc = force / planet_mass;
            vel += acc * time_per_step;
            pos += vel * time_per_step;

            if step % (steps_per_second / 5) == 0 {
                path.push_back(pos);
                if path.len() > path_len {
                    path.pop_front();
                }
            }
        }

        draw_poly(0.0, 0.0, 50, 10.0, 0.0, BLACK);

        if path.len() > 1 {
            for i in 0..path.len() - 1 {
                let p1 = path[i];
                let p2 = path[i + 1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 0.2, DARKGREEN);
            }
        }

        draw_circle(pos.x, pos.y, 1.0, BLACK);

        let vel_pos = pos + vel;
        draw_line(pos.x, pos.y, vel_pos.x, vel_pos.y, 1., RED);

        let acc_pos = pos + acc;
        draw_line(pos.x, pos.y, acc_pos.x, acc_pos.y, 1., BLUE);

        pop_camera_state();

        /*if gameover {
            clear_background(LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if asteroids.len() > 0 {
                text = "Game Over. Press [enter] to play again.";
            }
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
            if is_key_down(KeyCode::Enter) {
                ship = Ship {
                    pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                    rot: 0.,
                    vel: Vec2::new(0., 0.),
                };
                bullets = Vec::new();
                asteroids = Vec::new();
                gameover = false;
                screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
                for _ in 0..10 {
                    asteroids.push(Asteroid {
                        pos: screen_center
                            + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                                .normalize()
                                * screen_width().min(screen_height())
                                / 2.,
                        vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                        rot: 0.,
                        rot_speed: rand::gen_range(-2., 2.),
                        size: screen_width().min(screen_height()) / 10.,
                        sides: rand::gen_range(3, 8),
                        collided: false,
                    })
                }
            }
            next_frame().await;
            continue;
        }
        let frame_t = get_time();
        let rotation = ship.rot.to_radians();

        let mut acc = -ship.vel / 100.; // Friction

        // Forward
        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }

        // Shot
        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.5 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            bullets.push(Bullet {
                pos: ship.pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: rot_vec * 7.,
                shot_at: frame_t,
                collided: false,
            });
            last_shot = frame_t;
        }

        // Steer
        if is_key_down(KeyCode::Right) {
            ship.rot += 5.;
        } else if is_key_down(KeyCode::Left) {
            ship.rot -= 5.;
        }

        // Euler integration
        ship.vel += acc;
        if ship.vel.length() > 5. {
            ship.vel = ship.vel.normalize() * 5.;
        }
        ship.pos += ship.vel;
        ship.pos = wrap_around(&ship.pos);

        // Move each bullet
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }

        // Move each asteroid
        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(&asteroid.pos);
            asteroid.rot += asteroid.rot_speed;
        }

        // Bullet lifetime
        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            // Asteroid/ship collision
            if (asteroid.pos - ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                gameover = true;
                break;
            }

            // Asteroid/bullet collision
            for bullet in bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;

                    // Break the asteroid
                    if asteroid.sides > 3 {
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(bullet.vel.y, -bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(-bullet.vel.y, bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        })
                    }
                    break;
                }
            }
        }

        // Remove the collided objects
        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        asteroids.retain(|asteroid| !asteroid.collided);
        asteroids.append(&mut new_asteroids);

        // You win?
        if asteroids.len() == 0 {
            gameover = true;
        }

        if gameover {
            continue;
        }

        clear_background(LIGHTGRAY);

        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., BLACK);
        }

        for asteroid in asteroids.iter() {
            draw_poly_lines(
                asteroid.pos.x,
                asteroid.pos.y,
                asteroid.sides,
                asteroid.size,
                asteroid.rot,
                2.,
                BLACK,
            )
        }

        let v1 = Vec2::new(
            ship.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            ship.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            ship.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);*/

        next_frame().await
    }
}

*/
