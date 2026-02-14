use crate::GameData;
use crate::sprite::labels::{BossHealthLabel, DialogLabel, LevelLabel, PlayerHealthLabel};
use crate::sprite::traits::{Drawable, Updatable};

#[derive(Debug)]
pub struct Labels {
    pub dialog: DialogLabel,
    pub level: LevelLabel,
    pub player_health: PlayerHealthLabel,
    pub boss_health: BossHealthLabel,
}

impl Labels {
    pub fn new() -> Labels {
        Labels {
            dialog: DialogLabel::new(),
            level: LevelLabel::new(),
            player_health: PlayerHealthLabel::new(),
            boss_health: BossHealthLabel::new(),
        }
    }
}

impl Updatable for Labels {
    fn update(gd: &mut GameData) {
        DialogLabel::update(gd);
        LevelLabel::update(gd);
        PlayerHealthLabel::update(gd);
        BossHealthLabel::update(gd);
    }
}

impl Drawable for Labels {
    fn draw(gd: &GameData) {
        DialogLabel::draw(gd);
        LevelLabel::draw(gd);
        PlayerHealthLabel::draw(gd);
        BossHealthLabel::draw(gd);
    }
}
