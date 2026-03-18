//! UI rendering: drawing the game field, ball, paddles, and score.
use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::{Canvas, Circle, Context};

use crate::app::AppStyles;
use crate::model::{Ball, Model, Paddle};

pub trait Drawable {
    fn draw(&self, ctx: &mut Context, style: Style);
}

impl Drawable for Ball {
    fn draw(&self, ctx: &mut Context, style: Style) {
        let circle = Circle {
            x: self.x,
            y: self.y,
            radius: 0.5,
            color: style.fg.unwrap_or(Color::White),
        };
        ctx.draw(&circle);
    }
}
impl Drawable for Paddle {
    fn draw(&self, ctx: &mut Context, style: Style) {
        let line = canvas::Line {
            x1: self.x,
            x2: self.x,
            y1: self.y - self.len / 2.0,
            y2: self.y + self.len / 2.0,
            color: style.fg.unwrap_or(Color::White),
        };
        ctx.draw(&line);
    }
}

pub fn draw_ui(frame: &mut Frame, styles: &AppStyles, model: &Model) {
    let title = Line::from(" PONG ").centered().style(styles.border);

    let score = Line::from(format!(" {} : SCORE : {} ", model.l_score, model.r_score))
        .centered()
        .style(styles.score);

    let canvas = Canvas::default()
        .background_color(styles.field.bg.unwrap_or(Color::Reset))
        .block(
            Block::bordered()
                .title(title)
                .title_bottom(score)
                .style(styles.field)
                .border_style(styles.border),
        )
        .x_bounds([0.0, model.field.x])
        .y_bounds([0.0, model.field.y])
        .paint(|ctx| {
            model.ball.draw(ctx, styles.ball);
            model.l_paddle.draw(ctx, styles.paddle_l);
            model.r_paddle.draw(ctx, styles.paddle_r);
        });

    frame.render_widget(canvas, frame.area());
}
