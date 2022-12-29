#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;

use macroquad::{prelude::*, audio::{play_sound, PlaySoundParams, stop_sound}};
extern crate rand;
use rand::Rng;
use egui_macroquad::egui::{self, RichText, Color32};

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
            if y >= 1 && y < 7 && x >= 17 && x < 23 || // Next shape window
               y > 7 && y < 10 && x >= 17 && x < 23 || // Score window
               y > 10 && y < 13 && x >= 17 && x < 23 || // Level window
               y > 13 && y < 16 && x >= 17 && x < 23 // Lines removed window
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
    Intro,
    InitLevel,
    Game,
    LevelFail,
    Paused,
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

fn show_text(font: Font, header_text: &str, message_text: &str) {
    let header_dims = measure_text(header_text, Some(font), 60, 1.0);
    let message_dims = measure_text(message_text, Some(font), 23, 1.0);

    draw_text_ex(
        &header_text,
        290.0 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 50,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        260.0 - message_dims.width * 0.5,
        280.0,
        TextParams {
            font,
            font_size: 23,
            color: ORANGE,
            ..Default::default()
        },
    );
}

pub fn draw_info(font: Font, score: &str, level: &str, lines_removed: &str) {
    draw_text_ex("NEXT: ", 560.0, 60.0, 
        TextParams {
            font,
            font_size: 25,
            color: WHITE,
            ..Default::default()
        },
    );
    
    draw_text_ex("SCORE: ", 520.0, 275.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 620.0, 275.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("LEVEL: ", 520.0, 365.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(level, 620.0, 365.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("LINES: ", 520.0, 455.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(lines_removed, 620.0, 455.0, 
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
    let mut game_state = GameState::Intro;
    let mut points: Vec<Point> = make_board_array();
    let mut shapes: Vec<Shape> = Vec::new();
    let mut next_shapes: Vec<Shape> = Vec::new();
    let resources = Resources::new().await;
    let mut game = Game::new().await;
    let mut next_shape = select_shape();
    let mut sound_enabled: bool = true;
    let mut sound_volume_level: f32 = 0.2;
    let mut music_enabled: bool = true;
    let mut music_volume_level: f32 = 0.06;
    let mut music_started: bool = false;
    let mut previous_level_num = 1;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro, 0.0, 0.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::InitLevel;
                }
            },
            GameState::InitLevel => {
                points.clear();
                points = make_board_array();
                shapes.clear();
                next_shapes.clear();
                game.level = 1;
                game.score = 0;
                game.lines_removed = 0;
                game_state = GameState::Game;
                music_started = false;
            },
            GameState::Game => {
                if music_enabled && !music_started {
                    play_sound(resources.music, PlaySoundParams {
                        looped: true,
                        volume: music_volume_level,
                    });
                    music_started = true;
                }
                
                if is_key_pressed(KeyCode::Escape) {
                    stop_sound(resources.music);
                    game_state = GameState::Paused;
                    if sound_enabled {
                        play_sound(resources.pause, PlaySoundParams {
                            looped: false,
                            volume: sound_volume_level,
                        });
                    }
                }
                draw_board(&points, &resources);
                draw_info(resources.font, 
                          game.score.to_string().as_str(), 
                          game.level.to_string().as_str(), 
                          game.lines_removed.to_string().as_str());
                
                if shapes.len() == 0 {
                    let shape_type = next_shape;
                    next_shape = select_shape();
                    
                    shapes.push(
                        Shape::new(7.0 * resources::BLOCKSIZE, 1.0 * resources::BLOCKSIZE, shape_type, &resources).await,
                    );
                    shapes[0].i_am_next_shape = false;

                    next_shapes.clear();
                    next_shapes.push(
                        Shape::new(0.0, 0.0, next_shape, &resources).await,
                    );
                }

                for next in &mut next_shapes {
                    next.draw();
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

                    if can_move(shape, &points, "down".to_string()) {
                        if is_key_down(KeyCode::Down) {
                            shape.move_down(0.1);
                        } else {
                            shape.move_down(game.falling_speed);
                        }
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
                                if sound_enabled {
                                    play_sound(resources.removed_1_line, PlaySoundParams {
                                        looped: false,
                                        volume: sound_volume_level,
                                    });
                                }
                            },
                            2 => {
                                game.score += 30;
                                if sound_enabled {
                                    play_sound(resources.removed_1_line, PlaySoundParams {
                                        looped: false,
                                        volume: sound_volume_level,
                                    });
                                }
                            },
                            3 => {
                                game.score += 50;
                                if sound_enabled {
                                    play_sound(resources.removed_1_line, PlaySoundParams {
                                        looped: false,
                                        volume: sound_volume_level,
                                    });
                                }
                            },
                            4 => {
                                game.score += 100;
                                if sound_enabled {
                                    play_sound(resources.removed_4_lines, PlaySoundParams {
                                        looped: false,
                                        volume: sound_volume_level,
                                    });
                                }
                            },
                            _ => {
                                game.score += 1;
                            }
                        }
                        game.lines_removed += lines_removed;
                    }

                    match game.lines_removed {
                        0..=10 => {
                            game.level = 1;
                            game.falling_speed = resources::INIT_SPEED;
                        },
                        11..=30 => {
                            game.level = 2;
                            game.falling_speed = 0.8;
                        },
                        31..=50 => {
                            game.level = 3;
                            game.falling_speed = 0.7;
                        },
                        51..=70 => {
                            game.level = 4;
                            game.falling_speed = 0.6;
                        },
                        71..=80 => {
                            game.level = 5;
                            game.falling_speed = 0.5;
                        },
                        81..=90 => {
                            game.level = 6;
                            game.falling_speed = 0.4;
                        },
                        91..=100 => {
                            game.level = 7;
                            game.falling_speed = 0.3;
                        },
                        _ => {
                            game.level = 8;
                            game.falling_speed = 0.2;
                        }
                    }

                    if game.level != previous_level_num {
                        previous_level_num = game.level;
                        if sound_enabled {
                            play_sound(resources.level_up, PlaySoundParams {
                                looped: false,
                                volume: sound_volume_level,
                            });
                        }
                    }

                    shape.draw();
                }
            },
            GameState::LevelFail => {
                stop_sound(resources.music);
                draw_board(&points, &resources);
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.level.to_string().as_str(), 
                    game.lines_removed.to_string().as_str());
                
                for shape in &mut shapes {
                    shape.draw();
                }

                show_text(resources.font, "Game over", "Press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::InitLevel;
                }
            },
            GameState::Paused => {
                draw_board(&points, &resources);
                draw_info(resources.font, 
                    game.score.to_string().as_str(), 
                    game.level.to_string().as_str(), 
                    game.lines_removed.to_string().as_str());

                for next in &mut next_shapes {
                    next.draw();
                }

                for shape in &mut shapes {
                    shape.draw();
                }

                show_text(resources.font, "PAUSED", "Press 'space' to continue...");
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("Settings").current_pos([125.0, 40.0]).show(egui_ctx, |ui| {
                        ui.checkbox(&mut sound_enabled, "sound");
                        ui.add(egui::Slider::new(&mut sound_volume_level, 0.0..=0.9).text("Sound volume level"));

                        ui.checkbox(&mut music_enabled, "music");
                        ui.add(egui::Slider::new(&mut music_volume_level, 0.0..=0.9).text("Music volume level"));
                        
                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                game_state = GameState::Game;
                                if music_enabled {
                                    play_sound(resources.music, PlaySoundParams {
                                        looped: true,
                                        volume: music_volume_level,
                                    });
                                }
                            }
                            if ui.button(RichText::new("Quit game").color(Color32::RED)).clicked() {
                                process::exit(0x0100);
                            }
                        }); 
                    });
                });
                    
                egui_macroquad::draw();

                if is_key_pressed(KeyCode::Space) | is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Game;
                    if music_enabled {
                        play_sound(resources.music, PlaySoundParams {
                            looped: true,
                            volume: music_volume_level,
                        });
                    }
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
