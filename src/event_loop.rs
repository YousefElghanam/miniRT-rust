use std::time::Instant;

use winit;

use crate::app::App;
use crate::camera::Camera;
use crate::constants::{DEFAULT_RENDER_DIVISOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::Input;
use crate::player::Player;
use crate::renderer::Renderer;
use crate::scene::Scene;

pub fn render(scene: Scene) {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut application = App {
        scene,
        // Placeholder size; resumed() sets the real one from the window's physical size.
        renderer: Renderer::new(
            WINDOW_WIDTH / DEFAULT_RENDER_DIVISOR,
            WINDOW_HEIGHT / DEFAULT_RENDER_DIVISOR,
        ),
        window: None,
        pixels: None,
        input: Input::default(),
        camera: Camera::default(),
        needs_render: true,
        render_divisor: DEFAULT_RENDER_DIVISOR,
        player: Player { x: 0.0, speed: 5.0 },
        frame_count: 0,
        last_fps_update: Instant::now(),
        last_frame: Instant::now(),
    };
    event_loop.run_app(&mut application).unwrap();
}
