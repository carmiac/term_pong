// Application controller for terminal pong.
use clap::ValueEnum;
use std::{
    io,
    time::{Duration, Instant},
};

use crate::model::{self, Controls, Model, ai_input};
use crate::ui::draw_ui;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

#[derive(PartialEq, Debug, Clone, ValueEnum)]
pub enum PlayerType {
    Human,
    Ai,
}

pub struct AppConfig {
    pub player_l: PlayerType,
    pub player_r: PlayerType,
}
pub struct App {
    pub config: AppConfig,
    pub tick_rate: u64, // Frames per second
    terminal: DefaultTerminal,
    exit: bool,
    model: model::Model,
}

impl App {
    pub fn new(config: AppConfig, terminal: DefaultTerminal) -> App {
        let size = terminal.size().unwrap();
        App {
            config,
            terminal,
            tick_rate: 60,
            exit: false,
            model: Model::new(size.width.into(), size.height.into()),
        }
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        let tick_rate = Duration::from_millis(self.tick_rate);
        let mut last_tick = Instant::now();
        while !self.exit {
            self.terminal.draw(|frame| draw_ui(frame, &self.model))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }
            if last_tick.elapsed() >= tick_rate {
                if self.config.player_r == PlayerType::Ai {
                    self.model.r_input = if self.model.ball.vx > 0.0 {
                        ai_input(&self.model.ball, &self.model.r_paddle)
                    } else {
                        Controls::Stop
                    };
                }
                if self.config.player_l == PlayerType::Ai {
                    self.model.l_input = if self.model.ball.vx < 0.0 {
                        ai_input(&self.model.ball, &self.model.l_paddle)
                    } else {
                        Controls::Stop
                    };
                }
                self.model.tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Resize(x, y) => {
                self.model.resize(x.into(), y.into());
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('w') => self.model.l_input = Controls::Up,
            KeyCode::Char('s') => self.model.l_input = Controls::Down,
            KeyCode::Up => self.model.r_input = Controls::Up,
            KeyCode::Down => self.model.r_input = Controls::Down,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
