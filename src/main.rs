extern crate sdl2;

mod components;

use std::env;
use std::path::Path;
use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use components::audio;
use components::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("syntax: chip_8_rust [rom_file]");
    }

    // TODO: Build vm to read roms
    // let mut vm = dale8::VM::new();
    // if !vm.load_application(&args[1]) {
    //     println!("failed load rom");
    //     return;
    // }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let window = video_subsystem
        .window("Chip 8 Emu", DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())
        .unwrap();

    // let mut _audio_device = None;
    let has_sound = Path::new("beep.wav").exists();

    let mut timer = 0;

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } | Event::Quit { .. } => break 'mainloop,
                // Add other keycodes
                _ => {}
            }
        }

        if timer == 2000 {
            // TODO: add emulate cycle.
            // vm.emulate_cycle();
            timer = 0;
        } else {
            timer += 1;
        }

        // TODO: add draw flag.
        // if vm.draw_flag {}

        // TODO: add beep flag.
        // if vm.beep_flag {}
    }
}
