use crate::{
    Element, ElementType,
    constants::{CELLS_X_AMOUNT, CELLS_Y_AMOUNT},
    movement,
};

// TODO: Rename to world
pub struct Bitmap {
    cells: [Element; CELLS_X_AMOUNT * CELLS_Y_AMOUNT],
}

impl Bitmap {
    pub fn new() -> Self {
        Self {
            cells: core::array::from_fn(|_| Element::new(ElementType::Air)),
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * CELLS_X_AMOUNT + x
    }

    pub fn get(&self, x: usize, y: usize) -> Element {
        self.cells[self.idx(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, elem: Element) {
        self.cells[self.idx(x, y)] = elem;
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        self.cells[self.idx(x, y)] = Element::new(ElementType::Air);
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

        self.cells[self.idx(x as usize, y as usize)].t == ElementType::Air
    }

    pub fn can_move_into(&self, x: isize, y: isize, allowed: &[ElementType]) -> bool {
        if !self.is_in_bounds(x, y) {
            return false;
        }

        let source_elem_type = self.cells[self.idx(x as usize, y as usize)].t;
        allowed.contains(&source_elem_type)
    }

    pub fn swap_cells(&mut self, x: usize, y: usize, nx: usize, ny: usize) {
        let src = self.idx(x, y);
        let dest = self.idx(nx, ny);

        self.cells.swap(src, dest);

        // Hacky, calling this to run update on the cell that was swapped in place of src
        // coz otherwise update will never fire on that cell in that frame.
        // And in situations like pouring sand into water that makes water go as high as the
        // point of where sand starts pouring
        // FIXME: Probably would go into infinite loop when two cells could swap each other
        // self.update_cell(x, y);
    }

    fn update_cell(&mut self, x: usize, y: usize) {
        let element = self.get(x, y);

        match element.t {
            ElementType::Air | ElementType::Stone => {
                return;
            }
            ElementType::Sand => {
                if movement::try_fall(self, x, y, &[ElementType::Air, ElementType::Water]) {
                    return;
                }

                movement::try_diagonal_fall(self, x, y, &[ElementType::Air, ElementType::Water]);
            }
            ElementType::Water => {
                if movement::try_fall(self, x, y, &[ElementType::Air]) {
                    return;
                }

                if movement::try_diagonal_fall(self, x, y, &[ElementType::Air]) {
                    return;
                }

                movement::try_sideways(self, x, y, &[ElementType::Air]);
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
