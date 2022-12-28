#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

mod points;
use points::Point;

mod game;
use game::Game;

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

    for x in 17..24 {
        for y in 0..18 {
            if y > 1 && y < 7 && x >= 17 && x < 23 || // Next shape window
               y > 9 && y < 12 && x >= 17 && x < 23 || // Score window
               y > 12 && y < 15 && x >= 17 && x < 23 // Level window
            {
                points.push(
                    Point::new(x,y," ".to_string()),
                );
            } else {
                points.push(
                    Point::new(x,y,"#".to_string()),
                );
            }
        }
    }

    return points
}

pub fn get_val(check_x: i32, check_y: i32, points: &Vec<Point>) -> String {
    let ret = match points.iter().position(|x| x.x == check_x && x.y == check_y) {
        Some(idx) => points[idx].value.to_string(),
        _ => String::from("#"),
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

fn can_rotate(shape: &Shape, points: &Vec<Point>) -> bool {
    let mut ret: bool = false;
    let mut x_shift: i32 = 0;

    match shape.shape_type.to_string().as_str() {
        "I" => {
            if shape.rotation_index == 1 || shape.rotation_index == 3 {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = 0;
                } else {
                    x_shift = 0;
                }
            } else {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = -1;
                } else {
                    x_shift = 2;
                }
            }
            
        },
        "L" => {
            if shape.rotation_index == 1 || shape.rotation_index == 3 {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = 0;
                } else {
                    x_shift = 0;
                }
            } else {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = -1;
                } else {
                    x_shift = 1;
                }
            }
            
        },
        "J" => {
            if shape.rotation_index == 1 || shape.rotation_index == 3 {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = 1;
                } else {
                    x_shift = 1;
                }
            } else {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = -1;
                } else {
                    x_shift = 1;
                }
            }
            
        },
        "S" | "O" | "T" | "Z" => {
            if shape.rotation_index == 1 || shape.rotation_index == 3 {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = 0;
                } else {
                    x_shift = 0;
                }
            } else {
                if shape.x < 5.0 * resources::BLOCKSIZE {
                    x_shift = -1;
                } else {
                    x_shift = 1;
                }
            }
        },
        _ => {}
    }
    
    for i in shape.shape_structure {
        let x: i32 = (shape.x / resources::BLOCKSIZE) as i32 + i[0] + x_shift;
        let y: i32 = (shape.y / resources::BLOCKSIZE) as i32 + i[1];

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
    InitLevel,
    Game,
    LevelFail,
}

fn select_shape() -> &'static str {
    let shape_type = match rand::thread_rng().gen_range(0..=6) { 
        0 => "L",
        1 => "Z",
        2 => "I",
        3 => "T",
        4 => "O",
        5 => "J",
        _ => "S",
    };
    return shape_type
}

pub fn draw_info(font: Font, score: &str, level: &str) {
    draw_text_ex("NEXT: ", 560.0, 90.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );
    
    draw_text_ex("SCORE: ", 520.0, 335.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 620.0, 335.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("LEVEL: ", 520.0, 425.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(level, 620.0, 425.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::InitLevel;
    let mut points: Vec<Point> = make_board_array();
    let mut shapes: Vec<Shape> = Vec::new();
    let resources = Resources::new().await;
    let mut game = Game::new().await;
    let mut next_shape = select_shape();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::InitLevel => {
                game.level = 1;
                game.score = 0;
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_board(&points, &resources);
                draw_info(resources.font, game.score.to_string().as_str(), game.level.to_string().as_str());
                
                if shapes.len() == 0 {
                    let shape_type = next_shape;
                    next_shape = select_shape();
                    shapes.push(
                        // DEBUG
                        //Shape::new(7.0 * resources::BLOCKSIZE, 1.0 * resources::BLOCKSIZE, "J", &resources).await,
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

                    if is_key_pressed(KeyCode::Space) {
                        while can_move(shape, &points, "down".to_string()) {
                            shape.move_down(0.0);
                        }
                    }

                    if is_key_pressed(KeyCode::Up) {
                        if can_rotate(shape, &points) {
                            if shape.rotation_index == 3 {
                                shape.rotation_index = 0;
                            } else {
                                shape.rotation_index += 1;
                            }
                        }
                    }

                    if is_key_down(KeyCode::Down) {
                        game.falling_speed = 0.1;
                    } else {
                        game.falling_speed = resources::INIT_SPEED;
                    }

                    if can_move(shape, &points, "down".to_string()) {
                        shape.move_down(game.falling_speed);
                    } else {
                        // Достигли потолка
                        if ((shape.y / resources::BLOCKSIZE) as i32) < 2 {
                            game_state = GameState::LevelFail;
                        }

                        // Перенос блока в массив
                        for i in shape.shape_structure {
                            let x: i32 = (shape.x / resources::BLOCKSIZE) as i32 + i[0];
                            let y: i32 = (shape.y / resources::BLOCKSIZE) as i32 + i[1];
                            
                            let idx = get_index(x, y, &points);
                            points[idx].value = shape.shape_type.to_string();
                        }
                        
                        shape.destroyed = true;

                        let mut line_filled: bool = false;
                        let mut lines_removed: i32 = 0;
                        for r in 2..resources::NROWS {
                            for c in 1..resources::NCOLS {
                                if get_val(c, r, &points) != " " {
                                    line_filled = true;
                                } else {
                                    line_filled = false;
                                    break;
                                }
                            }
                            if line_filled {
                                for c in 1..resources::NCOLS {
                                    let mut line = r; 
                                    while line > 0 {
                                        let prev_value = get_val(c, line - 1, &points);
                                        let idx = get_index(c, line, &points);
                                        points[idx].value = prev_value;
                                        line -= 1;
                                    }
                                }
                                lines_removed += 1;
                                line_filled = false;
                            }
                        }
                        match lines_removed {
                            1 => {
                                game.score += 10;
                            },
                            2 => {
                                game.score += 30;
                            },
                            3 => {
                                game.score += 50;
                            },
                            4 => {
                                game.score += 100;
                            },
                            _ => {
                                game.score += 1;
                            }
                        }
                    }

                    shape.draw();
                }
            },
            GameState::LevelFail => {
                draw_board(&points, &resources);
                for shape in &mut shapes {
                    shape.draw();
                }
            },
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
