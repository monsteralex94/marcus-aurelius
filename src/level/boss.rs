use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum BossActions {
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

#[derive(Serialize, Deserialize)]
struct Boss {
    at_start: Vec<BossActions>,
    always: Vec<BossActions>,

    // When hit with a certain power
    when_hit: Vec<Vec<BossActions>>,
}
