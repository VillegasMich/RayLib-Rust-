mod player;

use player::Player;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Do Not Get Hit!")
        .vsync()
        .build();
    rl.set_target_fps(60);

    let mut frame_count: u32 = 1;
    let mut player = Player::new();

    while !rl.window_should_close() {
        player.update(&mut rl, &mut frame_count);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        player.draw(&mut d);
    }
}
