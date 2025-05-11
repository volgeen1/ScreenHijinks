use crate::games::pong::Pong;
use rand:: prelude::*;
use raylib::prelude::{RaylibDrawHandle, Rectangle};
use std::time::{Duration, SystemTime};

pub struct GameHandler {
    now: SystemTime,
    cooldown: Duration,
    game_list: Vec<Box<dyn Game>>,
    selected: Option<usize>,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler {
            now: SystemTime::now(),
            cooldown: Duration::from_secs(5),
            game_list: vec![Box::new(Pong::new())],
            selected: None,
        }
    }

    pub fn ready(&mut self) -> bool {
        if self.now.elapsed().unwrap() > self.cooldown {
            true
        }else {
            false
        }
        
    }

    pub fn select_game(&mut self) {
        let mut rng = rand::rng(); // use correct thread_rng
        let num = rng.random_range(0..self.game_list.len());
        self.selected = Some(num);
        
    }

    pub fn start_game(&mut self, rect: Rectangle) {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            game.init(rect);
        }
    }

    pub fn do_frame(&mut self, delta_time: f32, d: RaylibDrawHandle) {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            game.logic(delta_time);
            game.draw(d);
        }
    }

    pub fn finished(&mut self) -> bool {
        if let Some(index) = self.selected {
            let game = &mut self.game_list[index];
            if game.is_finished() {
                self.now = SystemTime::now();
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
    fn init(&mut self, rect: Rectangle);
    fn logic(&mut self, delta_time: f32);
    fn draw(&mut self, d: RaylibDrawHandle);
    fn is_finished(&mut self) -> bool;
}
