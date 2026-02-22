pub const WINDOW_HEIGHT: f32 = 900.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * 4.0 / 3.0;
pub const UNIT: f32 = WINDOW_HEIGHT / 10.0;
pub const GROUND: f32 = WINDOW_HEIGHT - UNIT;

pub mod player {
    use crate::consts::UNIT;

    pub const ANIMATION_LENGTH: f32 = 30.0;

    pub const ACCELERATION: f32 = 0.5 * UNIT;
    pub const DECELERATION: f32 = 0.5 * UNIT;
    pub const MAX_SPEED: f32 = 5.0 * UNIT;

    pub const JUMP_HEIGHT: f32 = 2.0 * UNIT;
    pub const JUMP_TIME: f32 = 1.0;
    pub const GRAVITY: f32 = 8.0 * JUMP_HEIGHT / (JUMP_TIME * JUMP_TIME);
}

pub mod dialog {
    pub const DIALOG_ANIMATION_LENGTH: f32 = 1.0;
}
