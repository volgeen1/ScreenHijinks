use crate::game_handler::Game;
use mki::Keyboard;
use yaml_rust2::Yaml;
use rand::prelude::*;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Pong {
    ball_size: f32,
    ball_pos: Vector2,
    ball_speed: Vector2,
    paddle1: Vector3,
    paddle2: Vector3,
    paddle_speed: f32,
    ai_paddle_speed: f32,
    game_size: Rectangle,
    screen_size: (i32, i32),
    pub finished: bool,
    lost: bool,
}

impl Pong {
    pub fn new(screen_size: (i32, i32), settings: &Vec<Yaml>) -> Pong {
        let game_size: (i32, i32) = (800, 400);
        let game_rect = Rectangle {
            x: ((screen_size.0 / 2) - (game_size.0 / 2)) as f32,
            y: ((screen_size.1 / 2) - (game_size.1 / 2)) as f32,
            width: game_size.0 as f32,
            height: game_size.1 as f32,
        };

        Pong {
            ball_size: 12.0,
            ball_pos: Vector2 {
                x: game_rect.x + (game_rect.width / 2.0),
                y: game_rect.y + (game_rect.height / 2.0),
            },
            ball_speed: Vector2 { x: settings[1].as_f64().unwrap() as f32 * 1.2, y: settings[1].as_f64().unwrap() as f32 },
            paddle1: Vector3 {
                x: 100.0,
                y: 0.0,
                z: 20.0,
            },
            paddle2: Vector3 {
                x: 100.0,
                y: 0.0,
                z: 20.0,
            },
            paddle_speed: settings[2].as_f64().unwrap() as f32,
            ai_paddle_speed: settings[3].as_f64().unwrap() as f32,
            game_size: game_rect,
            screen_size: screen_size,
            finished: false,
            lost: false,
        }
    }

    fn reset(&mut self) {
        self.ball_pos = Vector2 {
            x: self.game_size.x + (self.game_size.width / 2.0),
            y: self.game_size.y + (self.game_size.height / 2.0),
        };
        self.paddle1 = Vector3 {
            x: self.paddle1.x,
            y: 0.0,
            z: 20.0,
        };
        self.paddle2 = Vector3 {
            x: self.paddle2.x,
            y: 0.0,
            z: 20.0,
        };

        let mut rng = rand::rng();
        let game_size_x = rng.random_range(800..(self.screen_size.0 as f32 * 0.8) as i32);
        let game_size_y = rng.random_range(400..(self.screen_size.1 as f32 * 0.8) as i32);

        let game_rect = Rectangle {
            x: ((self.screen_size.0 / 2) - (game_size_x / 2)) as f32,
            y: ((self.screen_size.1 / 2) - (game_size_y / 2)) as f32,
            width: game_size_x as f32,
            height: game_size_y as f32,
        };

        self.game_size = game_rect;
        self.finished = false;
    }

    fn pong_logic(&mut self, delta_time: f32) {
        self.pong_ball(delta_time);

        // paddle1 controls
        if Keyboard::Up.is_pressed()
            && self.paddle1.y as i32 - self.paddle1.x as i32 / 2
                > self.game_size.height as i32 / 2 * -1
        {
            self.paddle1.y -= self.paddle_speed * delta_time;
        } else if Keyboard::Down.is_pressed()
            && self.paddle1.y as i32 + self.paddle1.x as i32 / 2 < self.game_size.height as i32 / 2
        {
            self.paddle1.y += self.paddle_speed * delta_time;
        }

        // paddle2 ai
        if self.ball_pos.y < self.paddle2.y + self.game_size.y + (self.game_size.height / 2.0) {
            self.paddle2.y -= self.ai_paddle_speed * delta_time;
        } else if self.ball_pos.y
            > self.paddle2.y + self.game_size.y + (self.game_size.height / 2.0)
        {
            self.paddle2.y += self.ai_paddle_speed * delta_time;
        }
    }

    fn pong_ball(&mut self, delta_time: f32) {
        self.ball_pos.x += self.ball_speed.x * delta_time;
        self.ball_pos.y += self.ball_speed.y * delta_time;
        let paddle1rec = Rectangle::new(
            self.game_size.x + 15.0,
            self.game_size.y + self.game_size.height / 2.0 + self.paddle1.y - self.paddle1.x / 2.0,
            self.paddle1.z,
            self.paddle1.x,
        );
        let paddle2rec = Rectangle::new(
            self.game_size.x + self.game_size.width - 15.0 - 20.0,
            self.game_size.y + self.game_size.height / 2.0 + self.paddle2.y - self.paddle2.x / 2.0,
            self.paddle2.z,
            self.paddle2.x,
        );

        let ball_speed = 300.0;
        if paddle1rec.check_collision_circle_rec(self.ball_pos, self.ball_size) {
            self.ball_speed.x = ball_speed;
        } else if paddle2rec.check_collision_circle_rec(self.ball_pos, self.ball_size) {
            self.ball_speed.x = ball_speed * -1.0;
        } else if self.ball_pos.x >= (self.game_size.x + self.game_size.width - self.ball_size)
        /* if hits right side */
        {
            self.finished = true;
            self.lost = false;
        } else if self.ball_pos.x <= self.ball_size + self.game_size.x
        /* if hit left side */
        {
            self.finished = true;
            self.lost = true;
        }
        if self.ball_pos.y >= (self.game_size.y + self.game_size.height - self.ball_size)
            || (self.ball_pos.y <= self.ball_size + self.game_size.y)
        {
            self.ball_speed.y *= -1.0;
        }
    }

    fn draw_paddles(self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.game_size.x as i32 + 15,
            self.game_size.y as i32 + self.game_size.height as i32 / 2 + self.paddle1.y as i32
                - self.paddle1.x as i32 / 2,
            self.paddle1.z as i32,
            self.paddle1.x as i32,
            Color::BLACK,
        );
        d.draw_rectangle(
            self.game_size.x as i32 + self.game_size.width as i32 - 15 - 20,
            self.game_size.y as i32 + self.game_size.height as i32 / 2 + self.paddle2.y as i32
                - self.paddle2.x as i32 / 2,
            self.paddle2.z as i32,
            self.paddle2.x as i32,
            Color::BLACK,
        );
    }

    fn draw_frame(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.game_size.x as i32,
            self.game_size.y as i32,
            self.game_size.width as i32,
            self.game_size.height as i32,
            raylib::prelude::Color::DARKGRAY,
        );

        d.draw_circle(
            self.ball_pos.x as i32,
            self.ball_pos.y as i32,
            self.ball_size,
            raylib::prelude::Color::BLACK,
        );

        (*self).draw_paddles(d);
    }
}

impl Game for Pong {
    fn get_info(&mut self) -> &str {
        "Pong" as &str
    }

    fn logic(&mut self, _mouse_pos: Vector2, delta_time: f32) {
        self.pong_logic(delta_time);
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_frame(d);
    }

    fn is_finished(&mut self) -> Option<bool> {
        if self.finished {
            (&mut *self).reset();
            Some(self.lost)
        } else {
            None
        }
    }
}