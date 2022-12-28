use macroquad::prelude::*;
use crate::resources::{self};

pub struct Game {
    pub falling_speed: f64,
    pub score: i32,
    pub level: i32,
    pub lines_removed: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            falling_speed: resources::INIT_SPEED,
            score: 0,
            level: 0,
            lines_removed: 0,
        }
    }
}
