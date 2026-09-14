use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::camera;
use crate::render::App;

pub struct Input {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,

    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_delta_x: f64,
    pub mouse_delta_y: f64,
    pub previous_mouse_x: f64,
    pub previous_mouse_y: f64,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            arrow_up: false,
            arrow_down: false,
            arrow_left: false,
            arrow_right: false,
            key_w: false,
            key_a: false,
            key_s: false,
            key_d: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0,
            previous_mouse_x: 0.0,
            previous_mouse_y: 0.0,
        }
    }
}

pub fn handle_keyboard_input(app: &mut App, event: KeyEvent, event_loop: &ActiveEventLoop) {
    match event.physical_key {
        PhysicalKey::Code(key) => {
            if event.state == ElementState::Pressed {
                println!("{:?}", key);
            }
            match key {
                KeyCode::Escape => {
                    println!("exiting");
                    event_loop.exit();
                }
                KeyCode::ArrowUp => app.input.arrow_up = event.state == ElementState::Pressed,
                KeyCode::ArrowDown => app.input.arrow_down = event.state == ElementState::Pressed,
                KeyCode::ArrowLeft => app.input.arrow_left = event.state == ElementState::Pressed,
                KeyCode::ArrowRight => app.input.arrow_right = event.state == ElementState::Pressed,
                KeyCode::KeyW => app.input.key_w = event.state == ElementState::Pressed,
                KeyCode::KeyA => app.input.key_a = event.state == ElementState::Pressed,
                KeyCode::KeyS => app.input.key_s = event.state == ElementState::Pressed,
                KeyCode::KeyD => app.input.key_d = event.state == ElementState::Pressed,
                _ => {}
            }
        }
        _ => {}
    }
}

// pub fn handle_mouse_input(app: &mut App, state: ElementState, button: MouseButton) {
//     println!("{:?}", state.is_pressed());
//     println!("{:?}", button);
// }

pub fn handle_cursor_move(app: &mut App, position: PhysicalPosition<f64>) {
    app.input.mouse_delta_x = position.x - app.input.previous_mouse_x;
    app.input.mouse_delta_y = position.y - app.input.previous_mouse_y;

    // println!("mouse delta: {} == {}", mouse_delta_x, mouse_delta_y);

    app.input.previous_mouse_x = position.x;
    app.input.previous_mouse_y = position.y;
}
