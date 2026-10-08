use winit::{
    event::{ElementState, KeyEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::app::App;

pub struct Input {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub super_left: bool,
    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,
    pub resolution_change: Option<i32>,

    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_delta_x: f64,
    pub mouse_delta_y: f64,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            arrow_up: false,
            arrow_down: false,
            arrow_left: false,
            arrow_right: false,
            super_left: false,
            key_w: false,
            key_a: false,
            key_s: false,
            key_d: false,
            resolution_change: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,
        }
    }
}

// pub fn handle_mouse_input(app: &mut App, state: ElementState, button: MouseButton) {
//     println!("{:?}", state.is_pressed());
//     println!("{:?}", button);
// }

// pub fn handle_cursor_move(app: &mut App, position: PhysicalPosition<f64>) {
//     app.input.mouse_delta_x = position.x - app.input.previous_mouse_x;
//     app.input.mouse_delta_y = position.y - app.input.previous_mouse_y;

//     // println!("mouse delta: {} == {}", mouse_delta_x, mouse_delta_y);

//     app.input.previous_mouse_x = position.x;
//     app.input.previous_mouse_y = position.y;
// }
