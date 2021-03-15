
use std::fmt;

use crate::cell;


pub enum GridDirection {
    Up { x: usize, y: usize },
    Down { x: usize, y: usize },
    Left { x: usize, y: usize },
    Right { x: usize, y: usize },
    UpLeft { x: usize, y: usize },
    UpRight { x: usize, y: usize },
    DownLeft { x: usize, y: usize },
    DownRight { x: usize, y: usize }
}

pub enum PositionDescription {
    Center,
    TopBound,
    BottomBound,
    RightBound,
    LeftBound,
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner
}


#[derive(Clone)]
pub struct Grid {
    pub size: (usize, usize),
    pub grid: Vec< Vec< Option<cell::Cell> > >
}

impl Grid {

    // static method
    pub fn new(size: (usize, usize)) -> Self {

        // TODO return Result for better error handling

        assert_eq!(size.0, size.1, "Grid dimensions do not match.");

        // row of optional Cells of length size[0]. Initialized to None.
        let row: Vec<Option<cell::Cell>> = vec![None; size.0];
        // populate grid with size[1] rows
        let grid = vec![row; size.1];

        Self {
            size,
            grid
        }

    }

    // instance methods
    pub fn display(&self) {

        for row in self.grid.iter() {

            let mut row_char = vec!['-'; self.size.0];

            for (j, col) in row.iter().enumerate() {

                // If option resolves to a Cell, display an "x"
                match col {
                    Some(_) => row_char[j] = 'x',
                    None => continue
                }

            }

            let row_string: String = row_char.into_iter().collect();
            println!("{:?}", row_string);

        }

    }


    pub fn seed(&mut self, seeds: Vec<cell::Cell>) {

        for cell in seeds {
            let position = *cell.get_position();
            self.grid[position.0][position.1] = Some(cell);
        }

    }

    pub fn get_dimensions(&self) -> &(usize, usize) {
        &self.size
    }

    pub fn get_neighbors(&self, cell: cell::Cell) -> Vec<&cell::Cell> {

        // For a given Cell, check all neighboring directions

        let position = cell.get_position();
        let mut neighbors: Vec<&cell::Cell> = Vec::new();

        // apply the search policy
        let search = self.search_policy(position);

        for direction in search.iter() {
            match direction {
                GridDirection::Up{x, y} => {
                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::Down{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::Left{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::Right{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::UpLeft{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::UpRight{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::DownLeft{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                },
                GridDirection::DownRight{x, y} => {

                    let search_result = &self.grid[*x][*y]; // to index need to dereference to the owned type, not a ref
                    if let Some(cell) = search_result {
                        neighbors.push(cell);
                    }
                }
            }
        }

        neighbors

    }

    pub fn get_position_description(&self, position: &(usize, usize)) -> PositionDescription {

        let description: PositionDescription;

        if position.0 == 0 && position.1 < self.size.1 && position.1 > 0 {

            description = PositionDescription::LeftBound;

        } else if position.0 == self.size.0 && position.1 < self.size.1 && position.1 > 0 {

            description = PositionDescription::RightBound;

        } else if position.1 == 0 && position.0 < self.size.0 && position.0 > 0 {

            description = PositionDescription::BottomBound;

        } else if position.1 == self.size.1 && position.0 < self.size.0 && position.0 > 0 {

            description = PositionDescription::TopBound;

        } else if position.0 == 0 && position.1 == 0 {

            description = PositionDescription::BottomLeftCorner;

        } else if position.0 == 0 && position.1 == self.size.1 {

            description = PositionDescription::TopLeftCorner;

        } else if position.0 == self.size.0 && position.1 == self.size.1 {

            description = PositionDescription::TopRightCorner;

        } else if position.0 == 0 && position.1 == self.size.1 {

            description = PositionDescription::BottomRightCorner;

        } else {

            description = PositionDescription::Center;

        }

        description

    }


    pub fn search_policy(&self, position: &(usize, usize)) -> Vec<GridDirection> {

        let search: Vec<GridDirection>;
        let dimensions = self.get_dimensions();

        match self.get_position_description(position) {
            PositionDescription::Center => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::UpLeft{ x: position.0 - 1, y: position.1 + 1 },
                    GridDirection::UpRight{ x: position.0 + 1, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: position.1 - 1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: position.1 -1 }
                ];
            },
            PositionDescription::TopBound => {
                search = vec![
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: position.1 - 1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: position.1 - 1 },

                    // wrap around
                    GridDirection::Up{ x: position.0, y: 0 },
                    GridDirection::UpLeft{ x: position.0 - 1, y: 0 },
                    GridDirection::UpRight{ x: position.0 + 1, y: 0 }
                ];

            },
            PositionDescription::BottomBound => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::UpLeft{ x: position.0 - 1, y: position.1 + 1 },
                    GridDirection::UpRight{ x: position.0 + 1, y: position.1 + 1 },

                    // wrap around
                    GridDirection::Down{ x: position.0, y: dimensions.1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: dimensions.1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: dimensions.1 }

                ];

            },
            PositionDescription::LeftBound => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::UpRight{ x: position.0 + 1, y: position.1 + 1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: position.1 -1 },

                    // wrap around
                    GridDirection::Left{ x: dimensions.0, y: position.1 },
                    GridDirection::UpLeft{ x: dimensions.0, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: dimensions.0, y: position.1 - 1 }

                ];

            },
            PositionDescription::RightBound => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::UpLeft{ x: position.0 - 1, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: position.1 - 1 },

                    // wrap around
                    GridDirection::Right{ x: 0, y: position.0 },
                    GridDirection::UpRight{ x: 0, y: position.1 + 1 },
                    GridDirection::DownRight{ x: 0, y: position.1 -1 },
                ];

            },
            PositionDescription::TopRightCorner => {
                search = vec![
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: position.1 - 1 },

                    // wrap around
                    GridDirection::Up{ x: position.0, y: 0 },
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::UpRight{ x: position.0 + 1, y: position.1 + 1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: position.1 -1 },
                ];

            },
            PositionDescription::TopLeftCorner => {
                search = vec![
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::DownRight{ x: position.0 + 1, y: position.1 -1 },

                    // wrap around
                    GridDirection::Up{ x: position.0, y: 0 },
                    GridDirection::Left{ x: dimensions.0, y: position.1 },
                    GridDirection::UpLeft{ x: dimensions.0 , y: 0 },
                    GridDirection::UpRight{ x: position.0 + 1, y: 0 },
                    GridDirection::DownLeft{ x: dimensions.0, y: position.1 - 1 },
                ];

            },
            PositionDescription::BottomRightCorner => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::UpLeft{ x: position.0 - 1, y: position.1 + 1 },

                    // wrap around
                    GridDirection::Down{ x: position.0, y: dimensions.1 },
                    GridDirection::Right{ x: 0, y: position.0 },
                    GridDirection::UpRight{ x: 0, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: dimensions.1 },
                    GridDirection::DownRight{ x: 0, y: dimensions.1 }

                ];

            },
            PositionDescription::BottomLeftCorner => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.0 },
                    GridDirection::UpRight{ x: position.0 + 1, y: position.1 + 1 },

                    // wrap around
                    GridDirection::Down{ x: position.0, y: dimensions.1 },
                    GridDirection::Left{ x: dimensions.0, y: position.1 },
                    GridDirection::UpLeft{ x: dimensions.0, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: dimensions.0, y: dimensions.1 },
                    GridDirection::DownRight{ x: position.0 + 1, y: dimensions.1 }
                ];

            }
        }


        search

    }

}

// TODO implement something to pretty print the grid with the cells
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Dimensions: ({}, {})", self.size.0, self.size.1)

    }
}









