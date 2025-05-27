use crate::{game_handler::Game, util::Timer};
use mki::Mouse;
use yaml_rust2::Yaml;
use rand::prelude::*;
use raylib::{ffi::MeasureText, prelude::*};
use std::{ffi::CString, time::Duration};

pub struct Circles {
    timer: Timer,
    amount: i32,
    max_amount: i32,
    circles: Vec<(Vector2, f32)>,
    game_size: Rectangle,
    lost: bool,
}

fn place_circles(amount: i32, rect: Rectangle) -> Vec<(Vector2, f32)> {
    let mut circle_vec: Vec<(Vector2, f32)> = Vec::new();
    let circle_size: f32 = 50.0;
    let mut rng = rand::rng();
    for _i in 0..amount {
        let mut random_x = rng.random_range(rect.x..rect.width);
        let mut random_y = rng.random_range(rect.y..rect.height);
        for circle in &circle_vec {
            if circle.0.distance_to(Vector2::new(random_x, random_y)) < circle_size * 2.0 {
                //new vector pos away from circle
                let dir = Vector2::new(random_x, random_y) - circle.0;
                let normalized = dir.normalized();
                random_x = circle.0.x + normalized.x * (circle_size * 2.0);
                random_y = circle.0.y + normalized.y * (circle_size * 2.0);
            }
        }
        circle_vec.push((Vector2::new(random_x, random_y), circle_size));
    }
    circle_vec
}

impl Circles {
    pub fn new(screen_size: (i32, i32), settings: &Vec<Yaml>) -> Circles {
        let game_rect = Rectangle {
            x: screen_size.0 as f32 * 0.1,
            y: screen_size.1 as f32 * 0.1,
            width: screen_size.0 as f32 * 0.8,
            height: screen_size.1 as f32 * 0.8,
        };
        
        let amount = settings[1].as_i64().unwrap() as i32;
        let max_amount = settings[2].as_i64().unwrap() as i32;
        let time = settings[3].as_f64().unwrap();
        let vec_circles = place_circles(amount, game_rect);

        Circles {
            timer: Timer::new(Duration::from_secs_f64(time)),
            amount: amount,
            max_amount: max_amount,
            circles: (vec_circles),
            game_size: game_rect,
            lost: false,
        }
    }

    fn circle_logic(&mut self, mouse_pos: Vector2, _delta_time: f32) {
        self.timer.update(_delta_time);

        if self.timer.is_finished() {
            self.lost = true;
        }

        if Mouse::Left.is_pressed() {
            self.circles
                .retain(|(pos, radius)| pos.distance_to(mouse_pos) > *radius);
        }
    }

    fn draw_frame(&mut self, d: &mut RaylibDrawHandle) {
        let cirlces = self.circles.clone();
        for circle in cirlces {
            d.draw_circle(circle.0.x as i32, circle.0.y as i32, circle.1, Color::RED);
        }
        unsafe {
            let time_left = self.timer.time_left();
            let text = CString::new(format!("{:?}", time_left as i32)).unwrap();
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2)
                    + 2,
                self.game_size.y as i32 + 1,
                50,
                Color::BLACK,
            );
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2)
                    - 2,
                self.game_size.y as i32 - 1,
                50,
                Color::BLACK,
            );
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2),
                self.game_size.y as i32,
                50,
                Color::RED,
            );
        }
    }
}

impl Game for Circles {
    fn get_info(&mut self) -> &str {
        "Circles" as &str
    }

    fn logic(&mut self, mouse_pos: Vector2, delta_time: f32) {
        self.circle_logic(mouse_pos, delta_time);
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_frame(d);
    }

    fn is_finished(&mut self) -> Option<bool> {
        if self.circles.len() < 1 || self.lost {
            let amount = (self.amount + 1).clamp(1, self.max_amount);
            self.amount = amount;
            self.circles = place_circles(amount, self.game_size);
            self.timer.reset();
            let lost = self.lost;
            self.lost = false;
            Some(lost)
        } else {
            None
        }
    }
}
