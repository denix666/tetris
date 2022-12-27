use std::{thread, time::Duration};
use macroquad::prelude::*;
use crate::resources::{self, Resources};

pub struct Shape {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    texture: Texture2D,
    pub shape_structure: [[i32; 2]; 4],
    pub shape_type: String,
    last_update_time: f64,
}

impl Shape {
    pub async fn new(x: f32, y: f32, shape_type: &str, resources: &Resources) -> Self {
        let structure: [[i32; 2]; 4];
        let sprite: Texture2D;
        let t: String;
        
        match shape_type {
            "L" => { // L - white
                sprite = resources.l;
                //structure = [[-1, 0],[0, 0],[1, 0],[1, -1]];
                //structure = [[0, -1],[0, 0],[0, 1],[1, 1]];
                structure = [[-1, 1],[-1, 0],[0, 0],[1, 0]];
                //structure = [[-1, -1],[0, -1],[0, 0],[0, 1]];
                t = "L".to_string();
            },
            "Z" => { // Z - red
                sprite = resources.z;
                structure = [[0, -1],[0, 0],[-1, 0],[-1, 1]];
                t = "Z".to_string();
            },
            "I" => { // I - pinc
                sprite = resources.i;
                structure = [[0, -1],[0, 0],[0, 1],[0, 2]];
                t = "I".to_string();
            },
            "T" => { // T - brown
                sprite = resources.t;
                structure = [[-1, 0],[0, 0],[1, 0],[0, 1]];
                t = "T".to_string();
            },
            "O" => { // O - green
                sprite = resources.o;
                structure = [[0, 0],[1, 0],[0, 1],[1, 1]];
                t = "O".to_string();
            },
            "J" => { // J - yellow
                sprite = resources.j;
                structure = [[1, -1],[0, -1],[0, 0],[0, 1]];
                t = "J".to_string();
            },
            _ => { // S blue
                sprite = resources.s;
                structure = [[0, -1],[0, 0],[1, 0],[1, 1]];
                t = "S".to_string();
            },
        }
        
        Self {
            x,
            y,
            destroyed: false,
            texture: sprite,
            shape_structure: structure,
            last_update_time: get_time(),
            shape_type: t,
        }
    }

    pub fn move_down(&mut self) {
        if get_time() - self.last_update_time >= resources::INIT_SPEED {
            self.y += 30.0;
            self.last_update_time = get_time();
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 30.0;
        thread::sleep(Duration::from_millis(resources::THREAD_SLEEP));
    }

    pub fn move_right(&mut self) {
        self.x += 30.0;
        thread::sleep(Duration::from_millis(resources::THREAD_SLEEP));
    }

    pub fn draw(&mut self) {
        for i in self.shape_structure {
            let x: f32 = self.x + i[0] as f32 * resources::BLOCKSIZE;
            let y: f32 = self.y + i[1] as f32 * resources::BLOCKSIZE;
            draw_texture(self.texture, x, y, WHITE);
        }
    }
}