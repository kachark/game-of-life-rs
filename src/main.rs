#[allow(dead_code)]


use std::{io, time::Duration, collections::HashMap};
use grid::Grid;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{
        canvas::{Canvas, Map, MapResolution, Rectangle},
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


struct App {
    x: f64,
    y: f64,
    area: Rect,
    cell_grid: Grid
}

impl App {

    fn new() -> Self {

        // TODO temporarily hardcoded
        let grid_x: usize = 100;
        let grid_y: usize = 200;
        let mut grid1 = grid::Grid::new((grid_x, grid_y));

        let mut seed: HashMap<(usize,usize), cell::CellState> = HashMap::new();
        seed.entry((3,3)).or_insert(cell::CellState::Alive);
        seed.entry((3,2)).or_insert(cell::CellState::Alive);
        seed.entry((3,1)).or_insert(cell::CellState::Alive);
        seed.entry((2,3)).or_insert(cell::CellState::Alive);
        seed.entry((1,2)).or_insert(cell::CellState::Alive);

        grid1.update(seed);

        Self {
            x: 0.0,
            y: 0.0,
            area: Rect::new(10, 10, 100, 100), // x, y, width, height layout
            cell_grid: grid1
        }

    }

    fn update(&mut self) {

        // update the game of life here
        let mut delta: HashMap<(usize, usize), cell::CellState> = HashMap::new();

        for i in 0..self.cell_grid.get_dimensions().0 {
            for j in 0..self.cell_grid.get_dimensions().1 {

                let neighbors = self.cell_grid.get_neighbors(&(i,j)).unwrap();

                let mut alive = 0;
                for indices in neighbors {

                    if let Some(neighbor_cell) = self.cell_grid.get_cell(&indices) {

                        match neighbor_cell.get_state() {
                            cell::CellState::Alive => {
                                alive += 1;
                            },
                            cell::CellState::Dead => continue
                        }

                    } else {
                        continue
                    }

                }

                // GAME OF LIFE
                // fewer than 2 live neighbors
                if alive < 2 {

                    if let Some(cell) = self.cell_grid.get_cell(&(i,j)) {

                        // why isn't it coming here for 6,6?
                        match cell.get_state() {
                            cell::CellState::Alive => {
                                delta.entry((i,j)).or_insert(cell::CellState::Dead);
                            },
                            cell::CellState::Dead => continue
                        }

                    }

                } else if alive == 2 || alive == 3 { // 2 or 3 live neighbors

                    if let Some(cell) = self.cell_grid.get_cell(&(i,j)) {

                        match cell.get_state() {
                            cell::CellState::Alive => continue,
                            cell::CellState::Dead => {
                                if alive == 3 {
                                    delta.entry((i,j)).or_insert(cell::CellState::Alive);
                                } else {
                                    continue
                                }
                            }
                        }

                    }

                } else if alive > 3 { // greater than 3 live neighbors

                    if let Some(cell) = self.cell_grid.get_cell(&(i,j)) {

                        match cell.get_state() {
                            cell::CellState::Alive => {
                                delta.entry((i,j)).or_insert(cell::CellState::Dead);
                            },
                            cell::CellState::Dead => continue
                        }

                    }

                }

            }

        }

        self.cell_grid.update(delta);
        // self.cell_grid.display();

    }

}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // TODO
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
                    ctx.draw(&app.cell_grid);
                })
                .x_bounds([10.0, 10.0])
                .y_bounds([10.0, 10.0]);
            f.render_widget(canvas, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                // update the app with game of life updates

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
