//! Let's play classic Pong in the terminal!
//! Controls:
//! w/s: Left Player Up/Down
//! i/k: Right Player Up/Down
//! q: quit
use crate::app::{AppConfig, PlayerType};
use clap::Parser;
use ratatui::crossterm::event::PopKeyboardEnhancementFlags;
use std::io::Result;

mod app;
mod model;
mod ui;
#[derive(Parser)]
#[clap(author, version, about, verbatim_doc_comment)]
struct Args {
    /// Left player type
    #[clap(short, long, value_enum, default_value_t = PlayerType::Human)]
    left: PlayerType,

    /// Right player type
    #[clap(short, long, value_enum, default_value_t = PlayerType::Ai)]
    right: PlayerType,

    /// Color theme
    #[clap(short, long)]
    theme: Option<String>,
}

fn main() -> Result<()> {
    // Parse arguments.
    let cli = Args::parse();
    let config = AppConfig {
        player_l: cli.left,
        player_r: cli.right,
        theme: tca_ratatui::TcaTheme::new(cli.theme.as_deref()),
    };

    // setup terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // create app and run it
    let mut app = app::App::new(config, terminal);

    let res = app.run_app();

    // restore terminal
    ratatui::crossterm::execute!(std::io::stdout(), PopKeyboardEnhancementFlags)?;
    ratatui::restore();

    res
}
