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

    pub fn empty(&self, x: usize, y: usize) -> bool {
        self.grid[self.idx(x, y)].is_none()
    }

    pub fn swap_cells(&mut self, x: usize, y: usize, nx: usize, ny: usize) {
        let src = self.idx(x, y);
        let dest = self.idx(nx, ny);
        self.grid.swap(src, dest);
    }
}
