use crate::game_data::GameData;
use crate::sprite::{
    labels::text::*,
    traits::{Drawable, Updatable},
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct LevelLabel {
    pub text: String,
}

impl LevelLabel {
    pub fn new() -> LevelLabel {
        LevelLabel {
            text: String::new(),
        }
    }
}

impl Updatable for LevelLabel {
    fn update(gd: &mut GameData) {
        gd.gs.labels.level.text = format!(
            "Level {}: '{}'",
            gd.agd.current_level + 1,
            gd.lgd.levels[gd.agd.current_level].name
        );
    }
}

impl Drawable for LevelLabel {
    fn draw(gd: &GameData) {
        let x = get_centered_text_x(gd.gs.labels.level.text.as_str(), 20.0);
        draw_text(gd.gs.labels.level.text.as_str(), x, 20.0, 20.0, BLACK);
    }
}
