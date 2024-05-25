use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

#[derive(Debug)]
pub struct Snake {
    pub position: Vector2,
    pub size: Vector2,
    pub speed: Vector2,
    pub color: Color,
}

impl Snake {
    pub fn new() -> Snake {
        let instance = Snake {
            position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            size: Vector2::new(12.0, 12.0),
            speed: Vector2::new(3.0, 3.0),
            color: Color::GREEN,
        };
        instance
    }
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.position.x <= 0.0 || self.position.x >= SCREEN_WIDTH - self.size.x {
            self.speed.x += 0.0;
        }
        if rl.is_key_down(KEY_W) {
            if self.position.y <= 0.0 {
            } else {
                self.position.y -= self.speed.y;
            }
        }
        if rl.is_key_down(KEY_D) {
            if self.position.x >= SCREEN_WIDTH - self.size.x {
            } else {
                self.position.x += self.speed.x;
            }
        }
        if rl.is_key_down(KEY_S) {
            if self.position.y >= SCREEN_HEIGHT - self.size.y {
            } else {
                self.position.y += self.speed.y;
            }
        }
        if rl.is_key_down(KEY_A) {
            if self.position.x >= SCREEN_WIDTH - self.size.x {
            } else {
                self.position.x -= self.speed.x;
            }
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let mut color = Color::GREEN;
        for _ in 0..1 {
            d.draw_rectangle_v(self.position, self.size, color);
            if color == Color::GREEN {
                color = Color::YELLOW;
            } else {
                color = Color::GREEN;
            }
        }
    }
}
