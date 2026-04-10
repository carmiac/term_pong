//! Game model: ball, paddles, field, and simulation logic.
use rand::RngExt;

const BALL_SPEED: f64 = 0.15;
const SPEED_INCREMENT: f64 = 1.1;
const PADDLE_SPEED: f64 = 0.6;

pub enum Controls {
    Stop,
    Up,
    Down,
}

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

pub struct Paddle {
    pub len: f64,
    pub x: f64,
    pub y: f64,
    pub vy: f64,
}

pub struct Field {
    pub x: f64,
    pub y: f64,
}

pub trait Tickable {
    fn tick(&mut self, field: &Field);
}

impl Tickable for Ball {
    fn tick(&mut self, field: &Field) {
        self.x += self.vx;
        self.y += self.vy;
        if self.y <= 0.0 {
            self.vy = -self.vy;
            self.y = -self.y;
        } else if self.y >= field.y {
            self.vy = -self.vy;
            self.y = 2.0 * field.y - self.y;
        }
    }
}

pub trait Controllable {
    fn tick(&mut self, field: &Field, input: &Controls);
}

impl Controllable for Paddle {
    fn tick(&mut self, field: &Field, input: &Controls) {
        match input {
            Controls::Stop => {
                self.vy = 0.0;
            }
            Controls::Down => {
                self.vy = -PADDLE_SPEED;
            }
            Controls::Up => {
                self.vy = PADDLE_SPEED;
            }
        }

        // Move, but not out of bounds.
        self.y = (self.y + self.vy).clamp(self.len / 2.0, field.y - self.len / 2.0);
    }
}
pub struct Model {
    pub ball: Ball,
    pub l_paddle: Paddle,
    pub r_paddle: Paddle,
    pub l_score: u64,
    pub r_score: u64,
    pub l_input: Controls,
    pub r_input: Controls,
    pub field: Field,
    rng: rand::rngs::ThreadRng,
}

impl Model {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            ball: Ball {
                x: x / 2.0,
                y: y / 2.0,
                vx: BALL_SPEED,
                vy: BALL_SPEED,
            },
            l_paddle: Paddle {
                len: x / 25.0,
                x: x / 12.0,
                y: y / 2.0,
                vy: 0.0,
            },
            r_paddle: Paddle {
                len: x / 25.0,
                x: x * 11.0 / 12.0,
                y: y / 2.0,
                vy: 0.0,
            },
            l_score: 0,
            r_score: 0,
            field: Field { x, y },
            l_input: Controls::Stop,
            r_input: Controls::Stop,
            rng: rand::rng(),
        }
    }

    pub fn tick(&mut self) {
        // Update the model for a single tick.
        self.l_paddle.tick(&self.field, &self.l_input);
        self.l_input = Controls::Stop;
        self.r_paddle.tick(&self.field, &self.r_input);
        self.r_input = Controls::Stop;

        self.paddle_collision_check();
        self.ball.tick(&self.field);
        self.score_check();
    }

    fn paddle_collision_check(&mut self) {
        let paddle = if self.ball.vx < 0.0 {
            &self.l_paddle
        } else {
            &self.r_paddle
        };

        if (self.ball.x - paddle.x).abs() <= self.ball.vx.abs()
            && self.ball.y < (paddle.y + paddle.len / 2.0)
            && self.ball.y > (paddle.y - paddle.len / 2.0)
        {
            // Bounce and speed up a bit
            self.ball.vx *= -SPEED_INCREMENT;
            self.ball.vy *= SPEED_INCREMENT;
        }
    }

    fn score_check(&mut self) {
        if self.ball.x < 0.0 {
            self.r_score += 1;
            self.reset_ball();
        } else if self.ball.x > self.field.x {
            self.l_score += 1;
            self.reset_ball();
            self.ball.vx *= -1.0;
        }
    }

    fn reset_ball(&mut self) {
        self.ball.x = self.field.x / 2.0;
        self.ball.y = self.field.y / 2.0;
        self.ball.vx = self.rng.random_range(0.1..0.17);
        self.ball.vy = self.rng.random_range(0.1..0.17);
    }

    pub fn resize(&mut self, x: f64, y: f64) {
        // Get the current positions as a fraction of the current size.
        let x_ratio = x / self.field.x;
        let y_ratio = y / self.field.y;
        self.ball.x *= x_ratio;
        self.ball.y *= y_ratio;
        self.ball.vx *= x_ratio;
        self.ball.vy *= y_ratio;
        self.l_paddle.x *= x_ratio;
        self.l_paddle.y *= y_ratio;
        self.l_paddle.len *= y_ratio;
        self.r_paddle.x *= x_ratio;
        self.r_paddle.y *= y_ratio;
        self.r_paddle.len *= y_ratio;
        self.field.x = x;
        self.field.y = y;
    }
}

pub fn ai_input(ball: &Ball, paddle: &Paddle) -> Controls {
    if (ball.y - paddle.y) > paddle.len / 3.0 {
        return Controls::Up;
    } else if (paddle.y - ball.y) > paddle.len / 3.0 {
        return Controls::Down;
    }
    Controls::Stop
}
