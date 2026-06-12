use crate::{ElementType, world::World};
use macroquad::rand;

fn try_move(
    world: &mut World,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    allowed: &[ElementType],
) -> bool {
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if !world.can_move_into(nx, ny, allowed) {
        return false;
    }

    let nx = nx as usize;
    let ny = ny as usize;

    world.swap_cells(x, y, nx, ny);

    return true;
}

pub fn try_fall(world: &mut World, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    try_move(world, x, y, 0, 1, allowed)
}

pub fn try_diagonal_fall(world: &mut World, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        if world.can_move_into(x as isize - 1, y as isize, allowed)
            && try_move(world, x, y, -1, 1, allowed)
        {
            return true;
        }
        if world.can_move_into(x as isize + 1, y as isize, allowed)
            && try_move(world, x, y, 1, 1, allowed)
        {
            return true;
        }
    } else {
        if world.can_move_into(x as isize + 1, y as isize, allowed)
            && try_move(world, x, y, 1, 1, allowed)
        {
            return true;
        }
        if world.can_move_into(x as isize - 1, y as isize, allowed)
            && try_move(world, x, y, -1, 1, allowed)
        {
            return true;
        }
    }

    return false;
}

pub fn try_sideways(world: &mut World, x: usize, y: usize, allowed: &[ElementType]) -> bool {
    if rand::gen_range(0.0, 1.0) < 0.5 {
        if try_move(world, x, y, -1, 0, allowed) || try_move(world, x, y, 1, 0, allowed) {
            return true;
        }
    } else {
        if try_move(world, x, y, 1, 0, allowed) || try_move(world, x, y, -1, 0, allowed) {
            return true;
        }
    }

    return false;
}
