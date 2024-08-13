use std::{
    collections::BTreeMap,
    cmp::max,
};
use macroquad::{
    shapes::{draw_rectangle},
    color::*,
};

pub type Elements = BTreeMap<Vec2<i16>, Element>;

use crate::vec_2::{*, Axis::*};
use self::Direction1::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction1 {
    Start,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction2 {
    pub axis: Axis,
    pub direction_1: Direction1,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Element {
    Gate {
        calc: u16,
        rotation: Direction2,
    },
    Wire {
        lenght: u16,
        axis: Axis,
    },
}

impl Element {
    pub fn print(&self, position: Vec2<i16>, (scale, offset): (f32, Vec2<f32>), selected: bool) {
        let position: Vec2<f32> = (position.f32() - offset) * scale;
        let a: f32 = 0.5 + selected as i32 as f32 * 0.25;
        match self {
            Element::Gate { calc: _, rotation } => {
                draw_rectangle(
                    position[X],
                    position[Y],
                    2.0 * scale,
                    2.0 * scale,
                    Color { r: 0.99, g: 0.98, b: 0.0, a },
                );

                let mut x: f32 = position[rotation.axis];
                let mut y: f32 = position[rotation.axis.swap()] + 1.0 * scale;
                if rotation.direction_1 == End { x += 2.0 * scale; }
                if rotation.axis == Y { (x, y) = (y, x); }

                draw_rectangle(x - 0.1 * scale, y - 0.1 * scale, 0.2 * scale, 0.2 * scale, Color { r: 0.0, g: 0.47, b: 0.95, a: a + 0.25 })
            },
            Element::Wire { lenght, axis } => draw_rectangle(
                position[X],
                position[Y],
                max(1, lenght * axis.swap() as u16) as f32 * scale,
                max(1, lenght * *axis as u16) as f32 * scale,
                GREEN,
            ),
        }
    }
}
