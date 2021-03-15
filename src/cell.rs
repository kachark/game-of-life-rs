

#[derive(Debug, Clone)]
pub enum CellState {
    Alive,
    Dead
}


#[derive(Debug, Clone)]
pub struct Cell {
    pub state: CellState,
    pub pos: (usize, usize)
}

impl Cell {

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

