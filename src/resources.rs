use macroquad::prelude::*;

// размер всего окна игры
pub const RES_WIDTH: i32 = 510;
pub const RES_HEIGHT: i32 = 540;

pub const NCOLS: i32 = 16; // X
pub const NROWS: i32 = 17; // Y

// размер блока в пикселях
pub const BLOCKSIZE: f32 = 30.0;

pub const INIT_SPEED: f64 = 0.3;

pub const THREAD_SLEEP: u64 = 60;

pub struct Resources {
    pub border: Texture2D,
    pub l: Texture2D,
    pub z: Texture2D,
    pub i: Texture2D,
    pub t: Texture2D,
    pub o: Texture2D,
    pub j: Texture2D,
    pub s: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            border: load_texture("assets/images/border.png").await.unwrap(),
            l: load_texture("assets/images/white.png").await.unwrap(),
            z: load_texture("assets/images/red.png").await.unwrap(),
            i: load_texture("assets/images/pinc.png").await.unwrap(),
            t: load_texture("assets/images/brown.png").await.unwrap(),
            o: load_texture("assets/images/green.png").await.unwrap(),
            j: load_texture("assets/images/yellow.png").await.unwrap(),
            s: load_texture("assets/images/blue.png").await.unwrap(),
        }
    }
}