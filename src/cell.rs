

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellState {
    Alive,
    Dead
}


#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub state: CellState,
    pub pos: (usize, usize)
}

impl Cell {

    pub fn new(state: CellState, pos: (usize, usize)) -> Self {
        Self {
            state,
            pos
        }
    }

    pub fn get_state(&self) -> &CellState {
        &self.state
    }

    pub fn get_position(&self) -> &(usize, usize) {
        &self.pos
    }

    pub fn update(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

}


// tests
#[cfg(test)]

#[test]
fn test_cell_new() {

    let new_cell = Cell::new(CellState::Alive, (5,5));

    let cell = Cell {
        state: CellState::Alive,
        pos: (5,5)
    };

    assert_eq!(new_cell, cell);

}

#[test]
fn test_cell_get_state() {

    let cell = Cell {
        state: CellState::Alive,
        pos: (5,5)
    };

    assert_eq!(*cell.get_state(), CellState::Alive);

}

#[test]
fn test_cell_get_position() {

    let cell = Cell {
        state: CellState::Alive,
        pos: (5,5)
    };

    assert_eq!(*cell.get_state(), CellState::Alive);

}

#[test]
fn test_cell_update() {

    let mut cell = Cell {
        state: CellState::Alive,
        pos: (5,5)
    };

    cell.update((10,10));

    assert_eq!(*cell.get_position(), (10,10));

}

