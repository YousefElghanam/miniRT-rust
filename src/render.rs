use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture, wgpu::wgc::command::EncoderStateError::Locked};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{self, ActiveEventLoop},
    window::Window,
};

use crate::constants::{AMBIENT, RENDER_RES_H, RENDER_RES_W, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::elements::Light;
use crate::input;
use crate::input::Input;
use crate::maths::{Color, Hit, Ray, Vec3};
use crate::scene::Scene;
use crate::{camera::Camera, constants::RENDER_ASPECT_RATIO};

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
    // pub fps: u64,
}

pub fn calculate_lighting(hit: &Hit, light: &Light, scene: &Scene) -> f32 {
    // let light_direction = light.position.sub(hit.point).normalize();
    let to_light = light.position.sub(hit.point);
    let light_distance = to_light.length();
    let light_direction = to_light.normalize();

    let shadow_origin = hit.point.add(hit.normal.scale(0.001));

    let shadow_ray = Ray {
        origin: shadow_origin,
        direction: light_direction,
        inv_direction: Vec3 {
            x: 1.0 / light_direction.x,
            y: 1.0 / light_direction.y,
            z: 1.0 / light_direction.z,
        },
    };
    if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
        if shadow_hit.t < light_distance {
            return 0.0;
        }
    }
    hit.normal.dot(light_direction).max(0.0)
}

#[derive(Debug, Clone)]
pub struct Renderer {
    width: u32,
    height: u32,
    frame_buffer: Vec<u8>,
}

impl Renderer {
    fn ray_for_pixel(&mut self, x: u32, y: u32, camera: &Camera) -> Ray {
        let u = (x as f32 + 0.5) / self.width as f32;
        let v = (y as f32 + 0.5) / self.height as f32;

        let aspect_ratio = self.width as f32 / self.height as f32;

        let fov = camera.fov.to_radians();
        let scale = (fov / 2.0).tan();

        let screen_x = (u * 2.0 - 1.0) * aspect_ratio * scale;
        let screen_y = 1.0 - v * 2.0 * scale;

        let (forward, right, up) = camera.basis();
        let direction = forward
            .add(right.scale(screen_x))
            .add(up.scale(screen_y))
            .normalize();
        Ray {
            origin: camera.position,
            direction,
            inv_direction: Vec3 {
                x: 1.0 / direction.x,
                y: 1.0 / direction.y,
                z: 1.0 / direction.z,
            },
        }
    }
    fn render(&mut self, scene: &Scene, camera: &Camera) {
        let mut intersection_time = 0.0;
        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;

                // let start = Instant::now();
                let ray = self.ray_for_pixel(x, y, camera);
                // let intersection_time = start.elapsed().as_secs_f64() * 1000.0;
                // println!("ray_for_pixel() time: {:.2}", intersection_time);

                let start = Instant::now();

                let color = match scene.intersect(&ray) {
                    Some(hit) => {
                        let diffuse =
                            calculate_lighting(&hit, &scene.lights.first().unwrap(), &scene); // make multiple lights
                        let brightness = (AMBIENT + diffuse).min(1.0); // put AMBIENT in scene file
                        Color::from_brightness(brightness, &hit.material)
                    }
                    None => Color { r: 0, g: 0, b: 0 },
                };

                intersection_time += start.elapsed().as_secs_f64() * 1000.0;

                self.frame_buffer[index] = color.r;
                self.frame_buffer[index + 1] = color.g;
                self.frame_buffer[index + 2] = color.b;
                self.frame_buffer[index + 3] = 255;
            }
        }
        // println!("rendering...");
        println!("Accumalted intersections time: {:.2}", intersection_time);
    }
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.frame_buffer.resize((width * height * 4) as usize, 0);
    }
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
        let prev_pos = object.position();
        object.set_position(prev_pos.add(movement));
    }
    app.needs_render = movement.length() > 0.0;
}

fn change_resolution(app: &mut App) {
    if app.input.key_o && app.input.super_left {
        // app.renderer.increase_resolution();
        let new_width = (app.renderer.width as f32 * 1.2) as u32;
        let new_height = (new_width as f32 / RENDER_ASPECT_RATIO) as u32;
        if new_width > 2000 {
            return;
        }
        app.renderer.resize(new_width, new_height);
        if let Some(pixels) = app.pixels.as_mut() {
            pixels.resize_buffer(new_width, new_height).unwrap();
        }
        println!("W:({}) H:({})", new_width, new_height);
    } else if app.input.key_l && app.input.super_left {
        // app.renderer.increase_resolution();
        let new_width = (app.renderer.width as f32 * 0.8) as u32;
        let new_height = (new_width as f32 / RENDER_ASPECT_RATIO) as u32;
        if new_width < 60 {
            return;
        }
        app.renderer.resize(new_width, new_height);
        if let Some(pixels) = app.pixels.as_mut() {
            pixels.resize_buffer(new_width, new_height).unwrap();
        }
        println!("W:({}) H:({})", new_width, new_height);
    }
}

impl App {
    fn update(&mut self, dt: f32) {
        move_object(self, dt);
        if self.input.res_up_pressed || self.input.res_down_pressed {
            change_resolution(self);
            self.input.res_down_pressed = false;
            self.input.res_up_pressed = false;
        }

        let camera_movement = self.camera.movement(&self.input, 2.0, dt);

        if !self.needs_render {
            self.needs_render = camera_movement.length() > 0.0
                || self.input.mouse_delta_x != 0.0
                || self.input.mouse_delta_y != 0.0;
        }
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
        self.camera.position = self.camera.position.add(camera_movement);

        self.camera.update(&self.input); // RECHECK THE ARGUMENT TYPE HERE LATER

        self.player.x += self.player.speed * dt;
        self.input.mouse_delta_x = 0.0;
        self.input.mouse_delta_y = 0.0;
    }
    fn render(&mut self) {
        if self.needs_render {
            let start = Instant::now();
            self.renderer.render(&self.scene, &self.camera);
            if let Some(frame) = self.pixels.as_mut() {
                let frame_buffer = frame.frame_mut();
                frame_buffer.copy_from_slice(&self.renderer.frame_buffer);
                frame.render().unwrap();
            }
            let render_time = start.elapsed().as_secs_f64() * 1000.0;
            println!("Frametime: {:.2} ms", render_time);
            self.needs_render = false;
        }
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
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("engmaxxx")
                        .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                )
                .unwrap(),
        );
        // window.set_cursor_visible(false);
        // window
        //     .set_cursor_grab(winit::window::CursorGrabMode::Locked)
        //     .ok();
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
            WindowEvent::RedrawRequested => {
                handle_redraw_request(self);
            }
            WindowEvent::Resized(size) => {
                println!("W:({}) H:({})", size.width, size.height);
                if size.width == 0 || size.height == 0 {
                    return;
                }
                if let Some(pixels) = self.pixels.as_mut() {
                    pixels.resize_surface(size.width, size.height).unwrap();
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
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.input.mouse_delta_x += delta.0;
                self.input.mouse_delta_y += delta.1;
            }
            _ => {}
        }
    }
}

pub fn render(scene: Scene) {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let mut app = App {
        scene: scene,
        renderer: Renderer {
            width: RENDER_RES_W,
            height: RENDER_RES_H,
            frame_buffer: vec![0; (RENDER_RES_W * RENDER_RES_H * 4) as usize],
        },
        window: None,
        pixels: None,
        input: Input::default(),
        camera: Camera::default(),
        needs_render: false,
        player: Player { x: 0.0, speed: 5.0 },
        frame_count: 0,
        last_fps_update: Instant::now(),
        last_frame: Instant::now(),
    };
    event_loop.run_app(&mut app).unwrap();
}
