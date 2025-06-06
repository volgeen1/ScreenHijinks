use crate::games::{avoider::Avoider, circles::Circles, pong::Pong};
use rand::prelude::*;
use raylib::prelude::*;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Read};
use yaml_rust2::YamlLoader;

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

        let mut contents = String::new();
        // Check if file exists
        if !settings_path.exists() {
            println!("no settings file. creating one");
                        let settings_content = 
r#"Settings:
    - 120.0    # Time between games (seconds)
            
Pong:
  - true    # Enable/disable game
  - 300.0   # Ball speed
  - 200.0   # Paddle speed
  - 200.0   # AI paddle speed

Circles:
  - true    # Enable/disable game
  - 4       # Minimum circles
  - 10      # Maximum circles
  - 5.0     # Time limit (seconds)

Avoider:
  - true    # Enable/disable game
  - 15.0    # Time limit (seconds)
  - 0.6     # Object spawn interval (seconds)
"#;
            std::fs::write(&settings_path, settings_content)?;
            contents.push_str(settings_content);
        } else {
            println!("settings file found");
            let mut file = File::open(&settings_path)?;
            file.read_to_string(&mut contents)?;
        }

        let temp_settings = YamlLoader::load_from_str(&contents)?;
        if temp_settings.is_empty() {
            return Err("YAML file is empty".into());
        }

        let settings = &temp_settings[0];

        // Validate required settings exist
        let game_settings = settings["Settings"]
            .as_vec()
            .ok_or("Game Settings not found or invalid")?;

        let pong_settings = settings["Pong"]
            .as_vec()
            .ok_or("Pong settings not found or invalid")?;

        let circles_settings = settings["Circles"]
            .as_vec()
            .ok_or("Circles settings not found or invalid")?;

        let avoider_settings = settings["Avoider"]
            .as_vec()
            .ok_or("Avoider settings not found or invalid")?;

        let mut games: Vec<Box<dyn Game>> = vec![];
        if pong_settings[0].as_bool().unwrap() {
            games.push(Box::new(Pong::new(screen_size, pong_settings)));
        }
        if circles_settings[0].as_bool().unwrap() {
            games.push(Box::new(Circles::new(screen_size, circles_settings)));
        }
        if avoider_settings[0].as_bool().unwrap() {
            games.push(Box::new(Avoider::new(screen_size, avoider_settings)));
        }

        Ok(GameHandler {
            now: SystemTime::now(),
            cooldown: Duration::from_secs_f32(
                game_settings[0]
                    .as_f64()
                    .ok_or("Invalid Game Settings cooldown")? as f32,
            ),
            game_list: games,
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
