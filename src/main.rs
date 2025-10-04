use std::error::Error;

use clap::Parser;

use crate::app::{AppConfig, PlayerType};

mod app;
mod model;
mod ui;

/// Let's play classic Pong in the terminal!
/// Controls:
/// w/s:  Left Player Up/Down
/// arrows keys: Right Player Up/Down
/// q: quit
#[derive(Parser)]
#[clap(author, version, about, verbatim_doc_comment)]
struct Args {
    /// Left player type
    #[clap(short, long, value_enum, default_value_t = PlayerType::Human)]
    left: PlayerType,

    /// Right player type
    #[clap(short, long, value_enum, default_value_t = PlayerType::Ai)]
    right: PlayerType,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments.
    let cli = Args::parse();
    let config = AppConfig {
        player_l: cli.left,
        player_r: cli.right,
    };

    // setup terminal
    let mut terminal = ratatui::init();
    let _ = terminal.clear();

    // create app and run it
    let mut app = app::App::new(config, terminal);

    let res = app::App::run_app(&mut app);

    // restore terminal
    ratatui::restore();

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
