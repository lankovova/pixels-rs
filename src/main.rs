mod bitmap;
mod constants;
mod movement;
mod utils;

use crate::{
    bitmap::Bitmap,
    constants::{CELLS_X_AMOUNT, CELLS_Y_AMOUNT},
    utils::{CellCoords, get_hovered_cell},
};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
enum ElementType {
    Stone,
    Sand,
    Water,
}

#[derive(Clone, Copy)]
struct Element {
    t: ElementType,
    color: Color,
}

impl Element {
    fn new(t: ElementType) -> Self {
        let color = match t {
            ElementType::Stone => GRAY,
            ElementType::Sand => GOLD,
            ElementType::Water => BLUE,
        };

        let amount = rand::gen_range(-0.2, 0.2);
        let color = utils::lighten_hsl(color, amount);

        Self { t, color }
    }
}

fn render(
    bitmap: &Bitmap,
    cell_width: f32,
    cell_height: f32,
    hovered_cell: Option<CellCoords>,
    active_element_type: &ElementType,
    is_eraser_on: bool,
) {
    clear_background(BLACK);

    for y in 0..CELLS_Y_AMOUNT {
        for x in 0..CELLS_X_AMOUNT {
            let mb_element = &bitmap.get(x, y);

            let Some(element) = mb_element else {
                continue;
            };

            draw_rectangle(
                (x as f32) * cell_width,
                (y as f32) * cell_height,
                cell_width,
                cell_height,
                element.color,
            );
        }
    }

    if let Some(hovered) = &hovered_cell {
        draw_rectangle_lines(
            (hovered.x as f32) * cell_width,
            (hovered.y as f32) * cell_height,
            cell_width,
            cell_height,
            2.0,
            BLUE,
        );
    }

    // UI
    let hovered_text = if let Some(cell) = hovered_cell {
        format!("Hovered: x: {}, y: {}", cell.x, cell.y)
    } else {
        "Hovered: None".to_string()
    };
    draw_text(hovered_text, 10.0, 20.0, 16.0, WHITE);

    let active_text = if is_eraser_on {
        "Selected: Eraser".to_string()
    } else {
        format!("Selected: {:?}", active_element_type)
    };
    draw_text(active_text, 10.0, 40.0, 16.0, WHITE);

    draw_multiline_text(
        "Elements:\ns[t]one\n[s]and\n[w]ater",
        screen_width() - 120.0,
        20.0,
        16.0,
        Some(1.0),
        WHITE,
    );
}

#[macroquad::main("Pixust")]
async fn main() {
    let mut bitmap = Bitmap::new();
    let mut active_element_type = ElementType::Stone;
    let mut is_eraser_on = false;
    let mut frame: u32 = 0;

    // Skip first frame because screen dimensions are wrong on the first pass
    // probably because of auto resize that happens when user uses WM
    next_frame().await;

    loop {
        let cell_width = screen_width() / (CELLS_X_AMOUNT as f32);
        let cell_height = screen_height() / (CELLS_Y_AMOUNT as f32);

        let hovered_cell = get_hovered_cell(cell_width, cell_height);

        if is_key_down(KeyCode::LeftControl) && is_key_released(KeyCode::E) {
            bitmap = Bitmap::new();
        }

        if is_key_pressed(KeyCode::E) && !is_key_down(KeyCode::LeftControl) {
            is_eraser_on = true;
        } else if is_key_pressed(KeyCode::W) {
            active_element_type = ElementType::Water;
            is_eraser_on = false;
        } else if is_key_pressed(KeyCode::S) {
            active_element_type = ElementType::Sand;
            is_eraser_on = false;
        } else if is_key_pressed(KeyCode::T) {
            active_element_type = ElementType::Stone;
            is_eraser_on = false;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(cell) = &hovered_cell {
                if is_eraser_on {
                    bitmap.clear(cell.x, cell.y);
                } else if bitmap.is_empty(cell.x as isize, cell.y as isize) {
                    bitmap.set(cell.x, cell.y, Some(Element::new(active_element_type)));
                }
            }
        }

        bitmap.update(frame);

        render(
            &bitmap,
            cell_width,
            cell_height,
            hovered_cell,
            &active_element_type,
            is_eraser_on,
        );

        frame += 1;
        next_frame().await
    }
}
