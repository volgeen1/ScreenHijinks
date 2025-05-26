use crate::effects::{screen_flash::Flash};
use crate::util::Timer;
use rand::prelude::*;
use raylib::prelude::RaylibDrawHandle;
use std::time::Duration;

pub struct LossHandler {
    effect_list: Vec<Box<dyn Effect>>,
    selected: Option<usize>,
    finished: bool
}

impl LossHandler {
    pub fn new(screen_size: (i32, i32)) -> LossHandler {
        LossHandler {
            effect_list: vec![Box::new(Flash::new(screen_size, Duration::from_secs(5)))],
            selected: None,
            finished: true,
        }
    }

    pub fn select_effect(&mut self) {
        let mut rng = rand::rng();
        let num = rng.random_range(0..self.effect_list.len());
        self.selected = Some(num);
        println!("selected effect: {num}");
    }

    pub fn do_effect(&mut self, d: &mut RaylibDrawHandle, delta_time: f32) {
        if let Some(index) = self.selected {
            self.finished = false;
            let effect = &mut self.effect_list[index];
            effect.draw(d, delta_time);
        }
    }

    pub fn finished(&mut self) -> bool {
        if let Some(index) = self.selected {
            let effect = &mut self.effect_list[index];
            let effect_over = effect.is_finished();
            if effect_over {
                self.select_effect();
                self.finished = true;
            }
        }
        self.finished
    }
}

pub trait Effect {
    // draw the effect
    fn draw(&mut self, d: &mut RaylibDrawHandle, delta_time: f32);
    // returns true when effect is finished
    fn is_finished(&mut self) -> bool;
}