use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Raylib Tutorial")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(3.0, 3.0),
        radius: 25.0,
        color: Color::GREEN,
    };
    let mut ball2 = Ball {
        position: Vector2::new(SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(3.0, 3.0),
        radius: 25.0,
        color: Color::BLUE,
    };
    let mut ball3 = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(0.0, 4.0),
        radius: 25.0,
        color: Color::PURPLE,
    };

    let mut pause = false;
    let mut _frame_count = 0;
    let mut mouse_button = false;

    while !rl.window_should_close() {
        /* --- UPDATE --- */
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }
        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            mouse_button = true;
            ball3.position = rl.get_mouse_position();
        }
        if !pause {
            ball.position += ball.speed;
            ball2.position += ball2.speed;
            ball3.position += ball3.speed;
            let collision =
                check_collision_circles(ball.position, ball.radius, ball2.position, ball2.radius);

            // Check collitions
            if collision {
                ball.speed.x *= ball2.speed.x.signum();
                ball.speed.y *= ball2.speed.y.signum();
                ball2.speed.x *= ball.speed.x.signum();
                ball2.speed.y *= ball.speed.y.signum();
                // pause = !pause;
            }
            if ball.position.x >= SCREEN_WIDTH - ball.radius || ball.position.x <= ball.radius {
                ball.speed.x *= -1.0;
            }
            if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
                ball.speed.y *= -1.0;
            }
            if ball2.position.x >= SCREEN_WIDTH - ball2.radius || ball2.position.x <= ball2.radius {
                ball2.speed.x *= -1.0;
            }
            if ball2.position.y >= SCREEN_HEIGHT - ball2.radius || ball2.position.y <= ball2.radius
            {
                ball2.speed.y *= -1.0;
            }
            if ball3.position.x >= SCREEN_WIDTH - ball3.radius || ball3.position.x <= ball3.radius {
                ball3.speed.x = 0.0;
            }
            if ball3.position.y >= SCREEN_HEIGHT - ball3.radius || ball3.position.y <= ball3.radius
            {
                ball3.speed.y = 0.0;
            }
        } else {
            _frame_count += 1;
        }
        /* --- DRAW --- */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle_v(ball.position, ball.radius, ball.color);
        d.draw_circle_v(ball2.position, ball2.radius, ball2.color);
        if pause {
            d.draw_text("Pause", 12, 12, 20, Color::WHITE);
        }
        d.draw_text(
            "Press SPACE to puse ball movement",
            10,
            (SCREEN_HEIGHT as i32) - 25,
            20,
            Color::WHITE,
        );
        d.draw_fps(SCREEN_WIDTH as i32 - 80, 10);
        if mouse_button {
            d.draw_circle_v(ball3.position, ball3.radius, ball3.color);
        }
    }
}
