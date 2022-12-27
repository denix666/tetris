use macroquad::prelude::*;
use crate::resources::{self};

pub struct Game {
    pub falling_speed: f64
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            falling_speed: resources::INIT_SPEED,
        }
    }
}
