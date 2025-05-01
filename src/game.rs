use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Pong {
    ball_size: f32,
    ball_pos: Vector2,
    ball_dir: f32,
    ball_speed: Vector2,
    paddle1: Vector2,
    paddle2: Vector2,
    game_size: Rectangle,
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            ball_size: 12.0,
            ball_pos: Vector2 { x: 0.0, y: 0.0 },
            ball_dir: 1.0,
            ball_speed: Vector2 { x: 300.0, y: 300.0 },
            paddle1: Vector2 { x: 0.0, y: 0.0 },
            paddle2: Vector2 { x: 0.0, y: 0.0 },
            game_size: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
        }
    }

    fn pong_logic(&mut self, delta_time: f32) {

        self.pong_ball(delta_time);
    }

    fn pong_ball(&mut self, delta_time: f32) {
        self.ball_pos.x += self.ball_speed.x * delta_time;
        self.ball_pos.y += self.ball_speed.y * delta_time;
        if self.ball_pos.x >= (self.game_size.x + self.game_size.width - self.ball_size) || (self.ball_pos.x <= self.ball_size + self.game_size.x) {self.ball_speed.x *= -1.0 }
        if self.ball_pos.y >= (self.game_size.y + self.game_size.height - self.ball_size) || (self.ball_pos.y <= self.ball_size + self.game_size.y) {self.ball_speed.y *= -1.0 }
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
        d.draw_rectangle(
            self.game_size.x as i32,
            self.game_size.y as i32,
            self.game_size.width as i32,
            self.game_size.height as i32,
            raylib::prelude::Color::WHITE,
        );

        self.pong_logic(delta_time);

        d.draw_circle(
            self.ball_pos.x as i32,
            self.ball_pos.y as i32,
            self.ball_size,
            raylib::prelude::Color::GREEN,
        );
    }
}

fn get_sin_cos_from_tan(tan: f32) -> (f32, f32) {
    let cos = 1.0 / (1.0 + tan * tan).sqrt();
    let sin = tan * cos;
    (cos, sin)
}
