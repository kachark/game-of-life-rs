
use std::collections::HashMap;

use crate::grid;
use crate::cell;

pub fn game_of_life() {

    let grid_x: usize = 10;
    let grid_y: usize = 20;
    let mut grid1 = grid::Grid::new((grid_x, grid_y));

    let mut seed: HashMap<(usize,usize), cell::CellState> = HashMap::new();
    seed.entry((3,3)).or_insert(cell::CellState::Alive);
    seed.entry((3,2)).or_insert(cell::CellState::Alive);
    seed.entry((3,1)).or_insert(cell::CellState::Alive);
    seed.entry((2,3)).or_insert(cell::CellState::Alive);
    seed.entry((1,2)).or_insert(cell::CellState::Alive);

    grid1.update(seed);
    grid1.display();

    // Update
    for _ in 0..50 {

        let mut delta: HashMap<(usize, usize), cell::CellState> = HashMap::new();

        for i in 0..grid1.get_dimensions().0 {
            for j in 0..grid1.get_dimensions().1 {

                let neighbors = grid1.get_neighbors(&(i,j)).unwrap();

                let mut alive = 0;
                for indices in neighbors {

                    if let Some(neighbor_cell) = grid1.get_cell(&indices) {

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

                    if let Some(cell) = grid1.get_cell(&(i,j)) {

                        // why isn't it coming here for 6,6?
                        match cell.get_state() {
                            cell::CellState::Alive => {
                                delta.entry((i,j)).or_insert(cell::CellState::Dead);
                            },
                            cell::CellState::Dead => continue
                        }

                    }

                } else if alive == 2 || alive == 3 { // 2 or 3 live neighbors

                    if let Some(cell) = grid1.get_cell(&(i,j)) {

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

                    if let Some(cell) = grid1.get_cell(&(i,j)) {

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

        // update the grid with the deltas
        println!("{:?}", delta.keys());
        grid1.update(delta);
        println!("----------------------------");
        grid1.display();

    }


}

