use crate::game_data::GameData;
use crate::level::LevelGroupData;
use crate::sprite::traits::{Drawable, Updatable};
use std::error::Error;

use macroquad::prelude::*;

#[derive(Debug)]
struct SingleBackground {
    texture: Texture2D,
}

impl SingleBackground {
    pub async fn new(directory: &str, level: &str, stage: usize) -> Result<Self, Box<dyn Error>> {
        let texture =
            load_texture(format!("{}/{}/{}.png", directory, level, stage).as_str()).await?;
        texture.set_filter(FilterMode::Nearest);
        Ok(Self { texture: texture })
    }
}

#[derive(Debug)]
pub struct Background {
    backgrounds: Vec<Vec<SingleBackground>>,
    current_level: usize,
    current_stage: usize,
}

impl Background {
    pub async fn new(lgd: &LevelGroupData) -> Result<Background, Box<dyn Error>> {
        let mut backgrounds: Vec<Vec<SingleBackground>> = Vec::new();

        for level in &lgd.levels {
            let mut level_backgrounds: Vec<SingleBackground> = Vec::new();

            for stage_i in 0..level.num_stages {
                level_backgrounds.push(
                    SingleBackground::new(lgd.directory.as_str(), level.name.as_str(), stage_i)
                        .await?,
                );
            }

            backgrounds.push(level_backgrounds);
        }

        Ok(Background {
            backgrounds: backgrounds,
            current_level: 0,
            current_stage: 0,
        })
    }
}

impl Updatable for Background {
    fn update(gd: &mut GameData) {
        gd.gs.background.current_level = gd.agd.current_level;
        gd.gs.background.current_stage = gd.agd.current_stage;
    }
}

impl Drawable for Background {
    fn draw(gd: &GameData) {
        draw_texture(
            &gd.gs.background.backgrounds[gd.gs.background.current_level]
                [gd.gs.background.current_stage]
                .texture,
            0.0,
            0.0,
            WHITE,
        );
    }
}
