
use std::fmt;
use std::collections::HashMap;

use crate::cell;
use tui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid grid position")
    }
}

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

// TODO replace this
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


// 2D grid finite grid
#[derive(Clone)]
pub struct Grid {
    size: (usize, usize),
    state: HashMap<(usize, usize), cell::Cell>,
    color: Color,
    dim: usize
}

impl Grid {

    // static method
    pub fn new(size: (usize, usize)) -> Self {

        let mut state = HashMap::<(usize, usize), cell::Cell>::new();

        for i in 0..size.0 {

            for j in 0..size.1 {

                let cell = cell::Cell::new(cell::CellState::Dead, (i,j));
                state.entry( (i,j) ).or_insert(cell);

            }
        }

        Self {
            size,
            state,
            color: Color::Cyan,
            dim: 2
        }

    }

    // instance methods
    pub fn display(&self) {

        // since printing is done line by line, index by y axis
        for j in 0..self.size.1 {

            let mut x_chars = vec!['-'; self.size.0];

            for i in 0..self.size.0 {

                // safely unwrap optional retrieved from hashmap
                if let Some(cell) = self.state.get(&(i,j)) {

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

    pub fn update(&mut self, updated_cells: HashMap<(usize, usize), cell::CellState>) {

        for (position, new_state) in updated_cells.iter() {
            if let Some(cell) = self.state.get_mut(position) {
                cell.state = *new_state;
            }
        }

    }

    pub fn get_size(&self) -> &(usize, usize) {
        &self.size
    }

    pub fn get_cell(&self, position: &(usize, usize)) -> Option<&cell::Cell> {
        self.state.get(position)
    }

    pub fn get_neighbors(&self, position: &(usize, usize)) -> Result<Vec<(usize, usize)>, OutOfBoundsError> {

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        // apply the search policy
        let search = self.search_policy(position);

        for direction in search?.iter() {
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

        Ok(neighbors)

    }

    pub fn get_position_description(&self, position: &(usize, usize)) -> Result<PositionDescription, OutOfBoundsError> {

        let description: PositionDescription;

        if position.0 == 0 && position.1 < (self.size.1-1) && position.1 > 0 {

            description = PositionDescription::LeftBound;

        } else if position.0 == (self.size.0-1) && position.1 < (self.size.1-1) && position.1 > 0 {

            description = PositionDescription::RightBound;

        } else if position.0 < (self.size.0-1) && position.0 > 0 && position.1 == 0 {

            description = PositionDescription::BottomBound;

        } else if position.0 < (self.size.0-1) && position.0 > 0 && position.1 == (self.size.1-1) {

            description = PositionDescription::TopBound;

        } else if position.0 == 0 && position.1 == 0 {

            description = PositionDescription::BottomLeftCorner;

        } else if position.0 == 0 && position.1 == (self.size.1-1) {

            description = PositionDescription::TopLeftCorner;

        } else if position.0 == (self.size.0-1) && position.1 == (self.size.1-1) {

            description = PositionDescription::TopRightCorner;

        } else if position.0 == (self.size.0-1) && position.1 == 0 {

            description = PositionDescription::BottomRightCorner;

        } else if position.0 > 0 && position.0 < (self.size.0-1) && position.1 > 0 && position.1 <
            (self.size.1-1) {

            description = PositionDescription::Center;

        } else {

            return Err(OutOfBoundsError);

        }

        Ok(description)

    }


    pub fn search_policy(&self, position: &(usize, usize)) -> Result<Vec<GridDirection>, OutOfBoundsError> {

        let search: Vec<GridDirection>;
        let mut dimensions = *self.get_size();
        // dimensions copies the result from get_dimensions()
        dimensions.0 -= 1;
        dimensions.1 -= 1;

        // ? evaluates the Result and expands the Ok()'s or automatically returns the Err()
        // unwrap evaluates the Result match and automatically raises a panic! for Err()
        match self.get_position_description(position)? {
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


        Ok(search)

    }

}

// tui-rs integration
impl Shape for Grid {

    fn draw(&self, painter: &mut Painter) {
        for (position, cell) in self.state.iter() {
            match cell.get_state() {
                cell::CellState::Alive => painter.paint(position.0, position.1, self.color),
                cell::CellState::Dead => continue
            }
        }
    }

}




#[cfg(test)]

#[test]
fn test_grid_new() {

    let _ = Grid::new((10,10));

}

#[test]
fn test_grid_update() {

    let mut new_grid = Grid::new((10,10));

    let mut seed: HashMap<(usize,usize), cell::CellState> = HashMap::new();
    seed.entry((3,3)).or_insert(cell::CellState::Alive);

    new_grid.update(seed);

}






