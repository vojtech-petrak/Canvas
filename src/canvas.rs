use macroquad::{
    window::*,
    input::{MouseButton, is_mouse_button_down, mouse_position, mouse_wheel},
};
use crate::vec_2::Vec2;

#[derive(Debug, Clone)]
pub struct Canvas {
    scale: f32,
    offset: Vec2<f32>,
    cursor_offset: Option<Vec2<f32>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            offset: - Vec2([screen_width(), screen_height()]) / 60.0,
            scale: 30.0,
            cursor_offset: None,
        }
    }

    pub fn update(&mut self) -> (Vec2<f32>, Vec2<i16>) {
        let mouse: Vec2<f32> = Vec2([mouse_position().0, mouse_position().1]);
        let cursor: Vec2<f32> = mouse / self.scale + self.offset;
        let _screen_size: Vec2<f32> = Vec2([screen_width(), screen_height()]);

        self.scale += mouse_wheel().1 / 1000.0 * self.scale;
        self.offset = cursor - mouse / self.scale;

        if is_mouse_button_down(MouseButton::Right) {
            match self.cursor_offset {
                Some(cursor_offset) => self.offset -= cursor - cursor_offset,
                None => self.cursor_offset = Some(cursor),
            }
        } else {
            self.cursor_offset = None;
        }
        (cursor, cursor.floor().i16())
    }

    pub fn get(&self) -> (f32, Vec2<f32>) {
        (self.scale, self.offset)
    }
}
