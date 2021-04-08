#[allow(dead_code)]

use std::{io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{
        canvas::Canvas,
        Block, Borders,
    },
    Terminal,
};

// declare game of life modules
pub mod grid;
pub mod cell;
pub mod game;
pub mod events;

use crate::events::{Config, Event, Events};
use game::GameOfLife;


struct App {
    area: Rect,
    game: GameOfLife
}

impl App {

    fn new() -> Self {

        Self {
            area: Rect::new(0, 0, 200, 200), // x, y, width, height layout
            game: GameOfLife::default()
        }

    }

    fn update(&mut self) {

        match self.game.update() {
            Ok(_) => (),
            Err(_) => println!("Out of bounds error!")
        }

    }

}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);

    // App
    let mut app = App::new();

    loop {

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Game of Life"))
                .paint(|ctx| {
                    ctx.draw(&app.game.cell_grid);
                });
            f.render_widget(canvas, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }

                // all other key presses
                _ => {}
            },

            Event::Tick => {
                app.update();
            }
        }

    }

    Ok(())

}
