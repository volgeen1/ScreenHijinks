#![windows_subsystem = "windows"]
use game::Pong;
use mki::Keyboard;
use raylib::{ffi::SetConfigFlags, prelude::*};
use std::sync::{Arc, Mutex};
use winapi::um::winuser::*;
mod game;

fn screen_size() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        return (width, height - 1);
    }
}

fn main() {
    let exit_window = Arc::new(Mutex::new(false));
    let exit_window_clone = exit_window.clone();
    let size_tuple = screen_size();
    let flags: u32 = 32_768 + 16_384 + 4_096 + 64 + 16 + 8;
    unsafe {
        SetConfigFlags(flags);
    }
    let (mut rl, thread) = raylib::init()
        .title("Borderless Fullscreen")
        .size(size_tuple.0, size_tuple.1)
        .build();

    (&Keyboard::F8).bind(move |_| {
        println!("quitting");
        let mut exit = exit_window_clone.lock().unwrap();
        *exit = true;
    });

    rl.set_exit_key(Some(KeyboardKey::KEY_F8));
    let mut pong = Pong::new();

    pong.set_game_size(Rectangle {
        x: ((size_tuple.0 / 2) - (640 / 2)) as f32,
        y: ((size_tuple.1 / 2) - (480 / 2)) as f32,
        width: 640 as f32,
        height: 480 as f32,
    });

    while !*exit_window.lock().unwrap() {

        let delta_time = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        d.draw_text(&format!("{:?}", delta_time), 10, 10, 100, Color::BLACK);

        pong.draw_frame(d, delta_time);
    }
}
