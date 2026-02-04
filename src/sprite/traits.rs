use crate::game_data::GameData;

pub trait Updatable {
    fn update(gd: &mut GameData);
}

pub trait Drawable {
    fn draw(gd: &GameData);
}
