use macroquad::{prelude::*, audio::{Sound, load_sound}};

// window size in pixels
pub const RES_WIDTH: i32 = 720;
pub const RES_HEIGHT: i32 = 540;

// rows and columns sizes
pub const NCOLS: i32 = 16; // X
pub const NROWS: i32 = 17; // Y

// cell size in pixels
pub const BLOCKSIZE: f32 = 30.0;

// init falling speed
pub const INIT_SPEED: f64 = 0.9;

// shape movement delay
pub const THREAD_SLEEP: u64 = 90;

pub struct Resources {
    pub border: Texture2D,
    pub l: Texture2D,
    pub z: Texture2D,
    pub i: Texture2D,
    pub t: Texture2D,
    pub o: Texture2D,
    pub j: Texture2D,
    pub s: Texture2D,
    pub font: Font,
    pub intro: Texture2D,
    pub music: Sound,
    pub pause: Sound,
    pub removed_1_line: Sound,
    pub removed_4_lines: Sound,
    pub level_up: Sound,
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
            font: load_ttf_font("assets/fonts/game_font.ttf").await.unwrap(),
            intro: load_texture("assets/images/intro.png").await.unwrap(),
            music: load_sound("assets/music/game_loop.ogg").await.unwrap(),
            pause: load_sound("assets/sounds/pause.ogg").await.unwrap(),
            removed_1_line: load_sound("assets/sounds/removed_1_line.ogg").await.unwrap(),
            removed_4_lines: load_sound("assets/sounds/removed_4_lines.ogg").await.unwrap(),
            level_up: load_sound("assets/sounds/level_up.ogg").await.unwrap(),
        }
    }
}