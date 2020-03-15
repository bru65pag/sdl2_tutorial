extern crate sdl2;

use std::convert::TryInto;
use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard;
// use sdl2::rect::Point;
use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use std::time::Duration;

// struct Player {
//     pos_x:i32,
//     pos_y:i32,
//     size_x:u32,
//     size_y:u32
// }
//
// impl Default for Player {
//     fn default() -> Player {
//         Player {pos_x: 20,pos_y:20,size_x:20,size_y:20}
//     }
// }

fn main() {
    let sleep_time = time::Duration::from_millis(16);
    let mut player = Rect::new(20,20,20,20);
    let pos_x:i32 = 100;
    let pos_y:i32 = 200;
    let size_x:u32 = 300;
    let size_y:u32 = 400;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Server",size_x,size_y)
        .position(pos_x,pos_y)
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let mut running:bool = true;
    while running {
        let mut event_pump = sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                }
                Event::KeyDown { timestamp: _, window_id: _,
                keycode , scancode: _, keymod: _, repeat: _ } => {
                    match keycode.unwrap() {
                        keyboard::Keycode::Right => {
                            if player.x + player.w <  size_x.try_into().unwrap() {
                                player.x += 1;
                            }
                        }
                        keyboard::Keycode::Left => {
                            if player.x > 0 {
                                player.x -= 1;
                            }
                        }
                        keyboard::Keycode::Down => {
                            if player.y + player.h < size_y.try_into().unwrap() {
                                player.y += 1;
                            }
                        }
                        keyboard::Keycode::Up => {
                            if player.y > 0 {
                                player.y -= 1;
                            }
                        }
                        keyboard::Keycode::Escape => {
                            running = false;
                        }
                        _ => {
                        }
                    }
                }
                _ => {
                }
            }
        }
        // render
        canvas.set_draw_color(Color::RGB(0,0,255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255,0,255));
        canvas.fill_rect(player).unwrap();

        canvas.present();

        //thread::sleep(sleep_time);
    }
}
