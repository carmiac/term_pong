//! Application controller: event loop, input handling, and configuration.
use clap::ValueEnum;
use ratatui::crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use ratatui::layout::Size;
use ratatui::style::Style;
use std::collections::HashSet;
use std::{
    io,
    time::{Duration, Instant},
};
use tca_ratatui::TcaTheme;

use crate::model::{self, ai_input, Controls, Model};

use crate::ui::draw_ui;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal,
};

#[derive(PartialEq, Debug, Clone, ValueEnum)]
pub enum PlayerType {
    Human,
    Ai,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub player_l: PlayerType,
    pub player_r: PlayerType,
    pub theme: TcaTheme,
}

#[derive(Debug, Clone)]
pub(crate) struct AppStyles {
    pub border: Style,
    pub score: Style,
    pub field: Style,
    pub ball: Style,
    pub paddle_l: Style,
    pub paddle_r: Style,
}

impl From<&TcaTheme> for AppStyles {
    fn from(value: &TcaTheme) -> Self {
        AppStyles {
            border: Style::default()
                .fg(value.ui.border_primary)
                .bg(value.ui.bg_primary)
                .bold(),
            score: Style::default()
                .fg(value.semantic.info)
                .bg(value.ui.bg_primary)
                .bold(),
            field: Style::default()
                .fg(value.ui.fg_primary)
                .bg(value.ui.bg_primary),
            ball: Style::default().fg(value.semantic.info),
            paddle_l: Style::default().fg(value.ansi.red),
            paddle_r: Style::default().fg(value.ansi.blue),
        }
    }
}

pub struct App {
    config: AppConfig,
    styles: AppStyles,
    terminal: DefaultTerminal,
    exit: bool,
    model: model::Model,
    held_keys: HashSet<KeyCode>,
}

const TICK_RATE: u64 = 60;

impl App {
    pub fn new(config: AppConfig, terminal: DefaultTerminal) -> App {
        let size = terminal.size().unwrap_or(Size {
            width: 80,
            height: 24,
        });
        let styles = AppStyles::from(&config.theme);
        App {
            config,
            terminal,
            styles,
            exit: false,
            model: Model::new(size.width.into(), size.height.into()),
            held_keys: HashSet::new(),
        }
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        ratatui::crossterm::execute!(
            std::io::stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )?;
        let tick_rate = Duration::from_millis(1000 / TICK_RATE);
        let mut last_tick = Instant::now();
        while !self.exit {
            self.terminal
                .draw(|frame| draw_ui(frame, &self.styles, &self.model))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }
            if last_tick.elapsed() >= tick_rate {
                self.model.r_input = match self.config.player_r {
                    PlayerType::Ai => {
                        if self.model.ball.vx > 0.0 {
                            ai_input(&self.model.ball, &self.model.r_paddle)
                        } else {
                            Controls::Stop
                        }
                    }
                    PlayerType::Human => {
                        match (
                            self.held_keys.contains(&KeyCode::Char('i')),
                            self.held_keys.contains(&KeyCode::Char('k')),
                        ) {
                            (true, _) => Controls::Up,
                            (_, true) => Controls::Down,
                            _ => Controls::Stop,
                        }
                    }
                };
                self.model.l_input = match self.config.player_l {
                    PlayerType::Ai => {
                        if self.model.ball.vx < 0.0 {
                            ai_input(&self.model.ball, &self.model.l_paddle)
                        } else {
                            Controls::Stop
                        }
                    }
                    PlayerType::Human => {
                        match (
                            self.held_keys.contains(&KeyCode::Char('w')),
                            self.held_keys.contains(&KeyCode::Char('s')),
                        ) {
                            (true, _) => Controls::Up,
                            (_, true) => Controls::Down,
                            _ => Controls::Stop,
                        }
                    }
                };
                self.model.tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Resize(x, y) => self.model.resize(x.into(), y.into()),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.kind {
            KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('q') {
                    self.exit();
                }
                self.held_keys.insert(key_event.code);
            }
            KeyEventKind::Release => {
                self.held_keys.remove(&key_event.code);
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
