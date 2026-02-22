mod consts;
mod controls;
mod game_data;
mod level;
mod scenes;
mod sprite;

use consts::*;
use controls::GameInput;
use game_data::GameData;
use scenes::*;

use macroquad::prelude::*;
use std::error::Error;

fn window_conf() -> Conf {
    Conf {
        window_title: "Marcus Aurelius".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

async fn running() -> Result<(), Box<dyn Error>> {
    let mut gd = GameData::new().await?;

    loop {
        match gd.agd.current_scene {
            Scene::Playing => playing(&mut gd).await?,
            Scene::LevelCompleted => level_completed(&mut gd),
            Scene::MainMenu => main_menu(&mut gd),
            Scene::Exit => break,
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

        if is_key_pressed(KeyCode::I) {
            gd.agd.controls = match &gd.agd.controls {
                GameInput::Keyboard => GameInput::NormalGamepad,
                GameInput::NormalGamepad => GameInput::CursedGamepad,
                GameInput::CursedGamepad => GameInput::Keyboard,
            };

            dbg!(&gd.agd.controls);
        }

        next_frame().await;
    }

    Ok(())
}

#[macroquad::main(window_conf)]
async fn main() {
    match running().await {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
        }
    }
}
