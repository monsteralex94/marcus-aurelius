use crate::consts::*;
use crate::game_data::GameData;
use crate::sprite::traits::*;
use std::error::Error;

use macroquad::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub texture: Texture2D,
    pub frames: Vec<Rect>,
    pub current_frame: usize,
    pub switch_frame_timer: f32,
    pub pos: Vec2,
    pub going_left: bool,
    pub going_right: bool,
    pub jumping: bool,
    pub facing_left: bool,
    pub vx: f32,
    pub vy: f32,
    pub health: f32,
}

impl Player {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let texture = load_texture("assets/marcus.png").await?;
        texture.set_filter(FilterMode::Nearest);

        Ok(Self {
            texture: texture,
            frames: vec![
                Rect::new(0.0, 0.0, 32.0, 32.0),
                Rect::new(32.0, 0.0, 32.0, 32.0),
                Rect::new(0.0, 32.0, 32.0, 32.0),
            ],
            current_frame: 0,
            switch_frame_timer: 0.0,
            pos: vec2(0.0, GROUND),
            going_left: false,
            going_right: false,
            jumping: false,
            facing_left: false,
            vx: 0.0,
            vy: 0.0,
            health: 1.0,
        })
    }

    // Controls
    pub fn left(gd: &mut GameData) {
        if gd.gs.player.vx > -player::MAX_SPEED {
            gd.gs.player.vx -= player::ACCELERATION;
        } else {
            gd.gs.player.vx = -player::MAX_SPEED;
        }

        gd.gs.player.facing_left = true;
    }

    pub fn right(gd: &mut GameData) {
        if gd.gs.player.vx < player::MAX_SPEED {
            gd.gs.player.vx += player::ACCELERATION;
        } else {
            gd.gs.player.vx = player::MAX_SPEED;
        }

        gd.gs.player.facing_left = false;
    }

    pub fn up(gd: &mut GameData) {
        if gd.gs.player.pos.y == GROUND - UNIT {
            gd.gs.player.vy -= (2.0 * player::GRAVITY * player::JUMP_HEIGHT).sqrt();
        }
    }

    pub fn moving(gd: &GameData) -> bool {
        gd.gs.player.going_left || gd.gs.player.going_right
    }

    pub fn stop_movement(gd: &mut GameData) {
        gd.gs.player.going_left = false;
        gd.gs.player.going_right = false;
    }

    pub fn physics(gd: &mut GameData) {
        if gd.gs.player.pos.x < -UNIT {
            gd.gs.player.pos.x = -UNIT;
        } else if gd.gs.player.pos.x > WINDOW_WIDTH {
            gd.agd.current_stage += 1;
            gd.agd.just_changed_stage = true;
            gd.agd.current_dialog = 0;
            gd.gs.player.pos.x = -UNIT;
        }

        if gd.agd.movement_on {
            if gd.gs.player.going_left {
                Player::left(gd);
            }

            if gd.gs.player.going_right {
                Player::right(gd);
            }
        }

        if !Player::moving(gd) {
            if gd.gs.player.vx < -player::DECELERATION / 2.0 {
                gd.gs.player.vx += player::DECELERATION;
            } else if gd.gs.player.vx > player::DECELERATION / 2.0 {
                gd.gs.player.vx -= player::DECELERATION;
            } else {
                gd.gs.player.vx = 0.0;
            }
        }

        gd.gs.player.pos.x += gd.gs.player.vx * gd.agd.dt;

        if gd.agd.movement_on && gd.gs.player.jumping {
            Player::up(gd);
        }

        gd.gs.player.pos.y += gd.gs.player.vy * gd.agd.dt;
        gd.gs.player.vy += player::GRAVITY * gd.agd.dt;

        if gd.gs.player.pos.y > GROUND - UNIT {
            gd.gs.player.pos.y = GROUND - UNIT;
            gd.gs.player.vy = 0.0;
        }

        if Player::moving(gd) {
            if gd.gs.player.switch_frame_timer >= player::ANIMATION_LENGTH / gd.gs.player.vx.abs() {
                gd.gs.player.switch_frame_timer = 0.0;
                gd.gs.player.current_frame = match gd.gs.player.current_frame {
                    1 => 2,
                    2 => 1,
                    _ => 1, // not important
                };
            }

            gd.gs.player.switch_frame_timer += gd.agd.dt;
        }
    }
}

impl Updatable for Player {
    fn update(gd: &mut GameData) {
        Player::physics(gd);
    }
}

impl Drawable for Player {
    fn draw(gd: &GameData) {
        draw_texture_ex(
            &gd.gs.player.texture,
            gd.gs.player.pos.x,
            gd.gs.player.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(UNIT, UNIT)),
                flip_x: gd.gs.player.facing_left,
                source: Some(match Player::moving(gd) {
                    true => gd.gs.player.frames[gd.gs.player.current_frame as usize],
                    false => gd.gs.player.frames[0],
                }),
                ..Default::default()
            },
        );
    }
}
