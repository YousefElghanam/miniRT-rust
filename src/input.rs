use crate::render::App;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::camera;

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,

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
            forward: false,
            backward: false,
            left: false,
            right: false,
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
                KeyCode::ArrowUp => app.input.forward = event.state == ElementState::Pressed,
                KeyCode::ArrowDown => app.input.backward = event.state == ElementState::Pressed,
                KeyCode::ArrowLeft => app.input.left = event.state == ElementState::Pressed,
                KeyCode::ArrowRight => app.input.right = event.state == ElementState::Pressed,
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
