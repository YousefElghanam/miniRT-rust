use std::time::Instant;

use crate::camera::Camera;
use crate::constants::AMBIENT;
use crate::elements::Light;
use crate::maths::{Color, Hit, Ray, Vec3};
use crate::scene::Scene;

pub fn calculate_lighting(hit: &Hit, light: &Light, scene: &Scene) -> f32 {
    let to_light = light.position.sub(hit.point);
    let light_distance = to_light.length();
    let light_direction = to_light.normalize();

    let shadow_ray = Ray {
        origin: hit.point.add(hit.normal.scale(0.001)),
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
    pub width: u32,
    pub height: u32,
    pub frame_buffer: Vec<u8>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            frame_buffer: vec![0; (width * height * 4) as usize],
        }
    }

    fn ray_for_pixel(&self, x: u32, y: u32, camera: &Camera) -> Ray {
        let u = (x as f32 + 0.5) / self.width as f32;
        let v = (y as f32 + 0.5) / self.height as f32;
        let aspect_ratio = self.width as f32 / self.height as f32;
        let scale = (camera.fov.to_radians() / 2.0).tan();
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

    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        let mut intersection_time = 0.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;
                let ray = self.ray_for_pixel(x, y, camera);
                let start = Instant::now();

                let color = match scene.intersect(&ray) {
                    Some(hit) => {
                        let diffuse = scene
                            .lights
                            .first()
                            .map_or(0.0, |light| calculate_lighting(&hit, light, scene));
                        let brightness = (AMBIENT + diffuse).min(1.0);
                        Color::from_brightness(brightness, &hit.material)
                    }
                    None => Color::default(),
                };

                intersection_time += start.elapsed().as_secs_f64() * 1000.0;
                self.frame_buffer[index] = color.r;
                self.frame_buffer[index + 1] = color.g;
                self.frame_buffer[index + 2] = color.b;
                self.frame_buffer[index + 3] = 255;
            }
        }

        println!("Accumalted intersections time: {:.2}", intersection_time);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.frame_buffer.resize((width * height * 4) as usize, 0);
    }
}
