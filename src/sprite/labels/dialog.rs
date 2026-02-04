use crate::consts::dialog::DIALOG_ANIMATION_LENGTH;
use crate::game_data::GameData;
use crate::sprite::{
    labels::text::*,
    traits::{Drawable, Updatable},
};
use macroquad::prelude::*;

fn draw_text_dialog(text: &str) {
    let lines: Vec<&str> = text.split('\n').collect();
    let font_size = 30.0;
    let line_spacing = 0.0;
    let y_pos = 270.0;

    let block_height = lines.len() as f32 * (font_size + line_spacing) - line_spacing;

    for (i, line) in lines.iter().enumerate() {
        let x = get_centered_text_x(line, font_size);
        let y =
            (screen_height() - block_height) / 2.0 + i as f32 * (font_size + line_spacing) + y_pos;
        draw_text_outline(line, x, y, font_size, 2.0, WHITE, BLACK);
    }
}

#[derive(Debug)]
pub struct DialogLabel {
    pub text: String,
    pub letters_num: usize,
    pub current_letters: usize,
    pub letter_timer: f32,
}

impl DialogLabel {
    pub fn new() -> DialogLabel {
        DialogLabel {
            text: String::new(),
            letters_num: 0,
            current_letters: 0,
            letter_timer: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.current_letters = 0;
        self.letter_timer = 0.0;
    }
}

impl Updatable for DialogLabel {
    fn update(gd: &mut GameData) {
        if !gd.level_completed() {
            if gd.in_dialog() && is_key_pressed(KeyCode::N) {
                gd.agd.current_dialog += 1;
                gd.gs.labels.dialog.reset();
            }

            if gd.gs.labels.dialog.current_letters < gd.gs.labels.dialog.letters_num
                && gd.gs.labels.dialog.letter_timer >= DIALOG_ANIMATION_LENGTH
            {
                gd.gs.labels.dialog.letter_timer = 0.0;
                gd.gs.labels.dialog.current_letters += 1;
            }

            gd.gs.labels.dialog.text = gd.get_dialog().to_string();
            gd.gs.labels.dialog.letters_num = gd.gs.labels.dialog.text.len();
            gd.gs.labels.dialog.letter_timer += 1.0;
        }
    }
}

impl Drawable for DialogLabel {
    fn draw(gd: &GameData) {
        if !gd.level_completed() && gd.in_dialog() {
            let end = gd
                .gs
                .labels
                .dialog
                .text
                .char_indices()
                .nth(gd.gs.labels.dialog.current_letters)
                .map(|(i, _)| i)
                .unwrap_or(gd.gs.labels.dialog.current_letters);

            draw_text_dialog(&gd.gs.labels.dialog.text[0..end]);
        }
    }
}
