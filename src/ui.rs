use crate::model::{Controlable, Drawable, Model};

use ratatui::{
    Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, canvas::Canvas},
};

pub fn draw_ui(frame: &mut Frame, model: &Model) {
    let title = Line::from(" PONG ".bold()).centered();

    let score_text = Line::from(vec![
        " ".into(),
        model.l_score.to_string().yellow(),
        " : SCORE : ".into(),
        model.r_score.to_string().yellow(),
        " ".into(),
    ])
    .centered();

    let canvas = Canvas::default()
        .block(Block::bordered().title(title).title_bottom(score_text))
        .x_bounds([0.0, model.field.x])
        .y_bounds([0.0, model.field.y])
        .paint(|ctx| {
            model.ball.draw(ctx);
            model.l_paddle.draw(ctx);
            model.r_paddle.draw(ctx);
        });

    frame.render_widget(canvas, frame.area());
}
