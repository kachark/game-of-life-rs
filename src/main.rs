
use std::collections::HashMap;

pub mod grid;
pub mod cell;

fn main() {
    println!("Hello, world!");

    let grid_x: usize = 10;
    let grid_y: usize = 20;
    let mut grid1 = grid::Grid::new((grid_x, grid_y));

    let seed = vec![
        cell::Cell::new(cell::CellState::Alive, (3,3)),
        cell::Cell::new(cell::CellState::Alive, (3,2)),
        cell::Cell::new(cell::CellState::Alive, (3,1)),
        cell::Cell::new(cell::CellState::Alive, (2,3)),
        cell::Cell::new(cell::CellState::Alive, (1,2))
    ];

    grid1.seed(seed);
    grid1.display();

    for _ in 0..50 {

        let mut delta = HashMap::<(usize, usize), cell::CellState>::new();

        for i in 0..grid1.size.0 {
            for j in 0..grid1.size.1 {

                let neighbors = grid1.get_neighbors(&(i,j));

                let mut alive = 0;
                for indices in neighbors {

                    if let Some(neighbor_cell) = grid1.cells.get(&indices) {

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

                    if let Some(cell) = grid1.cells.get(&(i,j)) {

                        // why isn't it coming here for 6,6?
                        match cell.get_state() {
                            cell::CellState::Alive => {
                                delta.entry((i,j)).or_insert(cell::CellState::Dead);
                            },
                            cell::CellState::Dead => continue
                        }

                    }

                } else if alive == 2 || alive == 3 { // 2 or 3 live neighbors

                    if let Some(cell) = grid1.cells.get(&(i,j)) {

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

                    if let Some(cell) = grid1.cells.get(&(i,j)) {

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
