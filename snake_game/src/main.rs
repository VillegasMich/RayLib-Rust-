pub mod head;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snacke Game")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut cabeza = head::Snake::new();

    while !rl.window_should_close() {
        cabeza.update(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        cabeza.draw(&mut d);
    }
}
