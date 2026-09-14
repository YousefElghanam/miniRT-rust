use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop},
    window::Window,
};

use crate::camera::Camera;
use crate::constants::{AMBIENT, PPM_HEIGHT, PPM_WIDTH};
use crate::elements::{Hittable, Light, Sphere};
use crate::input;
use crate::input::Input;
use crate::maths::{Color, Hit, Ray, Vec3};
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
    };
    if let Some(shadow_hit) = scene.intersect(&shadow_ray) {
        if shadow_hit.t < light_distance {
            return 0.0;
        }
    }
    hit.normal.dot(light_direction).max(0.0)
}

#[derive(Debug, Copy, Clone)]
pub struct Renderer {
    width: u32,
    height: u32,
}

impl Renderer {
    fn ray_for_pixel(&mut self, x: u32, y: u32) -> Ray {
        let u = (x as f32 + 0.5) / self.width as f32;
        let v = (y as f32 + 0.5) / self.height as f32;

        let screen_x = u * 2.0 - 1.0;
        let screen_y = 1.0 - v * 2.0;

        let direction = Vec3 {
            x: screen_x,
            y: screen_y,
            z: -1.0,
        }
        .normalize();

        Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction,
        }
    }
    fn render(&mut self, scene: &Scene) -> Vec<Color> {
        let mut pixels = Vec::new();
        for y in 0..PPM_HEIGHT {
            for x in 0..PPM_WIDTH {
                let ray = self.ray_for_pixel(x, y);

                let color = match scene.intersect(&ray) {
                    Some(hit) => {
                        let diffuse =
                            calculate_lighting(&hit, &scene.lights.first().unwrap(), &scene); // make multiple lights
                        let brightness = (AMBIENT + diffuse).min(1.0); // put AMBIENT in scene file
                        Color::from_brightness(brightness, &hit.material)
                    }
                    None => Color { r: 0, g: 0, b: 0 },
                };
                pixels.push(color);
            }
        }
        pixels
        // println!("rendering...");
    }
}

pub fn save_ppm(pixels: &Vec<Color>) {
    let mut file = File::create("output.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", PPM_WIDTH, PPM_HEIGHT).unwrap();
    writeln!(file, "255").unwrap();

    for &pixel in pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
    }
}

impl App {
    fn update(&mut self, dt: f32) {
        self.player.x += self.player.speed * dt;
        self.camera.update(&self.input); // RECHECK THE ARGUMENT TYPE HERE LATER
        self.input.mouse_delta_x = 0.0;
        self.input.mouse_delta_y = 0.0;
        let sphere = self.scene.objects.first_mut().unwrap();
        let old_pos = sphere.position();
        if self.input.arrow_up {
            sphere.set_position(old_pos.add(Vec3 {
                x: 0.0,
                y: 0.05,
                z: 0.0,
            }));
        }
        if self.input.arrow_down {
            sphere.set_position(old_pos.add(Vec3 {
                x: 0.0,
                y: -0.05,
                z: 0.0,
            }));
        }
        if self.input.arrow_left {
            sphere.set_position(old_pos.add(Vec3 {
                x: -0.05,
                y: 0.0,
                z: 0.0,
            }));
        }
        if self.input.arrow_right {
            sphere.set_position(old_pos.add(Vec3 {
                x: 0.05,
                y: 0.0,
                z: 0.0,
            }));
        }
    }
    fn render(&mut self) {
        let pixels = self.renderer.render(&self.scene);
        if let Some(frame) = self.pixels.as_mut() {
            let frame_buffer = frame.frame_mut();
            for (i, color) in pixels.iter().enumerate() {
                let index = i * 4;
                frame_buffer[index] = color.r;
                frame_buffer[index + 1] = color.g;
                frame_buffer[index + 2] = color.b;
                frame_buffer[index + 3] = 255;
            }
            frame.render().unwrap();
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
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            self.renderer.width,
                            self.renderer.height,
                        )),
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
            WindowEvent::CursorMoved { position, .. } => input::handle_cursor_move(self, position),
            // The game loop
            WindowEvent::RedrawRequested => {
                handle_redraw_request(self);
            }
            // WindowEvent::MouseInput {
            //     device_id,
            //     state,
            //     button,
            // } => input::handle_mouse_input(self, state, button),
            _ => {}
        }
    }
}

pub fn render(scene: Scene) {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let mut app = App {
        scene: scene,
        renderer: Renderer {
            width: PPM_WIDTH,
            height: PPM_HEIGHT,
        },
        window: None,
        pixels: None,
        input: Input::default(),
        camera: Camera::default(),
        player: Player { x: 0.0, speed: 5.0 },
        frame_count: 0,
        last_fps_update: Instant::now(),
        last_frame: Instant::now(),
    };
    event_loop.run_app(&mut app).unwrap();
}
