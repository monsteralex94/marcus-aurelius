mod consts;
mod game_data;
mod level;
mod scenes;
mod sprite;

use consts::*;
use game_data::GameData;
use macroquad::prelude::*;
use scenes::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Marcus Aurelius".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gd = match GameData::new().await {
        Ok(gd_) => gd_,
        Err(e) => {
            println!("An error occured while loading the game: {}", e);
            return;
        }
    };

    loop {
        match gd.agd.current_scene {
            Scene::Playing => playing(&mut gd),
            Scene::LevelCompleted => level_completed(&mut gd),
            Scene::MainMenu => main_menu(&mut gd),
            Scene::Exit => return,
        }

        if is_key_pressed(KeyCode::C) {
            dbg!(&gd.agd);
        }

        if is_key_pressed(KeyCode::L) {
            dbg!(&gd.lgd);
        }

        if is_key_pressed(KeyCode::G) {
            dbg!(&gd);
        }

        next_frame().await;
    }
}
