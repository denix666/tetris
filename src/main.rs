use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

mod points;
use points::Point;

mod shape;
use shape::Shape;

mod resources;
use resources::Resources;

fn draw_board(points: &Vec<Point>, resources: &Resources) {
    for point in points {
        match point.value.as_str() {
            "#" => {
                draw_texture(resources.border, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "L" => {
                draw_texture(resources.l, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "Z" => {
                draw_texture(resources.z, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "I" => {
                draw_texture(resources.i, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "T" => {
                draw_texture(resources.t, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "O" => {
                draw_texture(resources.o, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "J" => {
                draw_texture(resources.j, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            "S" => {
                draw_texture(resources.s, 
                                   point.x as f32 * resources::BLOCKSIZE, 
                                   point.y as f32 * resources::BLOCKSIZE, WHITE);
            },
            _ => {},
        };
    }
}

fn make_board_array() -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for r in 0..=resources::NROWS {
        for c in 0..=resources::NCOLS {
            if c == 0 || c == resources::NCOLS || r == resources::NROWS {
                points.push(
                    Point::new(c,r,"#".to_string()),
                );
            } else {
                points.push(
                    Point::new(c,r," ".to_string()),
                );
            }
        }
    }

    return points
}

pub fn get_val(check_x: i32, check_y: i32, points: &Vec<Point>) -> String {
    let ret = match points.iter().position(|x| x.x == check_x && x.y == check_y) {
        Some(idx) => points[idx].value.to_string(),
        _ => String::from(" "),
    };
    ret
}

fn get_index(check_x: i32, check_y: i32, points: &Vec<Point>) -> usize {
    if let Some(idx) = points.iter().position(|x| x.x == check_x && x.y == check_y) {
        idx
    } else {
        0
    }
}

fn can_move(shape: &Shape, points: &Vec<Point>, dir: String) -> bool {
    let mut ret: bool = false;
    let mut x_shift: i32 = 0;
    let mut y_shift: i32 = 0;

    match dir.to_string().as_str() {
        "left" => {x_shift = -1},
        "right" => {x_shift = 1},
        _ => {y_shift = 1},
    }

    for i in shape.shape_structure {
        let x: i32 = (shape.x / resources::BLOCKSIZE) as i32 + i[0] + x_shift;
        let y: i32 = (shape.y / resources::BLOCKSIZE) as i32 + i[1] + y_shift;
        
        if get_val(x, y, &points) != " " {
            ret = false;
            break;
        } else {
            ret = true;
        }
    }
    ret
}

fn window_conf() -> Conf {
    let mut title = String::from("Tetris v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::RES_WIDTH,
        window_height: resources::RES_HEIGHT,
        ..Default::default()
    }
}

pub enum GameState {
    Game,
}

#[macroquad::main(window_conf)]
async fn main() {
    let game_state = GameState::Game;
    let mut points: Vec<Point> = make_board_array(); // do not forget!!!!!!!
    let mut shapes: Vec<Shape> = Vec::new();
    let resources = Resources::new().await;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Game => {
                draw_board(&points, &resources);
                
                if shapes.len() == 0 {
                    let shape_type = match rand::thread_rng().gen_range(0..=6) { 
                        0 => "L",
                        1 => "Z",
                        2 => "I",
                        3 => "T",
                        4 => "O",
                        5 => "J",
                        _ => "S",
                    };
                    shapes.push(
                        //Shape::new(7.0 * resources::BLOCKSIZE, 1.0 * resources::BLOCKSIZE, "L", &resources).await,
                        Shape::new(7.0 * resources::BLOCKSIZE, 1.0 * resources::BLOCKSIZE, shape_type, &resources).await,
                    );
                }

                for shape in &mut shapes {
                    if is_key_down(KeyCode::Right) {
                        if can_move(shape, &points, "right".to_string()) {
                            shape.move_right();
                        }
                    }
    
                    if is_key_down(KeyCode::Left) {
                        if can_move(shape, &points, "left".to_string()) {
                            shape.move_left();
                        }
                    }

                    if is_key_pressed(KeyCode::Up) {
                        let mut i: usize = 0;
                        let tmp_x: i32 = shape.shape_structure[0][1];
                        let tmp_y: i32 = shape.shape_structure[0][0];
                        
                        for _ in 0..3 {
                            shape.shape_structure[i][0] = shape.shape_structure[i+1][1];
                            shape.shape_structure[i][1] = shape.shape_structure[i+1][0];
                            i += 1;
                        }
                        shape.shape_structure[3][0] = tmp_x;
                        shape.shape_structure[3][1] = tmp_y; 
                    }

                    if can_move(shape, &points, "down".to_string()) {
                        shape.move_down();
                    } else {
                        for i in shape.shape_structure {
                            let x: i32 = (shape.x / resources::BLOCKSIZE) as i32 + i[0];
                            let y: i32 = (shape.y / resources::BLOCKSIZE) as i32 + i[1];
                            
                            let idx = get_index(x, y, &points);
                            points[idx].value = shape.shape_type.to_string();
                        }
                        
                        shape.destroyed = true;
                    }

                    shape.draw();
                }
            }
        }

        // GC
        match shapes.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                shapes.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}
