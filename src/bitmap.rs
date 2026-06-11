use crate::{
    Element,
    constants::{CELLS_X_AMOUNT, CELLS_Y_AMOUNT},
};

pub struct Bitmap {
    grid: [Option<Element>; CELLS_X_AMOUNT * CELLS_Y_AMOUNT],
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            grid: core::array::from_fn(|_| None),
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * CELLS_X_AMOUNT + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Element> {
        self.grid[self.idx(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, elem: Option<Element>) {
        self.grid[self.idx(x, y)] = elem;
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        self.grid[self.idx(x, y)] = None;
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= CELLS_X_AMOUNT as isize || y >= CELLS_Y_AMOUNT as isize {
            return false;
        }

        return true;
    }

    // TODO: Change to can_move_into
    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if !self.is_in_bounds(x, y) {
            return false;
        }

        self.grid[self.idx(x as usize, y as usize)].is_none()
    }

    pub fn swap_cells(&mut self, x: usize, y: usize, nx: usize, ny: usize) {
        let src = self.idx(x, y);
        let dest = self.idx(nx, ny);
        self.grid.swap(src, dest);
    }
}
