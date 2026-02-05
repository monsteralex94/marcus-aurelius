use std::error::Error;

use crate::level::level::LevelGroupData;
use crate::scenes::Scene;
use crate::sprite::labels::PlayerHealthLabel;
use crate::sprite::{
    Background, Player,
    labels::{DialogLabel, LevelLabel},
    traits::{Drawable, Updatable},
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct ActiveGameData {
    pub current_scene: Scene,
    pub current_level: usize,
    pub current_stage: usize,
    pub current_dialog: usize,
    pub controls_on: bool,
    pub dt: f32,
}

impl ActiveGameData {
    pub fn new() -> ActiveGameData {
        ActiveGameData {
            current_scene: Scene::MainMenu,
            current_level: 0,
            current_stage: 0,
            current_dialog: 0,
            controls_on: true,
            dt: 0.0,
        }
    }

    pub fn reset(&mut self, current_level: usize) {
        self.current_scene = Scene::Playing;
        self.current_level = current_level;
        self.current_stage = 0;
        self.current_dialog = 0;
        self.controls_on = true;
    }
}

#[derive(Debug)]
pub struct Labels {
    pub dialog: DialogLabel,
    pub level: LevelLabel,
    pub health: PlayerHealthLabel,
}

impl Labels {
    pub fn new() -> Labels {
        Labels {
            dialog: DialogLabel::new(),
            level: LevelLabel::new(),
            health: PlayerHealthLabel::new(),
        }
    }
}

impl Updatable for Labels {
    fn update(gd: &mut GameData) {
        DialogLabel::update(gd);
        LevelLabel::update(gd);
        PlayerHealthLabel::update(gd);
    }
}

impl Drawable for Labels {
    fn draw(gd: &GameData) {
        DialogLabel::draw(gd);
        LevelLabel::draw(gd);
        PlayerHealthLabel::draw(gd);
    }
}

#[derive(Debug)]
pub struct GameSprites {
    pub background: Background,
    pub player: Player,
    pub labels: Labels,
}

impl Updatable for GameSprites {
    fn update(gd: &mut GameData) {
        Background::update(gd);
        Player::update(gd);
        Labels::update(gd);
    }
}

impl Drawable for GameSprites {
    fn draw(gd: &GameData) {
        Background::draw(gd);
        Player::draw(gd);
        Labels::draw(gd);
    }
}

#[derive(Debug)]
pub struct GameData {
    pub agd: ActiveGameData,
    pub lgd: LevelGroupData,
    pub gs: GameSprites,
}

impl GameData {
    pub async fn new() -> Result<GameData, Box<dyn Error>> {
        // damn this is shit
        let lgd_1 = LevelGroupData::new("assets/levels")?;
        let lgd_2 = LevelGroupData::new("assets/levels")?;

        Ok(GameData {
            agd: ActiveGameData::new(),
            lgd: lgd_1,
            gs: GameSprites {
                background: Background::new(&lgd_2).await?,
                player: Player::new().await?,
                labels: Labels::new(),
            },
        })
    }

    pub fn level_completed(&self) -> bool {
        self.agd.current_stage >= self.lgd.levels[self.agd.current_level].num_stages
    }

    pub fn get_dialog_len(&self) -> usize {
        self.lgd.levels[self.agd.current_level].dialog[self.agd.current_stage].len()
    }

    pub fn in_dialog(&self) -> bool {
        self.agd.current_dialog < self.get_dialog_len()
    }

    pub fn get_dialog(&self) -> &str {
        if self.in_dialog() {
            return self.lgd.levels[self.agd.current_level].dialog[self.agd.current_stage]
                [self.agd.current_dialog]
                .as_str();
        } else {
            return "";
        }
    }
}
