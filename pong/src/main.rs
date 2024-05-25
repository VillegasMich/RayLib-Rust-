use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SIZE_PLAYER_X: f32 = 10.0;
const SIZE_PLAYER_Y: f32 = 64.0;
const BALL_ACC: f32 = 1.3;
const BALL_SPEED: f32 = 4.0;
const PLAYER_SPEED: f32 = 8.0;

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
}

struct Player {
    position: Vector2,
    speed_y: f32,
    size: Vector2,
    score: u32,
    color: Color,
}

fn reset_ball(ball: &mut Ball) {
    ball.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    let new_speed = BALL_SPEED * ball.speed.x.signum();
    ball.speed = Vector2::new(new_speed * -1.0, 0.0);
}
fn clear_ball(ball: &mut Ball) {
    ball.position = Vector2::zero() - 10.0;
    ball.speed = Vector2::zero();
}
fn reset_game(ball: &mut Ball, player_1: &mut Player, player_2: &mut Player, winner: &mut i32) {
    ball.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    ball.speed = Vector2::new(BALL_SPEED, 0.0);
    player_1.position = Vector2::new(SIZE_PLAYER_X, SCREEN_HEIGHT / 2.0);
    player_1.score = 0;
    player_2.position = Vector2::new(SCREEN_WIDTH - (SIZE_PLAYER_X * 2.0), SCREEN_HEIGHT / 2.0);
    player_2.score = 0;
    *winner = 0
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Raylib Pong")
        .vsync()
        .build();
    rl.set_target_fps(60);

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(BALL_SPEED, 0.0),
        radius: 7.0,
        color: Color::WHITE,
    };

    let mut player_1 = Player {
        position: Vector2::new(SIZE_PLAYER_X, SCREEN_HEIGHT / 2.0),
        speed_y: PLAYER_SPEED,
        size: Vector2::new(SIZE_PLAYER_X, SIZE_PLAYER_Y),
        score: 0,
        color: Color::BLUE,
    };
    let mut player_2 = Player {
        position: Vector2::new(SCREEN_WIDTH - (SIZE_PLAYER_X * 2.0), SCREEN_HEIGHT / 2.0),
        speed_y: PLAYER_SPEED,
        size: Vector2::new(SIZE_PLAYER_X, SIZE_PLAYER_Y),
        score: 0,
        color: Color::RED,
    };

    let mut pause = false;
    let mut winner = 0;

    while !rl.window_should_close() {
        /* --- UPDATE --- */
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }

        if rl.is_key_pressed(KEY_R) {
            reset_game(&mut ball, &mut player_1, &mut player_2, &mut winner);
        }
        if player_1.score == 5 {
            winner = 1
        }
        if player_2.score == 5 {
            winner = 2
        }
        if !pause && winner == 0 {
            // Player movement
            ball.position += ball.speed;
            if rl.is_key_down(KEY_W) && player_1.position.y >= 0.0 {
                player_1.position.y -= player_1.speed_y
            }
            if rl.is_key_down(KEY_S) && player_1.position.y <= SCREEN_HEIGHT - player_1.size.y {
                player_1.position.y += player_1.speed_y
            }
            if rl.is_key_down(KEY_UP) && player_2.position.y >= 0.0 {
                player_2.position.y -= player_2.speed_y
            }
            if rl.is_key_down(KEY_DOWN) && player_2.position.y <= SCREEN_HEIGHT - player_2.size.y {
                player_2.position.y += player_2.speed_y
            }
            // Ball collisions
            if ball.position.y <= 0.0 || ball.position.y >= SCREEN_HEIGHT - ball.radius {
                ball.speed.y *= -1.0;
            }
            if ball.position.x - ball.radius <= player_1.position.x + ball.radius
                && (ball.position.y >= player_1.position.y
                    && ball.position.y <= player_1.position.y + player_1.size.y)
            {
                ball.speed.x *= -1.0;
                ball.speed.x *= BALL_ACC;
                let offset = (player_1.position.y - ball.position.y) / player_1.size.y;
                ball.speed.y += -offset;
            }
            if ball.position.x + ball.radius >= player_2.position.x
                && (ball.position.y >= player_2.position.y
                    && ball.position.y <= player_2.position.y + player_2.size.y)
            {
                ball.speed.x *= -1.0;
                ball.speed.x *= BALL_ACC;
                let offset = (player_2.position.y - ball.position.y) / player_2.size.y;
                ball.speed.y += -offset;
            }
            // Player scores
            if ball.position.x <= 0.0 {
                player_2.score += 1;
                reset_ball(&mut ball);
            }
            if ball.position.x >= SCREEN_WIDTH {
                player_1.score += 1;
                reset_ball(&mut ball);
            }

            // println!("{:?}", ball.position);
        }

        /* --- DRAW --- */

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        if pause {
            d.draw_text("Pause", 12, 12, 20, Color::WHITE);
        }
        if winner == 1 {
            clear_ball(&mut ball);
            d.draw_text(
                "Wins Player 1!",
                150,
                (SCREEN_HEIGHT / 2.0) as i32,
                48,
                Color::WHITE,
            );
            d.draw_text(
                "press esc to exit",
                150,
                (SCREEN_HEIGHT / 2.0) as i32 + 45,
                16,
                Color::WHITE,
            );
        }
        if winner == 2 {
            clear_ball(&mut ball);
            d.draw_text(
                "Wins Player 2!",
                150,
                (SCREEN_HEIGHT / 2.0) as i32,
                48,
                Color::WHITE,
            );
            d.draw_text(
                "press esc to exit",
                150,
                (SCREEN_HEIGHT / 2.0) as i32 + 45,
                16,
                Color::WHITE,
            );
        }
        d.draw_text(
            player_1.score.to_string().as_str(),
            (SCREEN_WIDTH / 2.0 - 150.0) as i32,
            (100.0) as i32,
            30,
            Color::WHITE,
        );
        d.draw_text(
            player_2.score.to_string().as_str(),
            (SCREEN_WIDTH / 2.0 + 150.0) as i32,
            (100.0) as i32,
            30,
            Color::WHITE,
        );
        d.draw_circle_v(ball.position, ball.radius, ball.color);
        d.draw_rectangle_v(player_1.position, player_1.size, player_1.color);
        d.draw_rectangle_v(player_2.position, player_2.size, player_2.color);
    }
}
