
pub mod grid;
pub mod cell;

fn main() {
    println!("Hello, world!");

    let mut grid1 = grid::Grid::new((10, 10));

    let seed = vec![
        cell::Cell{state: cell::CellState::Alive, pos: (3,3)},
        cell::Cell{state: cell::CellState::Alive, pos: (5,4)},
        cell::Cell{state: cell::CellState::Alive, pos: (5,5)},
        cell::Cell{state: cell::CellState::Alive, pos: (6,5)}
    ];

    grid1.seed(seed);



    println!("----------------------------");
    grid1.display();
}
