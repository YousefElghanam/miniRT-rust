use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState::Pressed, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::camera::Camera;
use crate::constants::{MIN_RENDER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::Input;
use crate::maths::Vec3;
use crate::player::Player;
use crate::renderer::Renderer;
use crate::scene::Scene;

pub struct App {
    pub scene: Scene,
    pub renderer: Renderer,
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub input: Input,
    pub camera: Camera,
    pub needs_render: bool,
    pub render_divisor: u32,
    pub player: Player,
    pub frame_count: u64,
    pub last_fps_update: Instant,
    pub last_frame: Instant,
}

fn move_object(app: &mut App, dt: f32) {
    let mut movement = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let speed = 2.0;

    if app.input.super_left && app.input.arrow_up {
        movement.z -= speed * dt;
    }
    if app.input.super_left && app.input.arrow_down {
        movement.z += speed * dt;
    }
    if app.input.arrow_up && !app.input.super_left {
        movement.y += speed * dt;
    }
    if app.input.arrow_down && !app.input.super_left {
        movement.y -= speed * dt;
    }
    if app.input.arrow_left {
        movement.x -= speed * dt;
    }
    if app.input.arrow_right {
        movement.x += speed * dt;
    }

    let moved = movement.length() > 0.0;
    if moved {
        if let Some(object) = app.scene.objects.first_mut() {
            let previous_position = object.position();
            object.set_position(previous_position.add(movement));
        }
        app.scene.rebuild_acceleration();
        app.needs_render = true;
    }
}

fn apply_render_resolution(app: &mut App, surface: PhysicalSize<u32>) {
    let width = surface.width / app.render_divisor;
    let height = surface.height / app.render_divisor;
    if width == 0 || height == 0 {
        return;
    }
    app.renderer.resize(width, height);
    if let Some(pixels) = app.pixels.as_mut() {
        pixels.resize_buffer(width, height).unwrap();
    }
    println!(
        "Render: W:({width}) H:({height}) divisor:({})",
        app.render_divisor
    );
    app.needs_render = true;
}

fn change_resolution(app: &mut App) {
    let Some(delta) = app.input.resolution_change.take() else {
        return;
    };
    let Some(window) = app.window.as_ref() else {
        return;
    };
    let surface = window.inner_size();

    let Some(candidate) = app.render_divisor.checked_add_signed(delta) else {
        return;
    };
    if candidate < 1 || surface.width / candidate < MIN_RENDER_WIDTH {
        return;
    }
    app.render_divisor = candidate;
    apply_render_resolution(app, surface);
}

fn move_camera(app: &mut App, dt: f32) {
    let camera_movement = app.camera.movement(&app.input, 2.0, dt);
    if !app.needs_render {
        app.needs_render = camera_movement.length() > 0.0
            || app.input.mouse_delta_x != 0.0
            || app.input.mouse_delta_y != 0.0;
    }
    app.camera.position = app.camera.position.add(camera_movement);
    app.camera.update(&app.input);
    app.input.mouse_delta_x = 0.0;
    app.input.mouse_delta_y = 0.0;
}

impl App {
    fn update(&mut self, dt: f32) {
        move_object(self, dt);
        change_resolution(self);
        move_camera(self, dt);

        // if let Some(window) = self.window.as_ref() {
        //     window.request_redraw();
        // }
        // next line is similar to the one before, revisit diff to test ur rust knowledge
        self.window
            .as_ref()
            .expect("window started after resumed()")
            .request_redraw();
    }

    fn render_frame(&mut self) {
        if !self.needs_render {
            return;
        }
        let start = Instant::now();
        self.renderer.render(&self.scene, &self.camera);
        if let Some(frame) = self.pixels.as_mut() {
            frame
                .frame_mut()
                .copy_from_slice(&self.renderer.frame_buffer);
            frame.render().unwrap();
        }
        self.record_presented_frame();
        println!(
            "Frametime: {:.2} ms",
            start.elapsed().as_secs_f64() * 1000.0
        );
        self.needs_render = false;
    }

    fn record_presented_frame(&mut self) {
        self.frame_count += 1;
        if self.last_fps_update.elapsed().as_secs_f32() >= 1.0 {
            println!("FPS: {}", self.frame_count);
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }
    }
}

fn handle_redraw_request(app: &mut App) {
    let dt = app.last_frame.elapsed().as_secs_f32();
    app.last_frame = Instant::now();
    app.update(dt);
    app.render_frame();
}

fn handle_resize(app: &mut App, size: PhysicalSize<u32>) {
    println!("W:({}) H:({})", size.width, size.height);
    if size.width > 0 && size.height > 0 {
        if let Some(pixels) = app.pixels.as_mut() {
            pixels.resize_surface(size.width, size.height).unwrap();
        }
        apply_render_resolution(app, size);
    }
}

pub fn handle_kb_input(app: &mut App, event: KeyEvent, event_loop: &ActiveEventLoop) {
    match event.physical_key {
        PhysicalKey::Code(key) => {
            // if event.state == ElementState::Pressed {
            //     println!("{:?}", key);
            // }
            match key {
                KeyCode::Escape => {
                    println!("exiting");
                    event_loop.exit();
                }
                KeyCode::ArrowUp => app.input.arrow_up = event.state == Pressed,
                KeyCode::ArrowDown => app.input.arrow_down = event.state == Pressed,
                KeyCode::ArrowLeft => app.input.arrow_left = event.state == Pressed,
                KeyCode::ArrowRight => app.input.arrow_right = event.state == Pressed,
                KeyCode::SuperLeft => app.input.super_left = event.state == Pressed,
                KeyCode::KeyW => app.input.key_w = event.state == Pressed,
                KeyCode::KeyA => app.input.key_a = event.state == Pressed,
                KeyCode::KeyS => app.input.key_s = event.state == Pressed,
                KeyCode::KeyD => app.input.key_d = event.state == Pressed,
                KeyCode::KeyO
                    if app.input.super_left && event.state == Pressed && !event.repeat =>
                {
                    app.input.resolution_change = Some(-1)
                }
                KeyCode::KeyL
                    if app.input.super_left && event.state == Pressed && !event.repeat =>
                {
                    app.input.resolution_change = Some(1);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("engmaxxx")
                        .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                )
                .unwrap(),
        );
        let surface_texture =
            SurfaceTexture::new(self.renderer.width, self.renderer.height, window.clone());
        let pixels =
            Pixels::new(self.renderer.width, self.renderer.height, surface_texture).unwrap();
        let surface = window.inner_size();
        self.window = Some(window);
        self.pixels = Some(pixels);
        apply_render_resolution(self, surface);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => handle_redraw_request(self),
            WindowEvent::Resized(size) => handle_resize(self, size),
            WindowEvent::KeyboardInput { event, .. } => handle_kb_input(self, event, event_loop),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.input.mouse_delta_x += delta.0;
            self.input.mouse_delta_y += delta.1;
        }
    }
}
