use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const PLAYER_HORIZONTAL_SPEED: f32 = 8.0;
const PLAYER_VERTICAL_SPEED: f32 = 5.0;
const BORDER_EXTRA_LIMIT: f32 = 10.0;
const GRAVITY: f32 = 7.2;

#[derive(Debug)]
pub struct Player {
    pub position: Vector2,
    pub speed: Vector2,
    // while the character is only a rectabgle
    pub size: Vector2,
    pub color: Color,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2::new(crate::SCREEN_WIDTH / 2.0, crate::SCREEN_HEIGHT / 2.0),
            speed: Vector2::new(PLAYER_HORIZONTAL_SPEED, PLAYER_VERTICAL_SPEED),
            size: Vector2::new(32.0, 64.0),
            color: Color::RED,
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.position, self.size, self.color);
        d.draw_text(
            ("X: ".to_string() + self.position.x.to_string().as_str()).as_str(),
            12,
            12,
            32,
            Color::WHITE,
        );
        d.draw_text(
            ("Y: ".to_string() + self.position.y.to_string().as_str()).as_str(),
            12,
            50,
            32,
            Color::WHITE,
        );
    }
    pub fn update(&mut self, rl: &mut RaylibHandle, jump_count: &mut u32) {
        if self.position.y < crate::SCREEN_HEIGHT - self.size.y - BORDER_EXTRA_LIMIT {
            // self.speed.y += GRAVITY * 2.0;
            self.position.y += GRAVITY;
        }
        if rl.is_key_down(KEY_D)
            && self.position.x <= crate::SCREEN_WIDTH - self.size.x - BORDER_EXTRA_LIMIT
        {
            self.position.x += self.speed.x;
        }
        if rl.is_key_down(KEY_A) && self.position.x >= BORDER_EXTRA_LIMIT {
            self.position.x -= self.speed.x;
        }
        if rl.is_key_pressed(KEY_W) && self.position.y >= BORDER_EXTRA_LIMIT {
            while *jump_count % 30 != 0 {
                self.position.y -= self.speed.y;
                *jump_count += 1;
            }
            *jump_count += 1;
        }
        if rl.is_key_down(KEY_S)
            && self.position.y <= crate::SCREEN_HEIGHT - self.size.y - BORDER_EXTRA_LIMIT
        {
            self.position.y += self.speed.y;
        }
        // *frame_count += 1;
        println!("{}", jump_count);
    }
}
