use crate::GameData;

use gilrs::{Button, EventType};
use macroquad::prelude::*;

#[derive(Debug)]
pub enum GameInput {
    Keyboard,
    NormalGamepad,
    CursedGamepad,
}

pub fn keyboard(gd: &mut GameData) {
    gd.gs.player.jumping = false;

    if is_key_pressed(KeyCode::A) {
        gd.gs.player.going_left = true;
    }

    if is_key_released(KeyCode::A) {
        gd.gs.player.going_left = false;
    }

    if is_key_pressed(KeyCode::D) {
        gd.gs.player.going_right = true;
    }

    if is_key_released(KeyCode::D) {
        gd.gs.player.going_right = false;
    }

    if is_key_pressed(KeyCode::W) {
        gd.gs.player.jumping = true;
    }

    if is_key_pressed(KeyCode::N) {
        gd.gs.labels.dialog.next = true;
    }
}

pub fn normal_gamepad(gd: &mut GameData) {
    gd.gs.player.jumping = false;

    while let Some(ev) = gd.girls.next_event() {
        match ev.event {
            EventType::ButtonPressed(Button::DPadRight, _) => {
                gd.gs.player.going_right = true;
            }
            EventType::ButtonReleased(Button::DPadRight, _) => {
                gd.gs.player.going_right = false;
            }
            EventType::ButtonPressed(Button::DPadLeft, _) => {
                gd.gs.player.going_left = true;
            }
            EventType::ButtonReleased(Button::DPadLeft, _) => {
                gd.gs.player.going_left = false;
            }
            EventType::ButtonPressed(Button::South, _) => {
                gd.gs.player.jumping = true;
            }
            EventType::ButtonPressed(Button::East, _) => {
                gd.agd.current_dialog += 1;
                gd.gs.labels.dialog.reset();
            }
            _ => (),
        }
    }
}

pub fn cursed_gamepad(gd: &mut GameData) {
    gd.gs.player.jumping = false;

    while let Some(ev) = gd.girls.next_event() {
        match ev.event {
            EventType::ButtonChanged(Button::DPadRight, value, _) => {
                if value >= 0.9 {
                    gd.gs.player.going_left = false;
                    gd.gs.player.going_right = true;
                } else if value <= 0.1 {
                    gd.gs.player.going_left = true;
                    gd.gs.player.going_right = false;
                } else {
                    gd.gs.player.going_left = false;
                    gd.gs.player.going_right = false;
                }
            }
            EventType::ButtonPressed(Button::South, _) => {
                gd.gs.player.jumping = true;
            }
            EventType::ButtonPressed(Button::East, _) => {
                gd.agd.current_dialog += 1;
                gd.gs.labels.dialog.reset();
            }
            _ => (),
        }
    }
}

pub fn main_controls(gd: &mut GameData) {
    match &gd.agd.controls {
        GameInput::Keyboard => keyboard(gd),
        GameInput::NormalGamepad => normal_gamepad(gd),
        GameInput::CursedGamepad => cursed_gamepad(gd),
    }
}
