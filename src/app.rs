use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    window::Window,
};

use crate::camera::Camera;
use crate::constants::{
    RENDER_ASPECT_RATIO, RENDER_RES_H, RENDER_RES_W, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::input::{self, Input};
use crate::maths::Vec3;
use crate::renderer::Renderer;
use crate::scene::Scene;

pub struct Player {
    x: f32,
    speed: f32,
}

pub struct App {
    pub scene: Scene,
    pub renderer: Renderer,
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub input: Input,
    pub camera: Camera,
    pub needs_render: bool,
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

    if let Some(object) = app.scene.objects.first_mut() {
        let previous_position = object.position();
        object.set_position(previous_position.add(movement));
    }
    app.needs_render = movement.length() > 0.0;
}

fn change_resolution(app: &mut App) {
    let scale = if app.input.key_o && app.input.super_left {
        Some(1.2)
    } else if app.input.key_l && app.input.super_left {
        Some(0.8)
    } else {
        None
    };

    let Some(scale) = scale else {
        return;
    };

    let new_width = (app.renderer.width as f32 * scale) as u32;
    let new_height = (new_width as f32 / RENDER_ASPECT_RATIO) as u32;
    if !(60..=2000).contains(&new_width) {
        return;
    }

    app.renderer.resize(new_width, new_height);
    if let Some(pixels) = app.pixels.as_mut() {
        pixels.resize_buffer(new_width, new_height).unwrap();
    }
    println!("W:({new_width}) H:({new_height})");
}

impl App {
    fn update(&mut self, dt: f32) {
        move_object(self, dt);
        if self.input.res_up_pressed || self.input.res_down_pressed {
            change_resolution(self);
            self.input.res_up_pressed = false;
            self.input.res_down_pressed = false;
        }

        let camera_movement = self.camera.movement(&self.input, 2.0, dt);
        if !self.needs_render {
            self.needs_render = camera_movement.length() > 0.0
                || self.input.mouse_delta_x != 0.0
                || self.input.mouse_delta_y != 0.0;
        }

        self.camera.position = self.camera.position.add(camera_movement);
        self.camera.update(&self.input);
        self.player.x += self.player.speed * dt;
        self.input.mouse_delta_x = 0.0;
        self.input.mouse_delta_y = 0.0;

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
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
        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                input::handle_keyboard_input(self, event, event_loop)
            }
            WindowEvent::RedrawRequested => handle_redraw_request(self),
            WindowEvent::Resized(size) => {
                println!("W:({}) H:({})", size.width, size.height);
                if size.width > 0 && size.height > 0 {
                    if let Some(pixels) = self.pixels.as_mut() {
                        pixels.resize_surface(size.width, size.height).unwrap();
                    }
                }
            }
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

pub fn create(scene: Scene) -> App {
    App {
        scene,
        renderer: Renderer::new(RENDER_RES_W, RENDER_RES_H),
        window: None,
        pixels: None,
        input: Input::default(),
        camera: Camera::default(),
        needs_render: false,
        player: Player { x: 0.0, speed: 5.0 },
        frame_count: 0,
        last_fps_update: Instant::now(),
        last_frame: Instant::now(),
    }
}
