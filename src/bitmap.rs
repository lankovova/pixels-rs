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

    pub fn get(&self, x: usize, y: usize) -> Option<Element> {
        self.grid[y * CELLS_X_AMOUNT + x]
    }

    pub fn set(&mut self, x: usize, y: usize, elem: Option<Element>) {
        self.grid[y * CELLS_X_AMOUNT + x] = elem;
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        self.grid[y * CELLS_X_AMOUNT + x] = None;
    }

    pub fn has(&self, x: usize, y: usize) -> bool {
        self.grid[y * CELLS_X_AMOUNT + x].is_some()
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < CELLS_X_AMOUNT && y < CELLS_Y_AMOUNT
    }
}
