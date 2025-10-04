use rand::Rng;

use ratatui::style::Color;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::{Circle, Line};

pub enum Controls {
    Stop,
    Up,
    Down,
}

pub struct Ball {
    pub circle: Circle,
    pub vx: f64,
    pub vy: f64,
}

pub struct Paddle {
    pub len: f64,
    pub x: f64,
    pub y: f64,
    pub vy: f64,
    pub color: Color,
}

pub struct Field {
    pub x: f64,
    pub y: f64,
}

pub trait Drawable {
    fn tick(&mut self, field: &Field);
    fn draw(&self, ctx: &mut canvas::Context);
}

impl Drawable for Ball {
    fn tick(&mut self, field: &Field) {
        self.circle.x += self.vx;

        self.circle.y += self.vy;
        if self.circle.y <= 0.0 {
            self.vy = -self.vy;
            self.circle.y = -self.circle.y;
        } else if self.circle.y >= field.y {
            self.vy = -self.vy;
            self.circle.y = 2.0 * field.y - self.circle.y;
        }
    }

    fn draw(&self, ctx: &mut canvas::Context) {
        ctx.draw(&self.circle);
    }
}

pub trait Controlable {
    fn tick(&mut self, field: &Field, input: &Controls);
    fn draw(&self, ctx: &mut canvas::Context);
}

impl Controlable for Paddle {
    fn tick(&mut self, field: &Field, input: &Controls) {
        match input {
            Controls::Stop => {
                self.vy = 0.0;
            }
            Controls::Down => {
                self.vy = -2.0;
            }
            Controls::Up => {
                self.vy = 2.0;
            }
        }

        // Move, but not out of bounds.
        self.y = (self.y + self.vy).clamp(self.len / 2.0, field.y - self.len / 2.0);
    }
    fn draw(&self, ctx: &mut canvas::Context) {
        let line = Line {
            x1: self.x,
            x2: self.x,
            y1: self.y - self.len / 2.0,
            y2: self.y + self.len / 2.0,
            color: self.color,
        };
        ctx.draw(&line);
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
                circle: Circle {
                    x: x / 2.0,
                    y: y / 2.0,
                    radius: 0.5,
                    color: Color::Yellow,
                },
                vx: 0.5,
                vy: 0.5,
            },
            l_paddle: Paddle {
                len: x / 25.0,
                x: x / 12.0,
                y: y / 2.0,
                vy: 0.0,
                color: Color::Blue,
            },
            r_paddle: Paddle {
                len: x / 25.0,
                x: x * 11.0 / 12.0,
                y: y / 2.0,
                vy: 0.0,
                color: Color::Red,
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

        if (self.ball.circle.x - paddle.x).abs() <= self.ball.vx.abs()
            && self.ball.circle.y < (paddle.y + paddle.len / 2.0)
            && self.ball.circle.y > (paddle.y - paddle.len / 2.0)
        {
            // Bounce and speed up a bit
            self.ball.vx *= -1.05;
            self.ball.vy *= 1.05;
        }
    }

    fn score_check(&mut self) {
        if self.ball.circle.x < 0.0 {
            self.r_score += 1;
            self.reset_ball();
        } else if self.ball.circle.x > self.field.x {
            self.l_score += 1;
            self.reset_ball();
            self.ball.vx *= -1.0;
        }
    }

    fn reset_ball(&mut self) {
        self.ball.circle.x = self.field.x / 2.0;
        self.ball.circle.y = self.field.y / 2.0;
        self.ball.vx = self.rng.random_range(0.4..0.6);
        self.ball.vy = self.rng.random_range(0.4..0.6);
    }

    pub fn resize(&mut self, x: f64, y: f64) {
        // Get the current positions as a fraction of the current size.
        let x_ratio = x / self.field.x;
        let y_ratio = y / self.field.y;
        self.ball.circle.x *= x_ratio;
        self.ball.circle.y *= y_ratio;
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
    if (ball.circle.y - paddle.y) > paddle.len / 3.0 {
        return Controls::Up;
    } else if (paddle.y - ball.circle.y) > paddle.len / 3.0 {
        return Controls::Down;
    }
    Controls::Stop
}
