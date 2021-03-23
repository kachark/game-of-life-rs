
use std::fmt;
use std::collections::HashMap;

use cell::Cell;

use crate::cell;


#[derive(Debug)]
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

#[derive(Debug)]
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
    pub cells: HashMap<(usize, usize), cell::Cell>
}

impl Grid {

    // static method
    pub fn new(size: (usize, usize)) -> Self {

        // TODO return Result for better error handling

        assert_eq!(size.0, size.1, "Grid dimensions do not match.");

        let mut cells = HashMap::<(usize, usize), cell::Cell>::new();

        for i in 0..size.0 {

            for j in 0..size.1 {

                let cell = cell::Cell::new(cell::CellState::Dead, (i,j));
                cells.entry( (i,j) ).or_insert(cell);

            }
        }

        Self {
            size,
            cells
        }

    }

    // instance methods
    pub fn display(&self) {

        // since printing is done line by line, index by y axis
        for j in 0..self.size.1 {

            let mut x_chars = vec!['-'; self.size.0];

            for i in 0..self.size.0 {

                // safely unwrap optional retrieved from hashmap
                if let Some(cell) = self.cells.get(&(i,j)) {

                    match cell.get_state() {
                        cell::CellState::Alive => x_chars[i] = 'x',
                        cell::CellState::Dead => continue
                    }

                }

            }

            let row_string: String = x_chars.into_iter().collect();
            println!("{:?}", row_string);

        }

    }

    pub fn seed(&mut self, seeds: Vec<cell::Cell>) {

        for seed_cell in seeds {
            let position = seed_cell.get_position();

            if let Some(cell) = self.cells.get_mut(position) {
                cell.state = seed_cell.state;
            }

        }

    }

    pub fn update(&mut self, updated_cells: HashMap<(usize, usize), cell::CellState>) {

        for (position, state) in updated_cells.iter() {
            if let Some(cell) = self.cells.get_mut(position) {
                cell.state = *state;
            }
        }

    }

    pub fn get_dimensions(&self) -> &(usize, usize) {
        &self.size
    }

    pub fn get_neighbors(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        // apply the search policy
        let search = self.search_policy(position);

        for direction in search.iter() {
            match direction {
                GridDirection::Up{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::Down{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::Left{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::Right{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::UpLeft{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::UpRight{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::DownLeft{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                },
                GridDirection::DownRight{x, y} => {
                    neighbors.push((*x, *y)); // since search will go out of scope, dereference
                }
            }
        }

        neighbors

    }

    pub fn get_position_description(&self, position: &(usize, usize)) -> PositionDescription {

        let description: PositionDescription;

        if position.0 == 0 && position.1 < (self.size.1-1) && position.1 > 0 {

            description = PositionDescription::LeftBound;

        } else if position.0 == (self.size.0-1) && position.1 < (self.size.1-1) && position.1 > 0 {

            description = PositionDescription::RightBound;

        } else if position.1 == 0 && position.0 < (self.size.0-1) && position.0 > 0 {

            description = PositionDescription::BottomBound;

        } else if position.1 == (self.size.1-1) && position.0 < (self.size.0-1) && position.0 > 0 {

            description = PositionDescription::TopBound;

        } else if position.0 == 0 && position.1 == 0 {

            description = PositionDescription::BottomLeftCorner;

        } else if position.0 == 0 && position.1 == (self.size.1-1) {

            description = PositionDescription::TopLeftCorner;

        } else if position.0 == (self.size.0-1) && position.1 == (self.size.1-1) {

            description = PositionDescription::TopRightCorner;

        } else if position.1 == 0 && position.0 == (self.size.1-1) {

            description = PositionDescription::BottomRightCorner;

        } else {

            description = PositionDescription::Center;

        }

        description

    }


    pub fn search_policy(&self, position: &(usize, usize)) -> Vec<GridDirection> {

        let search: Vec<GridDirection>;
        let mut dimensions = *self.get_dimensions();
        // dimensions copies the result from get_dimensions()
        dimensions.0 -= 1;
        dimensions.1 -= 1;

        match self.get_position_description(position) {
            PositionDescription::Center => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Left{ x: position.0 - 1, y: position.1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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
                    GridDirection::Right{ x: 0, y: position.1 },
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
                    GridDirection::UpLeft{ x: position.0 - 1, y: 0 },
                    GridDirection::UpRight{ x: 0, y: 0 },
                    GridDirection::Right{ x: 0, y: position.1 },
                    GridDirection::DownRight{ x: 0, y: position.1 - 1 },
                ];

            },
            PositionDescription::TopLeftCorner => {
                search = vec![
                    GridDirection::Down{ x: position.0, y: position.1 - 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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
                    GridDirection::Right{ x: 0, y: position.1 },
                    GridDirection::UpRight{ x: 0, y: position.1 + 1 },
                    GridDirection::DownLeft{ x: position.0 - 1, y: dimensions.1 },
                    GridDirection::DownRight{ x: 0, y: dimensions.1 }

                ];

            },
            PositionDescription::BottomLeftCorner => {
                search = vec![
                    GridDirection::Up{ x: position.0, y: position.1 + 1 },
                    GridDirection::Right{ x: position.0 + 1, y: position.1 },
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









