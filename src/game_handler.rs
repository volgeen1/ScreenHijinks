use crate::games::{avoider::Avoider, circles::Circles, pong::Pong};
use rand::prelude::*;
use raylib::prelude::*;
use std::time::{Duration, SystemTime};

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
    pub fn new(screen_size: (i32, i32)) -> GameHandler {
        GameHandler {
            now: SystemTime::now(),
            cooldown: Duration::from_secs(5),
            game_list: vec![
                Box::new(Pong::new(screen_size)),
                Box::new(Circles::new(screen_size, 4, 10, Duration::from_secs(5))),
                Box::new(Avoider::new(screen_size, Duration::from_secs(15), Duration::from_millis(600))),
            ],
            selected: None,
        }
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

    pub fn finished(&mut self) -> bool {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            let game_over = game.is_finished();
            if game_over.0 {
                self.now = SystemTime::now();
                self.select_game();
                return true;
            }
            false
        } else {
            println!("error");
            false
        }
    }
}

pub trait Game {
    // gets name of the game
    fn get_info(&mut self) -> &str;
    // main logic of the game
    fn logic(&mut self, mouse_pos: Vector2, delta_time: f32);
    // drawing the frame of the game
    fn draw(&mut self, d: &mut RaylibDrawHandle);
    // returns 2 bools, (over?, lost?)
    fn is_finished(&mut self) -> (bool, bool);
}
