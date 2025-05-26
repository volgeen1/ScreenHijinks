use std::{time::Duration, ffi::CString};
use raylib::{prelude::*, ffi::MeasureText};


use crate::{loss_handler::Effect, util::Timer};

pub struct Flash {
    effect_size: Rectangle,
    duration: Timer,
}

impl Flash {
    pub fn new(screen_size: (i32, i32), duration: Duration) -> Flash {
        Flash {
            effect_size: rrect(0, 0, screen_size.0, screen_size.1),
            duration: Timer::new(duration),
        }
    }

    pub fn draw_frame(&mut self, d: &mut RaylibDrawHandle, delta_time: f32) {
        self.duration.update(delta_time);
        d.draw_rectangle(0 as i32, 0 as i32, self.effect_size.width as i32, self.effect_size.height as i32, Color::DARKRED);

        unsafe {
            let time_left = self.duration.time_left();
            let text = CString::new(format!("{:?}", time_left as i32)).unwrap();
            d.draw_text(
                &format!("{}", text.to_str().unwrap()),
                (self.effect_size.x * 2.0 + self.effect_size.width) as i32 / 2
                    - (MeasureText(text.as_ptr() as *const i8, 50) as i32 / 2)
                    + 2,
                self.effect_size.y as i32 + 10,
                50,
                Color::BLACK,
            );
        }
    }
}

impl Effect for Flash {
    fn draw(&mut self, d: &mut RaylibDrawHandle, delta_time: f32) {
        self.draw_frame(d, delta_time);
    }

    fn is_finished(&mut self) -> bool {
        if self.duration.is_finished() {
            self.duration.reset();
            true
        } else {
            false
        }
    }
}