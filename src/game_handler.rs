use crate::games::{avoider::Avoider, circles::Circles, pong::Pong};
use rand::prelude::*;
use raylib::prelude::*;
use std::fs;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Read};
use yaml_rust2::{YamlEmitter, YamlLoader};

fn draw_title(d: &mut RaylibDrawHandle, title: &str) {
    d.draw_text("current game:", 9, 9, 30, Color::WHITE);
    d.draw_text("current game:", 11, 11, 30, Color::WHITE);
    d.draw_text("current game:", 10, 10, 30, Color::BLACK);
    d.draw_text(title, 9, 39, 50, Color::WHITE);
    d.draw_text(title, 11, 41, 50, Color::WHITE);
    d.draw_text(title, 10, 40, 50, Color::BLACK);
}

pub struct GameHandler {
    now: SystemTime,
    cooldown: Duration,
    game_list: Vec<Box<dyn Game>>,
    selected: Option<usize>,
}

impl GameHandler {
    pub fn new(screen_size: (i32, i32)) -> Result<GameHandler, Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or("Could not get executable directory")?;
        let settings_path = exe_dir.join("settings.yaml");

        // Check if file exists
        if !settings_path.exists() {
            return Err("settings.yaml not found".into());
        }

        let mut file = File::open(&settings_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let temp_settings = YamlLoader::load_from_str(&contents)?;
        if temp_settings.is_empty() {
            return Err("YAML file is empty".into());
        }

        let settings = &temp_settings[0];

        // Validate required settings exist
        let game_settings = settings["Settings"]
            .as_vec()
            .ok_or("Game Settings not found or invalid")?;

        let circles_settings = settings["Circles"]
            .as_vec()
            .ok_or("Circles settings not found or invalid")?;

        let avoider_settings = settings["Avoider"]
            .as_vec()
            .ok_or("Avoider settings not found or invalid")?;

        Ok(GameHandler {
            now: SystemTime::now(),
            cooldown: Duration::from_secs_f32(
                game_settings[0]
                    .as_f64()
                    .ok_or("Invalid Game Settings cooldown")? as f32,
            ),
            game_list: vec![
                Box::new(Pong::new(screen_size)),
                Box::new(Circles::new(
                    screen_size,
                    circles_settings[1]
                        .as_i64()
                        .ok_or("Invalid Circles min_amount")? as i32,
                    circles_settings[2]
                        .as_i64()
                        .ok_or("Invalid Circles max_amount")? as i32,
                    Duration::from_secs_f32(
                        circles_settings[3]
                            .as_f64()
                            .ok_or("Invalid Circles time_limit")? as f32,
                    ),
                )),
                Box::new(Avoider::new(
                    screen_size,
                    Duration::from_secs_f32(
                        avoider_settings[1]
                            .as_f64()
                            .ok_or("Invalid Avoider parameter 1")? as f32,
                    ),
                    Duration::from_secs_f32(
                        avoider_settings[2]
                            .as_f64()
                            .ok_or("Invalid Avoider parameter 2")? as f32,
                    ),
                )),
            ],
            selected: None,
        })
    }

    pub fn ready(&mut self) -> bool {
        if self.now.elapsed().unwrap() > self.cooldown {
            true
        } else {
            false
        }
    }

    pub fn select_game(&mut self) {
        let mut rng = rand::rng(); // use correct thread_rng
        let num = rng.random_range(0..self.game_list.len());
        self.selected = Some(num);
        println!("selected: {num}");
    }

    pub fn do_frame(&mut self, delta_time: f32, mouse_pos: Vector2, d: &mut RaylibDrawHandle) {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            game.logic(mouse_pos, delta_time);
            game.draw(d);
            draw_title(d, game.get_info());
        }
    }

    pub fn finished(&mut self) -> Option<bool> {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            if let Some(game_over) = game.is_finished() {
                self.now = SystemTime::now();
                self.select_game();
                return Some(game_over);
            }
        }
        None
    }
}

pub trait Game {
    // gets name of the game
    fn get_info(&mut self) -> &str;
    // main logic of the game
    fn logic(&mut self, mouse_pos: Vector2, delta_time: f32);
    // drawing the frame of the game
    fn draw(&mut self, d: &mut RaylibDrawHandle);
    // None if still going else return if lost
    fn is_finished(&mut self) -> Option<bool>;
}
