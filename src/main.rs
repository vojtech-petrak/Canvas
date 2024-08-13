#![windows_subsystem = "windows"]

mod vec_2;
mod element;
mod canvas;
mod selection;

//use std::collections::BTreeMap;
use macroquad::{
    window::*,
    shapes::{draw_rectangle, draw_circle},
    color::*, text::draw_text,
};
use crate::{
    vec_2::{Vec2, Axis::*},
    canvas::*,
    element::*,
    selection::*,
};

#[macroquad::main("Logic simulator")]
async fn main() {
    let mut canvas: Canvas = Canvas::new();
    let mut elements: Elements = Elements::new();
    let mut selection: Selection = Selection::new();
    
    loop {
        let cursor: (Vec2<f32>, Vec2<i16>) = canvas.update();
        selection.update(&mut elements, cursor);
        
        let unit: u32 = print_dots(canvas.get(), 64.0);
        for (position, element) in &elements {
            element.print(*position, canvas.get(), false);
        }
        selection.print(canvas.get(), cursor.0);
        print_ui(elements.len() + match &selection {
            Selection::Selecting { .. } => 0,
            Selection::Selected { elements, .. } => elements.len(),
        }, unit);
        
        next_frame().await;
    }
}

pub fn print_dots((scale, offset): (f32, Vec2<f32>), dot_density: f32) -> u32 {
    let unit: u32 = (dot_density / scale + 1.0).log2() as u32;
    let dot_skip: f32 = 2_u16.pow(unit) as f32;
    let dot_offset: Vec2<f32> = offset.trunc() - offset - offset.trunc() % dot_skip;

    for x in 0..=(screen_width() / scale / dot_skip) as i16 {
        let x_position: f32 = (x as f32 * dot_skip + dot_offset[X]) * scale - 1.0;
        
        for y in 0..=(screen_height() / scale / dot_skip) as i16 {
            draw_rectangle(
                x_position,
                (y as f32 * dot_skip + dot_offset[Y]) * scale - 1.0,
                2.0,
                2.0,
                WHITE
            );
        }
    }

    draw_circle(- offset[X] * scale, - offset[Y] * scale, 4.0, WHITE);
    unit
}
pub fn print_ui(element_count: usize, unit: u32) {
    draw_rectangle(0.0, screen_height() - 30.0, screen_width(), 30.0, Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 });
    draw_text(&("unit: ".to_owned() + &unit.to_string()), 10.0, screen_height() - 10.0, 20.0, WHITE);
    draw_text(
        &("elements: ".to_owned() + &element_count.to_string()),
        90.0,
        screen_height() - 10.0,
        20.0,
        WHITE
    );
}
