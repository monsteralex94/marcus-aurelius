use crate::GameData;
use crate::sprite::labels::{DialogLabel, LevelLabel, PlayerHealthLabel};
use crate::sprite::traits::{Drawable, Updatable};

#[derive(Debug)]
pub struct Labels {
    pub dialog: DialogLabel,
    pub level: LevelLabel,
    pub health: PlayerHealthLabel,
}

impl Labels {
    pub fn new() -> Labels {
        Labels {
            dialog: DialogLabel::new(),
            level: LevelLabel::new(),
            health: PlayerHealthLabel::new(),
        }
    }
}

impl Updatable for Labels {
    fn update(gd: &mut GameData) {
        DialogLabel::update(gd);
        LevelLabel::update(gd);
        PlayerHealthLabel::update(gd);
    }
}

impl Drawable for Labels {
    fn draw(gd: &GameData) {
        DialogLabel::draw(gd);
        LevelLabel::draw(gd);
        PlayerHealthLabel::draw(gd);
    }
}
