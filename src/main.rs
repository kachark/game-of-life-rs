
pub mod grid;
pub mod cell;

fn main() {
    println!("Hello, world!");

    let mut grid1 = grid::Grid::new((10, 10));

    let seed = vec![
        cell::Cell::new(cell::CellState::Alive, (3,3)),
        cell::Cell::new(cell::CellState::Alive, (5,4)),
        cell::Cell::new(cell::CellState::Alive, (5,5)),
        cell::Cell::new(cell::CellState::Alive, (6,5))
    ];

    grid1.seed(seed);


    for k in 0..1 {


        //  probably not good design to do it this way
        // have a mutable reference but will need immutable references -> not possible
        // interior mutability problem -> modifying vectors in place
        // Cell and RefCell may be solutions

        // access the grid elements using indices instead of using iterators and getting references
        for i in 0..grid1.grid.len() {
            for j in 0..grid1.grid[i].len() {

                match grid1.grid[i].get(j) {
                    Some(cell) => {
                        println!("{:?}", cell);
                    },
                    None => continue
                }

            }
        }
    }


    println!("----------------------------");
    grid1.display();
}
