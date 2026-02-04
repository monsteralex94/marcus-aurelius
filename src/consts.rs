pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const UNIT_SIZE: f32 = 60.0;
pub const GROUND: f32 = WINDOW_HEIGHT - UNIT_SIZE * 2.0;
pub const GRAVITY_CONSTANT: f32 = 50.0;

pub mod marcus {
    pub const MARCUS_ANIMATION_LENGTH: f32 = 3.0;
    pub const MARCUS_X_ACCELERATION: f32 = 2.0;
    pub const MARCUS_X_DECELERATION: f32 = 4.0;
    pub const MARCUS_X_MAX_SPEED: f32 = 50.0;
    pub const MARCUS_X_SPEED_MUL: f32 = 10.0;
    pub const MARCUS_Y_SPEED_MUL: f32 = 20.0;
    pub const MARCUS_JUMP_POWER: f32 = 25.0;
}

pub mod dialog {
    pub const DIALOG_ANIMATION_LENGTH: f32 = 1.0;
}
