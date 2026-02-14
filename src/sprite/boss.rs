use std::error::Error;

use crate::consts::*;
use crate::game_data::GameData;
use crate::level::LevelGroupData;
use crate::sprite::traits::{Drawable, Updatable};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Boss {
    pub texture: Texture2D,
    pub frames: Vec<Rect>,
    pub current_frame: usize,
    pub switch_frame_timer: f32,
    pub pos: Vec2,
    pub moving: bool,
    pub facing_left: bool,
    pub health: f32,
}

impl Boss {
    pub async fn new(
        lgd: &LevelGroupData,
        level: usize,
        stage: usize,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let boss_data = match &lgd.levels[level].boss_data[stage] {
            Some(d) => d,
            None => return Ok(None),
        };

        let texture = load_texture(
            format!(
                "{}/{}/{}",
                lgd.directory, lgd.levels[level].name, boss_data.texture_name
            )
            .as_str(),
        )
        .await?;
        texture.set_filter(FilterMode::Nearest);

        Ok(Some(Boss {
            texture: texture,
            frames: vec![
                Rect::new(0.0, 0.0, 32.0, 32.0),
                Rect::new(32.0, 0.0, 32.0, 32.0),
                Rect::new(0.0, 32.0, 32.0, 32.0),
            ],
            current_frame: 0,
            switch_frame_timer: 0.0,
            pos: vec2(WINDOW_WIDTH - UNIT_SIZE, GROUND),
            moving: false,
            facing_left: true,
            health: 1.0,
        }))
    }
}

impl Updatable for Boss {
    fn update(gd: &mut GameData) {
        // update the boss (with BossActionsData)
    }
}

impl Drawable for Boss {
    fn draw(gd: &GameData) {
        let boss = match &gd.gs.boss {
            Some(b) => b,
            None => return,
        };

        draw_texture_ex(
            &boss.texture,
            boss.pos.x,
            boss.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(UNIT_SIZE, UNIT_SIZE)), // adapt when larger boss
                flip_x: boss.facing_left,
                source: Some(match boss.moving {
                    true => boss.frames[boss.current_frame],
                    false => boss.frames[0],
                }),
                ..Default::default()
            },
        );
    }
}
