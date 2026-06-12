use crate::{ElementType, bitmap::Bitmap};
use macroquad::rand;

fn try_move(
    bitmap: &mut Bitmap,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    allowed: &[ElementType],
) -> bool {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if !bitmap.can_move_into(nx, ny, allowed) {
        return false;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    bitmap.swap_cells(x, y, nx, ny);

    return true;
}

pub fn try_fall(bitmap: &mut Bitmap, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    try_move(bitmap, x, y, 0, 1, allowed)
}

pub fn try_diagonal_fall(bitmap: &mut Bitmap, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        if bitmap.can_move_into(x as isize - 1, y as isize, allowed)
            && try_move(bitmap, x, y, -1, 1, allowed)
        {
            return true;
        }
        if bitmap.can_move_into(x as isize + 1, y as isize, allowed)
            && try_move(bitmap, x, y, 1, 1, allowed)
        {
            return true;
        }
    } else {
        if bitmap.can_move_into(x as isize + 1, y as isize, allowed)
            && try_move(bitmap, x, y, 1, 1, allowed)
        {
            return true;
        }
        if bitmap.can_move_into(x as isize - 1, y as isize, allowed)
            && try_move(bitmap, x, y, -1, 1, allowed)
        {
            return true;
        }
    }

    return false;
}

pub fn try_sideways(bitmap: &mut Bitmap, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        if try_move(bitmap, x, y, -1, 0, allowed) || try_move(bitmap, x, y, 1, 0, allowed) {
            return true;
        }
    } else {
        if try_move(bitmap, x, y, 1, 0, allowed) || try_move(bitmap, x, y, -1, 0, allowed) {
            return true;
        }
    }

    return false;
}
