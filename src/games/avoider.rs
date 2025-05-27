use crate::game_handler::Game;
use crate::util::Timer;
use yaml_rust2::Yaml;
use rand::{Rng, rngs::ThreadRng};
use std::{ffi::CString, time::Duration};
use {raylib::ffi::MeasureText, raylib::prelude::*};

pub struct Avoider {
    player_pos: Vector2,
    player_size: f32,
    enemies: Vec<(Vector2, Vector2)>,
    enemy_size: f32,
    game_size: Rectangle,
    timer: Timer,
    spawn_timer: Timer,
    rng: ThreadRng,
    lost: bool
}

impl Avoider {
    pub fn new(screen_size: (i32, i32), settings: &Vec<Yaml>) -> Avoider {
        let game_rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: screen_size.0 as f32,
            height: screen_size.1 as f32,
        };
        
        let time_length = settings[1].as_f64().unwrap();
        let spawn_timer = settings[2].as_f64().unwrap();

        Avoider {
            player_pos: Vector2::new((screen_size.0 / 2) as f32, (screen_size.1 / 2) as f32),
            player_size: 20.0,
            enemies: vec![],
            enemy_size: 30.0,
            game_size: game_rect,
            timer: Timer::new(Duration::from_secs_f64(time_length)),
            spawn_timer: Timer::new(Duration::from_secs_f64(spawn_timer)),
            rng: rand::rng(),
            lost: false,
        }
    }

    fn place_enemy(&mut self, mouse_pos: Vector2) {
        // vec[(min_x, max_x), (min_y, max_y)]
        let sides: Vec<(f32, f32)> = vec![(self.game_size.x, self.game_size.width + self.game_size.x), (self.game_size.y, self.game_size.height + self.game_size.y)];

        // 0 top, 1 right, 2 down, 3 left.
        let start_side = self.rng.random_range(0..4);
        let start_cord =  self.rng.random_range(sides[start_side % 2].0..sides[start_side % 2].1);

        let mut start_pos = Vector2::zero();
        if start_side == 0 {
            start_pos.x = start_cord;
        } else if start_side == 1 {
            start_pos.x = self.game_size.width;
            start_pos.y = start_cord;
        } else if start_side == 2 {
            start_pos.x = start_cord;
            start_pos.y = self.game_size.height;
        } else if start_side == 3 {
            start_pos.y = start_cord;
        }        

        let direction = mouse_pos - start_pos;
        let direction = direction.normalized();

        self.enemies.push((start_pos, direction));
    }

    fn move_enemies(&mut self, delta_time: f32) {
        let enemy_speed = 500.0;
        let enemies = self.enemies.clone();
        let mut updated_enemies: Vec<(Vector2, Vector2)> = vec![];

        for &enemy in &enemies {
            if enemy.0.distance_to(self.player_pos) < self.player_size + self.enemy_size {
                self.lost = true;
            }
            if enemy.0.x > self.game_size.x + self.game_size.width ||
               enemy.0.x < self.game_size.x ||
               enemy.0.y > self.game_size.y + self.game_size.height ||
               enemy.0.y < self.game_size.y {
                continue;
            }
            let movement = enemy.1 * enemy_speed * delta_time;

            let new_pos = enemy.0 + movement;

            updated_enemies.push((new_pos, enemy.1));
        }
        (*self).enemies = updated_enemies;
    }

    fn avoider_logic(&mut self, mouse_pos: Vector2, delta_time: f32) {
        self.player_pos = self.player_pos.lerp(mouse_pos, delta_time * 15.0);

        self.timer.update(delta_time);
        self.spawn_timer.update(delta_time);

        if self.spawn_timer.is_finished() {
            self.spawn_timer.reset();
            self.place_enemy(mouse_pos);
        }
        self.move_enemies(delta_time);
    }

    fn draw_frame(&mut self, d: &mut RaylibDrawHandle) {
        for enemy in self.enemies.clone() {
            d.draw_circle(
                enemy.0.x as i32,
                enemy.0.y as i32,
                self.enemy_size,
                Color::BLACK,
            );
            d.draw_circle(
                enemy.0.x as i32,
                enemy.0.y as i32,
                self.enemy_size - 4.0,
                Color::RED,
            );
        }

        d.draw_circle(
            self.player_pos.x as i32,
            self.player_pos.y as i32,
            self.player_size,
            Color::BLACK,
        );
        d.draw_circle(
            self.player_pos.x as i32,
            self.player_pos.y as i32,
            self.player_size - 3.4,
            Color::GREENYELLOW,
        );

        unsafe {
            let time_left = self.timer.time_left();
            let text = CString::new(format!("{:?}", time_left as i32)).unwrap();
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2)
                    + 2,
                self.game_size.y as i32 + 11,
                50,
                Color::BLACK,
            );
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2)
                    - 2,
                self.game_size.y as i32 + 9,
                50,
                Color::BLACK,
            );
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.game_size.x * 2.0 + self.game_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2),
                self.game_size.y as i32 + 10,
                50,
                Color::RED,
            );
        }
    }
}

impl Game for Avoider {
    fn get_info(&mut self) -> &str {
        "Avoider" as &str
    }

    fn logic(&mut self, mouse_pos: Vector2, delta_time: f32) {
        self.avoider_logic(mouse_pos, delta_time);
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_frame(d);
    }

    fn is_finished(&mut self) -> Option<bool> {
        if self.timer.is_finished()  {
            (*self).timer.reset();
            self.enemies.clear();
            Some(false)
        } else if self.lost {
            self.lost = false;
            (*self).timer.reset();
            self.enemies.clear();
            Some(true)
        } else {
            None
        }
    }
}
