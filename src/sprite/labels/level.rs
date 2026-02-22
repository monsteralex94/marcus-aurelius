use crate::consts::WINDOW_HEIGHT;
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
        if gd.agd.current_level >= gd.lgd.levels.len() {
            return;
        }

        gd.gs.labels.level.text = format!(
            "Level {}: '{}'",
            gd.agd.current_level + 1,
            gd.lgd.levels[gd.agd.current_level].name
        );
    }
}

impl Drawable for LevelLabel {
    fn draw(gd: &GameData) {
        let font_size = WINDOW_HEIGHT / 30.0;
        let x = get_centered_text_x(gd.gs.labels.level.text.as_str(), font_size);
        draw_text(
            gd.gs.labels.level.text.as_str(),
            x,
            font_size,
            font_size,
            BLACK,
        );
    }
}
