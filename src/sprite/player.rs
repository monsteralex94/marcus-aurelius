use crate::consts::marcus::*;
use crate::consts::*;
use crate::game_data::GameData;
use crate::sprite::traits::*;
use std::error::Error;

use macroquad::prelude::*;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum MarcusMovementFrame {
    Left = 1,
    Right = 2,
}

#[derive(Debug)]
pub struct Player {
    pub texture: Texture2D,
    pub frames: Vec<Rect>,
    pub current_frame: MarcusMovementFrame,
    pub switch_frame_time: f32,
    pub pos: Vec2,
    pub moving: bool,
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
            current_frame: MarcusMovementFrame::Left,
            switch_frame_time: 0.0,
            pos: vec2(0.0, GROUND),
            moving: false,
            facing_left: false,
            vx: 0.0,
            vy: 0.0,
            health: 1.0,
        })
    }
}

impl Updatable for Player {
    fn update(gd: &mut GameData) {
        if gd.gs.player.pos.x < -UNIT_SIZE {
            if gd.agd.current_stage > 0 {
                gd.agd.current_stage -= 1;
                gd.agd.current_dialog = 0;
                gd.gs.player.pos.x = WINDOW_WIDTH;
            } else {
                gd.gs.player.pos.x = -UNIT_SIZE;
            }
        } else if gd.gs.player.pos.x > WINDOW_WIDTH {
            gd.agd.current_stage += 1;
            gd.agd.current_dialog = 0;
            gd.gs.player.pos.x = -UNIT_SIZE;
        }

        gd.gs.player.moving = false;

        if gd.agd.controls_on {
            if is_key_down(KeyCode::A) {
                if gd.gs.player.vx > -MARCUS_X_MAX_SPEED {
                    gd.gs.player.vx -= MARCUS_X_ACCELERATION;
                } else {
                    gd.gs.player.vx = -MARCUS_X_MAX_SPEED;
                }

                gd.gs.player.moving = true;
                gd.gs.player.facing_left = true;
            }

            if is_key_down(KeyCode::D) {
                if gd.gs.player.vx < MARCUS_X_MAX_SPEED {
                    gd.gs.player.vx += MARCUS_X_ACCELERATION;
                } else {
                    gd.gs.player.vx = MARCUS_X_MAX_SPEED;
                }

                gd.gs.player.moving = true;
                gd.gs.player.facing_left = false;
            }
        }

        if !gd.gs.player.moving {
            if gd.gs.player.vx < -MARCUS_X_DECELERATION / 2.0 {
                gd.gs.player.vx += MARCUS_X_DECELERATION;
            } else if gd.gs.player.vx > MARCUS_X_DECELERATION / 2.0 {
                gd.gs.player.vx -= MARCUS_X_DECELERATION;
            } else {
                gd.gs.player.vx = 0.0;
            }
        }

        gd.gs.player.pos.x += MARCUS_X_SPEED_MUL * gd.gs.player.vx * gd.agd.dt;

        if gd.agd.controls_on && is_key_down(KeyCode::W) {
            if gd.gs.player.pos.y == GROUND {
                gd.gs.player.vy -= MARCUS_JUMP_POWER;
            }
        }

        gd.gs.player.pos.y += MARCUS_Y_SPEED_MUL * gd.gs.player.vy * gd.agd.dt;
        gd.gs.player.vy += GRAVITY_CONSTANT * gd.agd.dt;

        if gd.gs.player.pos.y > GROUND {
            gd.gs.player.pos.y = GROUND;
            gd.gs.player.vy = 0.0;
        }

        if gd.gs.player.moving {
            if gd.gs.player.switch_frame_time >= MARCUS_ANIMATION_LENGTH / gd.gs.player.vx.abs() {
                gd.gs.player.switch_frame_time = 0.0;
                gd.gs.player.current_frame = match gd.gs.player.current_frame {
                    MarcusMovementFrame::Left => MarcusMovementFrame::Right,
                    MarcusMovementFrame::Right => MarcusMovementFrame::Left,
                };
            }

            gd.gs.player.switch_frame_time += gd.agd.dt;
        }
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
                dest_size: Some(vec2(UNIT_SIZE, UNIT_SIZE)),
                flip_x: gd.gs.player.facing_left,
                source: Some(match gd.gs.player.moving {
                    true => gd.gs.player.frames[gd.gs.player.current_frame as usize],
                    false => gd.gs.player.frames[0],
                }),
                ..Default::default()
            },
        );
    }
}
