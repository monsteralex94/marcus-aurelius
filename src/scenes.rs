use crate::game_data::{GameData, GameSprites};
use crate::sprite::{
    labels::text::*,
    traits::{Drawable, Updatable},
};
use macroquad::prelude::*;

#[derive(Debug)]
pub enum Scene {
    Playing,
    LevelCompleted,
    MainMenu,
    Exit,
}

pub fn playing(gd: &mut GameData) {
    if gd.agd.current_level > gd.lgd.num_levels - 1 {
        dbg!("Congratulations, you finished the game!");
        gd.agd.current_scene = Scene::Exit;
        return;
    }

    if gd.level_completed() {
        gd.agd.current_scene = Scene::LevelCompleted;
        return;
    }

    gd.agd.controls_on = !gd.in_dialog();
    gd.agd.dt = get_frame_time();

    GameSprites::update(gd);
    clear_background(GREEN);
    GameSprites::draw(gd);
}

pub fn level_completed(gd: &mut GameData) {
    if is_mouse_button_pressed(MouseButton::Left) {
        gd.agd.reset(gd.agd.current_level + 1);
        return;
    }

    clear_background(GREEN);

    let text = format!(
        "Level {}: '{}' Abgeschlosen!",
        gd.agd.current_level + 1,
        gd.lgd.levels[gd.agd.current_level].name
    );

    let x = get_centered_text_x(text.as_str(), 30.0);
    draw_text(text.as_str(), x, 300.0, 30.0, BLACK);

    draw_text(
        "Bitte klicke auf den Bildschirm um fortzufahren",
        194.375,
        400.0,
        20.0,
        BLACK,
    );
}

pub fn main_menu(gd: &mut GameData) {
    if is_mouse_button_pressed(MouseButton::Left) {
        gd.agd.reset(0);
        return;
    }

    clear_background(GREEN);
    draw_text("MARCUS AURELIUS!", 80.0, 300.0, 40.0, BLACK);
    draw_text(format!("{}", gd.agd.dt).as_str(), 100.0, 400.0, 20.0, BLACK);
    draw_text("WIP", 100.0, 450.0, 20.0, BLACK);
}
