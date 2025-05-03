use mki::Keyboard;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Pong {
    ball_size: f32,
    ball_pos: Vector2,
    ball_speed: Vector2,
    paddle1: Vector3,
    paddle2: Vector3,
    game_size: Rectangle,
    pub finished: bool,
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            ball_size: 12.0,
            ball_pos: Vector2 { x: 0.0, y: 0.0 },
            ball_speed: Vector2 { x: 300.0, y: 300.0 },
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
            game_size: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            finished: false,
        }
    }

    pub fn reset(&mut self) {
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
        self.finished = false;
    }

    fn pong_logic(&mut self, delta_time: f32) {
        let paddle_speed = 150.0;

        self.pong_ball(delta_time);

        if Keyboard::Up.is_pressed()
            && self.paddle1.y as i32 - self.paddle1.x as i32 / 2
                > self.game_size.height as i32 / 2 * -1
        {
            self.paddle1.y -= paddle_speed * delta_time;
        } else if Keyboard::Down.is_pressed()
            && self.paddle1.y as i32 + self.paddle1.x as i32 / 2 < self.game_size.height as i32 / 2
        {
            self.paddle1.y += paddle_speed * delta_time;
        }

        if self.ball_pos.y < self.paddle2.y + self.game_size.y + (self.game_size.height / 2.0) {
            self.paddle2.y -= paddle_speed * delta_time;
        }else if self.ball_pos.y > self.paddle2.y + self.game_size.y + (self.game_size.height / 2.0) {
            self.paddle2.y += paddle_speed * delta_time;
        }
    }

    fn pong_ball(&mut self, delta_time: f32) {
        self.ball_pos.x += self.ball_speed.x * delta_time;
        self.ball_pos.y += self.ball_speed.y * delta_time;
        let paddle1rec = Rectangle::new(
            self.game_size.x + 15.0,
            self.game_size.y + self.game_size.height / 2.0 + self.paddle1.y
                - self.paddle1.x / 2.0,
            self.paddle1.z,
            self.paddle1.x ,
        );
        let paddle2rec = Rectangle::new(
            self.game_size.x + self.game_size.width - 15.0 - 20.0,
            self.game_size.y + self.game_size.height / 2.0 + self.paddle2.y
                - self.paddle2.x / 2.0,
            self.paddle2.z,
            self.paddle2.x ,
        );

        let ball_speed = 300.0;
        if paddle1rec.check_collision_circle_rec(self.ball_pos, self.ball_size) {
            self.ball_speed.x = ball_speed;
        }else if paddle2rec.check_collision_circle_rec(self.ball_pos, self.ball_size) {
            self.ball_speed.x = ball_speed * -1.0;
        }else if self.ball_pos.x >= (self.game_size.x + self.game_size.width - self.ball_size)
            || (self.ball_pos.x <= self.ball_size + self.game_size.x)
        {
            self.finished = true;
        }
        if self.ball_pos.y >= (self.game_size.y + self.game_size.height - self.ball_size)
            || (self.ball_pos.y <= self.ball_size + self.game_size.y)
        {
            self.ball_speed.y *= -1.0;
        }
    }

    fn draw_paddles(self, mut d: RaylibDrawHandle) {
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

    pub fn set_game_size(&mut self, game_size: Rectangle) {
        self.game_size.x = game_size.x;
        self.game_size.y = game_size.y;
        self.game_size.width = game_size.width;
        self.game_size.height = game_size.height;
        self.ball_pos.x = game_size.x + (game_size.width / 2.0);
        self.ball_pos.y = game_size.y + (game_size.height / 2.0);
    }

    pub fn draw_frame(&mut self, mut d: RaylibDrawHandle, delta_time: f32) {
        (&mut *self).pong_logic(delta_time);

        d.draw_rectangle(
            self.game_size.x as i32,
            self.game_size.y as i32,
            self.game_size.width as i32,
            self.game_size.height as i32,
            raylib::prelude::Color::WHITE,
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
