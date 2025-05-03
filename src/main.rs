#![windows_subsystem = "windows"]
use mki::{bind_key, Action, InhibitEvent, Keyboard};
use raylib::{ffi::SetConfigFlags, prelude::*};
use rand::prelude::*;
use std::{sync::{Arc, Mutex}, time::{Duration, SystemTime}};
use winapi::um::winuser::*;
mod pong;
use pong::Pong;
mod circles;

fn screen_size() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        return (width, height - 1);
    }
}

fn draw_title(d: &mut RaylibDrawHandle, title: &str) {
    d.draw_text("current game:", 9, 9, 30, Color::WHITE);
    d.draw_text("current game:", 11, 11, 30, Color::WHITE);
    d.draw_text("current game:", 10, 10, 30, Color::BLACK);
    d.draw_text(title, 9, 39, 50, Color::WHITE);
    d.draw_text(title, 11, 41, 50, Color::WHITE);
    d.draw_text(title, 10, 40, 50, Color::BLACK);
}

fn main() {
    let exit_window = Arc::new(Mutex::new(false));
    let exit_window_clone = exit_window.clone();
    let size_tuple = screen_size();

    // flags are in order: borderless, mouse passthrough, window topmost, vsync, transparent, undecorated
    let flags: u32 = 32_768 + 16_384 + 4_096 + 64 + 16 + 8;
    unsafe {
        SetConfigFlags(flags);
    }
    let (mut rl, thread) = raylib::init()
        .title("Borderless Fullscreen")
        .size(size_tuple.0, size_tuple.1)
        .build();

    bind_key(
        Keyboard::F8,
        Action {
            callback: Box::new(move |_, _| {
                println!("quitting");
                let mut exit = exit_window_clone.lock().unwrap();
                *exit = true;
            }),
            inhibit: InhibitEvent::Yes,
            sequencer: false,
            defer: true,
        },
    );

    rl.set_exit_key(Some(KeyboardKey::KEY_F8));
    let mut pong = Pong::new();

    let game_size: (i32, i32) = (800, 400);

    pong.set_game_size(Rectangle {
        x: ((size_tuple.0 / 2) - (game_size.0 / 2)) as f32,
        y: ((size_tuple.1 / 2) - (game_size.1 / 2)) as f32,
        width: game_size.0 as f32,
        height: game_size.1 as f32,
    });

    let mut rng = rand::rng();
    let mut now = SystemTime::now();
    let cooldown = Duration::from_secs(rng.random_range(5..15) * 1);

    while !*exit_window.lock().unwrap() {
        let delta_time = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        
        if pong.finished {
            pong.reset();
            now = SystemTime::now();
        }else if now.elapsed().unwrap() > cooldown {
            draw_title(&mut d, "pong");
            (&mut pong).draw_frame(d, delta_time);
        }
        
    }
}
