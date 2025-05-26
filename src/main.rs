//#![windows_subsystem = "windows"]
use mki::{Action, InhibitEvent, Keyboard, bind_key};
use raylib::{ffi::SetConfigFlags, prelude::*};
use std::sync::{Arc, Mutex};
use winapi::um::winuser::*;
mod game_handler;
mod loss_handler;
mod games;
mod effects;
mod util;
use game_handler::GameHandler;
use loss_handler::LossHandler;

fn screen_size() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        return (width, height - 1);
    }
}

fn get_mouse_pos() -> Vector2 {
    unsafe {
        let mut point = winapi::shared::windef::POINT { x: 0, y: 0 };
        winapi::um::winuser::GetCursorPos(&mut point);
        return Vector2::new(point.x as f32, point.y as f32);
    }
}

fn main() {
    let exit_window = Arc::new(Mutex::new(false));
    let exit_window_clone = exit_window.clone();
    let size_tuple = screen_size();

    // All ConfigFlags:
    // Basic window flags (1-16)
    // FLAG_FULLSCREEN_MODE = 2              - Set to run program in fullscreen
    // FLAG_WINDOW_RESIZABLE = 4             - Set to allow resizable window
    // FLAG_WINDOW_UNDECORATED = 8           - Set to disable window decoration (frame and buttons)
    // FLAG_WINDOW_TRANSPARENT = 16          - Set to allow transparent framebuffer

    // Display quality flags (32-64)
    // FLAG_MSAA_4X_HINT = 32               - Set to try enabling MSAA 4X
    // FLAG_VSYNC_HINT = 64                  - Vsync enables vertical synchronization

    // Window state flags (128-2048)
    // FLAG_WINDOW_HIDDEN = 128              - Set to hide window
    // FLAG_WINDOW_ALWAYS_RUN = 256          - Set to allow windows running while minimized
    // FLAG_WINDOW_MINIMIZED = 512           - Set to minimize window
    // FLAG_WINDOW_MAXIMIZED = 1_024         - Set to maximize window
    // FLAG_WINDOW_UNFOCUSED = 2_048         - Set to window non focused

    // Advanced window flags (4096-65536)
    // FLAG_WINDOW_TOPMOST = 4_096           - Set to window always on top
    // FLAG_WINDOW_HIGHDPI = 8_192          - Set to support HighDPI
    // FLAG_WINDOW_MOUSE_PASSTHROUGH = 16_384 - Set to support mouse passthrough
    // FLAG_BORDERLESS_WINDOWED_MODE = 32_768 - Set to borderless windowed mode
    // FLAG_INTERLACED_HINT = 65_536        - Set to try enabling interlaced video format

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

    let mut game_handler = GameHandler::new(size_tuple).unwrap();
    let mut loss_handler = LossHandler::new(size_tuple);

    game_handler.select_game();
    loss_handler.select_effect();
    println!("entering loop");
    while !*exit_window.lock().unwrap() {
        let mouse_pos = get_mouse_pos();
        let delta_time = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });

        if !loss_handler.finished() {
            loss_handler.do_effect(&mut d, delta_time);
            continue;
        }
        if let Some(result) =game_handler.finished() {
            if result {
                loss_handler.do_effect(&mut d, delta_time);
            }
        } else if game_handler.ready() {
            game_handler.do_frame(delta_time, mouse_pos, &mut d);
        }
    }
}
