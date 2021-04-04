
use std::collections::HashMap;

use crate::grid;
use crate::cell;

pub struct GameOfLife {
    evolution: u32,
    pub cell_grid: grid::Grid
}

impl GameOfLife {

    pub fn default() -> Self {

        let grid_x: usize = 100;
        let grid_y: usize = 200;
        let mut default_grid = grid::Grid::new((grid_x, grid_y));

        let mut seed: HashMap<(usize,usize), cell::CellState> = HashMap::new();
        seed.entry((3,3)).or_insert(cell::CellState::Alive);
        seed.entry((3,2)).or_insert(cell::CellState::Alive);
        seed.entry((3,1)).or_insert(cell::CellState::Alive);
        seed.entry((2,3)).or_insert(cell::CellState::Alive);
        seed.entry((1,2)).or_insert(cell::CellState::Alive);

        default_grid.update(seed);

        Self {
            evolution: 0,
            cell_grid: default_grid,
        }

    }

    pub fn update(&mut self) -> Result<(), grid::OutOfBoundsError> {

        self.evolution += 1;

        let mut delta: HashMap<(usize, usize), cell::CellState> = HashMap::new();

        // Traverse the grid
        for i in 0..self.cell_grid.get_size().0 {
            for j in 0..self.cell_grid.get_size().1 {

                // Compute neighbors at (i, j)
                let neighbors = self.cell_grid.get_neighbors(&(i,j));
                let neighbors = match neighbors {

                    Ok(positions) => positions,
                    Err(error) => return Err(error)

                };

                // Count number of living neighbors
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

                // Apply Game of Life rules and track changes to be made to the grid
                self.play(alive, &(i,j), &mut delta);

            }
        }

        // Update grid with changes
        self.cell_grid.update(delta);
        Ok(())

    }

    fn play(&self, living_neighbors: i32, current_position: &(usize, usize),  delta: &mut
        HashMap<(usize, usize), cell::CellState>) {

        // GAME OF LIFE
        if living_neighbors < 2 {

            if let Some(cell) = self.cell_grid.get_cell(current_position) {

                match cell.get_state() {
                    cell::CellState::Alive => {
                        delta.entry(*current_position).or_insert(cell::CellState::Dead);
                    },
                    cell::CellState::Dead => ()
                }

            }

        } else if living_neighbors == 2 || living_neighbors == 3 {

            if let Some(cell) = self.cell_grid.get_cell(current_position) {

                match cell.get_state() {
                    cell::CellState::Alive => (),
                    cell::CellState::Dead => {
                        if living_neighbors == 3 {
                            delta.entry(*current_position).or_insert(cell::CellState::Alive);
                        }
                    }
                }

            }

        } else if living_neighbors > 3 {

            if let Some(cell) = self.cell_grid.get_cell(current_position) {

                match cell.get_state() {
                    cell::CellState::Alive => {
                        delta.entry(*current_position).or_insert(cell::CellState::Dead);
                    },
                    cell::CellState::Dead => ()
                }

            }

        }

    }

    pub fn display(&self) {

        println!("----------------------------");
        self.cell_grid.display();

    }

}

