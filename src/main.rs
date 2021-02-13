extern crate sdl2;
mod game;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::thread;
use std::time::{
    Duration, Instant
};

fn main() {
    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();
 
    let window = sdl_video.window("Game Of Life", 1280, 720)
        .position_centered()
        .build()
        .unwrap();
    let window_size = window.drawable_size();
    let (window_width, window_height) = window_size;

    sdl.mouse().show_cursor(false);
    let mut event_pump = sdl.event_pump().unwrap();
 
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(Some(PixelFormatEnum::RGB24), window_width, window_height).unwrap();
    let texture_info = texture.query();
    let bytes_per_pixel = texture_info.format.byte_size_per_pixel();
    assert_eq!(bytes_per_pixel, 3);

    let window_width_u = window_width as usize;
    let window_height_u = window_height as usize;
    let mut pixels = vec![0; bytes_per_pixel * window_width_u * window_height_u];
    let mut buffer = game::ImageBuffer {
        width: window_width_u,
        height: window_height_u,
        pitch: window_width_u * bytes_per_pixel,
        pixels: &mut pixels
    };
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut game_state = game::init(window_size, CELL_SIZE);

    let mut frame: u32 = 0;
    let target_frame_duration = Duration::from_secs_f32(1.0 / 120.0);
    let mut time_last_frame = Instant::now();
    'game_loop: loop {
        frame += 1;
        let events = event_pump.poll_iter();

        let should_exit = game::update(frame, &mut buffer, &mut game_state, events);
        if should_exit {
            break 'game_loop;
        }

        texture.update(None, buffer.pixels, buffer.pitch).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        let time_end_work = Instant::now();

        let target_time = time_last_frame + target_frame_duration;
        sleep_until(target_time);
        let time_end_sleep = Instant::now();

        canvas.present();

        let time_end_frame = time_end_sleep;
        if frame % 60 == 0 {
            let duration_work = time_end_work - time_last_frame;
            let duration_sleep = time_end_sleep - time_end_work;
            let duration_frame = time_end_frame - time_last_frame;
            println!("wk={:.1} sl={:.1} fr={:.3} fps={:.1}",
                     duration_work.as_secs_f32() * 1000.0,
                     duration_sleep.as_secs_f32() * 1000.0,
                     duration_frame.as_secs_f32() * 1000.0,
                     1.0 / duration_frame.as_secs_f32());
        }
        time_last_frame = time_end_frame;
    }
}

fn sleep_until(target_time: Instant) {
    let mut now = Instant::now();
    let threshold_duration = Duration::from_micros(1000);
    while now + threshold_duration < target_time {
        let sleep_time = target_time - now - threshold_duration;
        thread::sleep(sleep_time);
        now = Instant::now();
    }
    while now < target_time {
        now = Instant::now();
    }
}

const CELL_SIZE: u32 = 2;
