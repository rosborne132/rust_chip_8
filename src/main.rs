extern crate sdl2;

mod components;

use sdl2::audio::{AudioCVT, AudioSpecDesired, AudioSpecWAV};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::{env, error, path};

use components::audio::Sound;
use components::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use components::cpu::Cpu;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("syntax: chip_8_rust [rom_file]");
    }

    let mut cpu = Cpu::new();
    if !cpu.load_application(&args[1]) {
        println!("Failed to load rom");
        return Ok(());
    }

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
    let mut _audio_device = None;
    let has_sound = path::Path::new("beep.wav").exists();
    let mut timer = 0;

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                // ------------------------------
                // ---------- key down ----------
                // ------------------------------
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    cpu.key[1] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    cpu.key[2] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    cpu.key[3] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    cpu.key[0xC] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    cpu.key[4] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    cpu.key[5] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    cpu.key[6] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    cpu.key[0xD] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cpu.key[7] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    cpu.key[8] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    cpu.key[9] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    cpu.key[0xE] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    cpu.key[0xA] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    cpu.key[0] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    cpu.key[0xB] = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => {
                    cpu.key[0xF] = 1;
                }
                // ------------------------------
                // ----------- key up -----------
                // ------------------------------
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    cpu.key[1] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    cpu.key[2] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    cpu.key[3] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    cpu.key[0xC] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    cpu.key[4] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    cpu.key[5] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    cpu.key[6] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    cpu.key[0xD] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cpu.key[7] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    cpu.key[8] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    cpu.key[9] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    cpu.key[0xE] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    cpu.key[0xA] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    cpu.key[0] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    cpu.key[0xB] = 0;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => {
                    cpu.key[0xF] = 0;
                }
                _ => {}
            }
        }

        if timer == 2000 {
            cpu.opcode();
            timer = 0;
        } else {
            timer += 1;
        }

        if cpu.draw_flag {
            // Update texture
            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for y in 0..SCREEN_HEIGHT as usize {
                        for x in 0..SCREEN_WIDTH as usize {
                            let offset: usize = y * pitch + x * 3;
                            let mut color: u8 = 0;
                            if cpu.gfx[((y * SCREEN_WIDTH as usize) + x) as usize] != 0 {
                                color = 255;
                            }
                            buffer[offset] = color;
                            buffer[offset + 1] = color;
                            buffer[offset + 2] = color;
                        }
                    }
                })
                .unwrap();

            canvas.clear();

            // Copy over new texture to canvas
            canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT)),
                )
                .unwrap();

            // display new changes to canvas
            canvas.present();

            cpu.draw_flag = false;
        }

        if cpu.beep_flag {
            if has_sound {
                let desired_spec = AudioSpecDesired {
                    freq: Some(44_100),
                    channels: Some(1), // mono
                    samples: None,     // default
                };

                _audio_device = Some(Box::new(
                    audio_subsystem
                        .open_playback(None, &desired_spec, |spec| {
                            let wav = AudioSpecWAV::load_wav("beep.wav")
                                .expect("could not load test WAV file");
                            let cvt = AudioCVT::new(
                                wav.format,
                                wav.channels,
                                wav.freq,
                                spec.format,
                                spec.channels,
                                spec.freq,
                            )
                            .expect("could not convert WAV file");
                            let data = cvt.convert(wav.buffer().to_vec());

                            Sound {
                                data,
                                volume: 0.25,
                                pos: 0,
                            }
                        })
                        .unwrap(),
                ));

                // start playback
                if let Some(ref dev) = _audio_device {
                    dev.resume();
                }
            } else {
                println!("BEEP");
            }

            cpu.beep_flag = false;
        }
    }

    println!("Tearing down emu.");
    return Ok(());
}
