extern crate sdl2;

use std::{thread, time};
use sdl2::pixels::Color;

fn main() {
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
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();
    canvas.present();

    let sleep_time = time::Duration::from_secs(4);
    thread::sleep(sleep_time);
}
