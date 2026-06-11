use crate::{
    Element, ElementType,
    constants::{CELLS_X_AMOUNT, CELLS_Y_AMOUNT},
    movement,
};

pub struct Bitmap {
    cells: [Option<Element>; CELLS_X_AMOUNT * CELLS_Y_AMOUNT],
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            cells: core::array::from_fn(|_| None),
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * CELLS_X_AMOUNT + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Element> {
        self.cells[self.idx(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, elem: Option<Element>) {
        self.cells[self.idx(x, y)] = elem;
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        self.cells[self.idx(x, y)] = None;
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

        self.cells[self.idx(x as usize, y as usize)].is_none()
    }

    pub fn swap_cells(&mut self, x: usize, y: usize, nx: usize, ny: usize) {
        let src = self.idx(x, y);
        let dest = self.idx(nx, ny);
        self.cells.swap(src, dest);
    }

    fn update_cell(&mut self, x: usize, y: usize) {
        let mb_element = self.get(x, y);

        let Some(element) = mb_element else {
            return;
        };

        match element.t {
            ElementType::Stone => {
                return;
            }
            ElementType::Sand => {
                if movement::try_fall(self, x, y) {
                    return;
                }

                movement::try_diagonal_fall(self, x, y);
            }
            ElementType::Water => {
                if movement::try_fall(self, x, y) {
                    return;
                }

                if movement::try_diagonal_fall(self, x, y) {
                    return;
                }

                movement::try_sideways(self, x, y);
            }
        }
    }

    pub fn update(&mut self, frame: u32) {
        for y in (0..CELLS_Y_AMOUNT).rev() {
            // Alternating the scan order to prevent one direction bias (can be seen especially in fluids)
            if frame % 2 == 0 {
                for x in 0..CELLS_X_AMOUNT {
                    self.update_cell(x, y);
                }
            } else {
                for x in (0..CELLS_X_AMOUNT).rev() {
                    self.update_cell(x, y);
                }
            }
        }
    }
}
