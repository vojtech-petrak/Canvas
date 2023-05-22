use std::{
    cmp::{min, max},
};
use macroquad::prelude::*;

const X: usize = 0;
const Y: usize = 1;
static mut SCALE: f32 = 1.0;
static mut CANVAS_OFFSET: [f32; 2] = [0.0; 2];
static mut SCREEN_SIZE: [f32; 2] = [0.0; 2];

#[derive(Clone, Copy)]
enum Conversion {
    ToScreen = 1,
    ToDescription = -1,
} 
trait Convert {
    fn convert(self, conversion: Conversion) -> Self;
    fn transform(self, conversion: Conversion) -> Self;
}
impl Convert for [f32; 2] {
    fn convert(self, conversion: Conversion) -> Self {
        let mut output: [f32; 2] = [0.0; 2];
        for i in 0..2 {
            output[i] = self[i] * unsafe { SCALE }.powi(conversion as i32);
        }
        output
    }
    fn transform(mut self, conversion: Conversion) -> Self {
        for i in 0..2 * max(conversion as isize, 0) as usize {
            self[i] -= unsafe { CANVAS_OFFSET }[i];
        }
        self = self.convert(conversion);
        for i in min(conversion as i32 + 1, 1) as usize * 2..2 {
            self[i] += unsafe { CANVAS_OFFSET }[i];
        }
        self
    }
}

struct Rectangle {
    position: [f32; 2],
    size: [f32; 2],
    color: Color,
}
struct Drag {
    offset: [f32; 2],
    element: usize,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Shapes".to_owned(),
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut rectangles: Vec<Rectangle> = Vec::new();
    let mut cursor_offset: Option<Drag> = None;
    let mut canvas_cursor_offset: Option<[f32; 2]> = None;
    
    unsafe {
        SCREEN_SIZE = [screen_width(), screen_height()];
        CANVAS_OFFSET = [- SCREEN_SIZE[X] / 2.0, - SCREEN_SIZE[Y] / 2.0];
        //CANVAS_OFFSET = - SCREEN_SIZE / 2.0;
    }

    loop {
        unsafe {
            SCALE *= screen_width() / SCREEN_SIZE[X];
            SCREEN_SIZE = [screen_width(), screen_height()];
        }

        let mut cursor_position: [f32; 2] = [0.0; 2];
        (cursor_position[X], cursor_position[Y]) = mouse_position();
        cursor_position = cursor_position.transform(Conversion::ToDescription);

        // rectangle move
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(Drag { offset, element }) = cursor_offset {
                rectangles[element].position[X] = cursor_position[X] - offset[X];
                rectangles[element].position[Y] = cursor_position[Y] - offset[Y];
            } else {
                for (index, rectangle) in rectangles.iter().enumerate() {
                    let offset: [f32; 2] = [
                        cursor_position[X] - rectangle.position[X],
                        cursor_position[Y] - rectangle.position[Y],
                    ];
                    if offset[X] >= 0.0 && offset[X] <= rectangle.size[X]
                    && offset[Y] >= 0.0 && offset[Y] <= rectangle.size[Y] {
                        cursor_offset = Some(Drag { offset, element: index });
                        break;
                    } else {
                        cursor_offset = None;
                    }
                }
            }
        } else {
            cursor_offset = None;
        }

        // zoom
        unsafe {
            SCALE += mouse_wheel().1 / 1000.0 * SCALE;
            CANVAS_OFFSET[X] = cursor_position[X] - mouse_position().0 / SCALE;
            CANVAS_OFFSET[Y] = cursor_position[Y] - mouse_position().1 / SCALE;
        }

        // canvas move
        if is_mouse_button_down(MouseButton::Right) {
            if let Some(cursor_offset) = canvas_cursor_offset {
                unsafe {
                    CANVAS_OFFSET[X] -= cursor_position[X] - cursor_offset[X];
                    CANVAS_OFFSET[Y] -= cursor_position[Y] - cursor_offset[Y];
                }
            } else {
                canvas_cursor_offset = Some([
                     cursor_position[X],
                     cursor_position[Y],
                ]);
            }
        } else {
            canvas_cursor_offset = None;
        }

        // rectangle add
        if is_key_pressed(KeyCode::Insert) {
            let mut position: [f32; 2] = [screen_width() / 2.0, screen_height() / 2.0].transform(Conversion::ToDescription);
            position[X] -= 20.0;
            position[Y] -= 20.0;

            rectangles.push(Rectangle {
                position,
                size: [40.0; 2],
                color: YELLOW,
            });
        // rectangle remove
        } else if is_key_pressed(KeyCode::Delete) {
            rectangles.pop();
        }

        // draw
        clear_background(Color { r: 0.0, g: 0.0, b: 0.1, a: 1.0 });
        for rectangle in &mut rectangles {
            let position: [f32; 2] = rectangle.position.transform(Conversion::ToScreen);
            let size: [f32; 2] = rectangle.size.convert(Conversion::ToScreen);
            draw_rectangle(
                position[X],
                position[Y],
                size[X],
                size[Y],
                rectangle.color,
            );
        }
        {
            let start: [f32; 2] = [0.0; 2].transform(Conversion::ToScreen);
            draw_circle(start[X], start[Y], 5.0, WHITE);
        }

        next_frame().await
    }
}
