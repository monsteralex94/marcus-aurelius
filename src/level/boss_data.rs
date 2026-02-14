use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BossAction {
    Move {
        x: f32,
        y: f32, // interesting physics
        duration: f32,
    },
    Jump {
        power: f32,
    },
    Wait {
        time: f32,
    },
    Shoot {
        weapon: usize,
        angle: f32,
        start_speed: f32,
        acceleration: f32,
    },
    LoseLife {
        amount: f32,
    },
    Say {
        text: String,
        disappear_time: f32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BossActionSet {
    pub at_start: Vec<BossAction>,
    pub always: Vec<BossAction>,
    pub when_hit: Vec<Vec<BossAction>>, // When hit with a certain power
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BossData {
    pub texture_name: String,
    pub action_set: BossActionSet,
}
