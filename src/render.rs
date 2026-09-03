use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{self, ActiveEventLoop},
    window::Window,
};

use crate::camera::Camera;
use crate::input;
use crate::input::Input;

pub struct Player {
    x: f32,
    speed: f32,
}

pub struct App {
    window: Option<Window>,
    pub input: Input,
    pub camera: Camera,
    pub player: Player,

    pub frame_count: u64,
    pub last_fps_update: Instant,
    pub last_frame: Instant,
    // pub fps: u64,
}

impl App {
    fn update(&mut self, dt: f32) {
        self.player.x += self.player.speed * dt;
        self.camera.update(&self.input); // RECHECK THE ARGUMENT TYPE HERE LATER
        self.input.mouse_delta_x = 0.0;
        self.input.mouse_delta_y = 0.0;
    }
    fn render(&mut self) {
        // println!("rendering...");
    }
}

fn count_fps(app: &mut App) {
    app.frame_count += 1;

    if app.last_fps_update.elapsed().as_secs_f32() >= 1.0 {
        println!("FPS: {}", app.frame_count);

        app.frame_count = 0;
        app.last_fps_update = Instant::now();
    }
}

fn count_dt(app: &mut App) -> f32 {
    let dt = app.last_frame.elapsed().as_secs_f32();
    app.last_frame = Instant::now();
    // println!("dt: {}", dt);
    dt
}

fn handle_redraw_request(app: &mut App) {
    count_fps(app);
    let dt = count_dt(app);

    app.update(dt);
    app.render();
    if let Some(window) = app.window.as_ref() {
        window.request_redraw();
    }
}
// through this trait, winit gets access to our Apps
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("engmaxx")
                        .with_inner_size(winit::dpi::LogicalSize::new(1270, 720)),
                )
                .unwrap(),
        )
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => input::handle_keyboard_input(self, event, event_loop),
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => input::handle_cursor_move(self, position),
            // The game loop
            WindowEvent::RedrawRequested => handle_redraw_request(self),
            // WindowEvent::MouseInput {
            //     device_id,
            //     state,
            //     button,
            // } => input::handle_mouse_input(self, state, button),
            _ => {}
        }
    }
}

pub fn render() {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        input: Input::default(),
        camera: Camera::default(),
        player: Player { x: 0.0, speed: 5.0 },
        frame_count: 0,
        last_fps_update: Instant::now(),
        last_frame: Instant::now(),
    };
    event_loop.run_app(&mut app).unwrap();
}
