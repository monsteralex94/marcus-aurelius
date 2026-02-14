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
        match &gd.gs.boss {
            Some(_) => (),
            None => return,
        }

        gd.gs.labels.player_health.text = format!("{:.2}%", gd.gs.player.health * 100.0);
        gd.gs.labels.player_health.color.g = gd.gs.player.health;
        gd.gs.labels.player_health.color.r = 1.0 - gd.gs.player.health;
    }
}

impl Drawable for PlayerHealthLabel {
    fn draw(gd: &GameData) {
        match &gd.gs.boss {
            Some(_) => (),
            None => return,
        }

        draw_text_outline(
            gd.gs.labels.player_health.text.as_str(),
            50.0,
            70.0,
            70.0,
            3.0,
            gd.gs.labels.player_health.color,
            WHITE,
        );
    }
}

#[derive(Debug)]
pub struct BossHealthLabel {
    pub text: String,
    pub color: Color,
}

impl BossHealthLabel {
    pub fn new() -> BossHealthLabel {
        BossHealthLabel {
            text: String::new(),
            color: BLACK,
        }
    }
}

impl Updatable for BossHealthLabel {
    fn update(gd: &mut GameData) {
        let boss = match &gd.gs.boss {
            Some(b) => b,
            None => return,
        };

        gd.gs.labels.boss_health.text = format!("{:.2}%", boss.health * 100.0);
        gd.gs.labels.boss_health.color.g = boss.health;
        gd.gs.labels.boss_health.color.r = 1.0 - boss.health;
    }
}

impl Drawable for BossHealthLabel {
    fn draw(gd: &GameData) {
        match &gd.gs.boss {
            Some(_) => (),
            None => return,
        }

        draw_text_outline(
            gd.gs.labels.boss_health.text.as_str(),
            540.0,
            70.0,
            70.0,
            3.0,
            gd.gs.labels.boss_health.color,
            WHITE,
        );
    }
}
