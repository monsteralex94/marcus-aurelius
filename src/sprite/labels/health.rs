use crate::game_data::GameData;
use crate::sprite::{
    labels::text::*,
    traits::{Drawable, Updatable},
};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct PlayerHealthLabel {
    pub text: String,
    pub color: Color,
}

impl PlayerHealthLabel {
    pub fn new() -> PlayerHealthLabel {
        PlayerHealthLabel {
            text: String::new(),
            color: BLACK,
        }
    }
}

impl Updatable for PlayerHealthLabel {
    fn update(gd: &mut GameData) {
        gd.gs.labels.health.text = format!("{:.2}%", gd.gs.player.health * 100.0);
        gd.gs.labels.health.color.g = gd.gs.player.health;
        gd.gs.labels.health.color.r = 1.0 - gd.gs.player.health;
    }
}

impl Drawable for PlayerHealthLabel {
    fn draw(gd: &GameData) {
        draw_text_outline(
            gd.gs.labels.health.text.as_str(),
            50.0,
            50.0,
            70.0,
            3.0,
            gd.gs.labels.health.color,
            WHITE,
        );
    }
}
