use std::collections::BTreeMap;
use macroquad::{
    input::{MouseButton, is_mouse_button_pressed,  is_mouse_button_released, KeyCode, is_key_pressed, is_key_down},
    shapes::draw_rectangle_lines,
    color::*,
};

use crate::{
    vec_2::{Vec2, Axis::*},
    element::{*, Direction1::*},
};

#[derive(Debug)]
pub enum Selection {
    Selecting { start: Vec2<f32> },
    Selected { elements: Elements, offset: Vec2<i16>, previous_cursor: Option<Vec2<i16>> },
}

impl Selection {
    pub fn new() -> Self {
        Selection::Selected { elements: BTreeMap::new(), offset: Vec2([0, 0]), previous_cursor: None }
    }

    pub fn update(&mut self, elements: &mut Elements, (cursor, grid_cursor): (Vec2<f32>, Vec2<i16>)) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let Selection::Selected { elements: selected, offset, previous_cursor } = self else { return; };
            match selected.contains_element(grid_cursor - *offset) && *previous_cursor == None {
                true => *previous_cursor = Some(grid_cursor),
                false => {
                    selected.unselect(elements, *offset);
                    *self = match elements.get_element(grid_cursor) {
                        Some((position, _)) => Selection::Selected {
                            elements: BTreeMap::from([(grid_cursor, elements.remove(&position.clone()).unwrap())]),
                            offset: Vec2([0, 0]),
                            previous_cursor: Some(grid_cursor),
                        },
                        None => Selection::Selecting { start: cursor },
                    };
                },
            }

        } else if is_mouse_button_released(MouseButton::Left) {
            match self {
                Selection::Selecting { start } => {
                    let mut selected: BTreeMap<Vec2<i16>, Element> = BTreeMap::new();

                    for x in start[X].min(cursor[X]).ceil() as i16..start[X].max(cursor[X]).floor() as i16 {
                        let mut y: i16 = start[Y].min(cursor[Y]).ceil() as i16;

                        loop {
                            let Some((position, ..)) = elements
                                .range(Vec2([x, y])..Vec2([x, start[Y].max(cursor[Y]).floor() as i16]))
                                .next() else { break; };
                            y = position[Y];
                            let position: Vec2<i16> = *position;
                            selected.insert(position, elements.remove(&position).unwrap());
                        }
                    }
                    *self = Selection::Selected { elements: selected, offset: Vec2([0, 0]), previous_cursor: None };
                },
                Selection::Selected { previous_cursor, .. } => *previous_cursor = None,
            }

        } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            if let Selection::Selected { elements: selected, offset, .. } = self {
                selected.unselect(elements, *offset);
            }
            *self = Selection::Selected {
                elements: BTreeMap::from([(grid_cursor, Element::Gate { calc: 0, rotation: {
                    if is_key_pressed(KeyCode::Left) { Direction2 { axis: X, direction_1: Start } }
                    else if is_key_pressed(KeyCode::Right) { Direction2 { axis: X, direction_1: End } }
                    else if is_key_pressed(KeyCode::Up) { Direction2 { axis: Y, direction_1: Start } }
                    else { Direction2 { axis: Y, direction_1: End } }
                }})]),
                offset: Vec2([0, 0]),
                previous_cursor: Some(grid_cursor),
            };

        } else if is_key_pressed(KeyCode::Delete) {
            *self = Selection::new();

        } else if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
            if is_key_pressed(KeyCode::D) {
                if let Selection::Selected { elements: selected, offset, previous_cursor } = self {
                    for (position, element) in selected {
                        elements.insert(*position + *offset, element.clone());
                    }
                    if let None = previous_cursor {
                        *previous_cursor = Some(grid_cursor);
                    }
                }
            }
        }

        if let Selection::Selected { offset, previous_cursor: Some(previous_cursor), .. } = self {
            *offset += grid_cursor - *previous_cursor;
            *previous_cursor = grid_cursor;
        }
    }
    
    pub fn print(&self, (scale, offset): (f32, Vec2<f32>), cursor: Vec2<f32>) {
        match self {
            Selection::Selecting { start } => draw_rectangle_lines(
                (start[X].min(cursor[X]) - offset[X]) * scale,
                (start[Y].min(cursor[Y]) - offset[Y]) * scale,
                (cursor[X] - start[X]).abs() * scale,
                (cursor[Y] - start[Y]).abs() * scale,
                4.0,
                WHITE
            ),
            Selection::Selected { elements: selected, offset: elements_offset, .. } => for (position, element) in selected {
                element.print(*position + *elements_offset, (scale, offset), true);
            },
        }
    }
}

trait Select {
    fn unselect(&mut self, elements: &mut Elements, offset: Vec2<i16>);
    fn get_element(&self, position: Vec2<i16>) -> Option<(&Vec2<i16>, &Element)>;
    fn contains_element(&self, position: Vec2<i16>) -> bool;
}
impl Select for Elements {
    fn unselect(&mut self, elements: &mut Elements, offset: Vec2<i16>) {
        loop {
            let Some((position, element)) = self.pop_last() else { break; };
            elements.insert(position + offset, element);
        }
    }
    fn get_element(&self, position: Vec2<i16>) -> Option<(&Vec2<i16>, &Element)> {
        if let Some(positioned_element) = self.range(position - Vec2([0, 1])..=position).next() {
            return Some(positioned_element);
        } else if let Some(positioned_element) = self.range(position - 1..=position - Vec2([1, 0])).next() {
            return Some(positioned_element);
        } else {
            return None;
        }
    }
    fn contains_element(&self, position: Vec2<i16>) -> bool {
        match self.get_element(position) {
            Some(_) => true,
            None => false,
        }
    }
}
