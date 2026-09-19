#[cfg(feature = "timing")]
use std::time::Instant;

use crate::camera::Camera;
use crate::constants::AMBIENT;
use crate::elements::Light;
use crate::maths::{Color, Hit, Ray, TraversalStats, Vec3};
use crate::scene::Scene;

pub fn calculate_lighting(
    hit: &Hit,
    light: &Light,
    scene: &Scene,
    stats: &mut TraversalStats,
) -> f32 {
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

    if let Some(shadow_hit) = scene.intersect_with_stats(&shadow_ray, stats) {
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
    pub traversal_stats: TraversalStats,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            frame_buffer: vec![0; (width * height * 4) as usize],
            traversal_stats: TraversalStats::default(),
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
        self.traversal_stats = TraversalStats::default();
        #[cfg(feature = "timing")]
        let mut intersection_time = 0.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;
                let ray = self.ray_for_pixel(x, y, camera);
                #[cfg(feature = "timing")]
                let start = Instant::now();

                let color = match scene.intersect_with_stats(&ray, &mut self.traversal_stats) {
                    Some(hit) => {
                        let diffuse = scene.lights.first().map_or(0.0, |light| {
                            calculate_lighting(&hit, light, scene, &mut self.traversal_stats)
                        });
                        let brightness = (AMBIENT + diffuse).min(1.0);
                        Color::from_brightness(brightness, &hit.material)
                    }
                    None => Color::default(),
                };

                #[cfg(feature = "timing")]
                {
                    intersection_time += start.elapsed().as_secs_f64() * 1000.0;
                }
                self.frame_buffer[index] = color.r;
                self.frame_buffer[index + 1] = color.g;
                self.frame_buffer[index + 2] = color.b;
                self.frame_buffer[index + 3] = 255;
            }
        }

        #[cfg(feature = "timing")]
        println!("Accumalted intersections time: {:.2}", intersection_time);
        #[cfg(feature = "timing")]
        let bvh_other_time_ns = self
            .traversal_stats
            .bvh_time_ns
            .saturating_sub(self.traversal_stats.aabb_time_ns)
            .saturating_sub(self.traversal_stats.bvh_primitive_time_ns);
        let total_candidates = self.traversal_stats.rays * scene.object_count();
        let aabb_percentage = percentage(
            self.traversal_stats.aabb_candidates_eliminated,
            total_candidates,
        );
        let bvh_percentage = percentage(
            self.traversal_stats.bvh_candidates_eliminated,
            total_candidates,
        );
        println!(
            "Traversal: AABB eliminated {} ({:.1}%), BVH eliminated {} ({:.1}%), primitive tests {} / {} candidates (AABB tests {}, BVH nodes {}, rays {})",
            self.traversal_stats.aabb_candidates_eliminated,
            aabb_percentage,
            self.traversal_stats.bvh_candidates_eliminated,
            bvh_percentage,
            self.traversal_stats.primitive_tests,
            total_candidates,
            self.traversal_stats.aabb_tests,
            self.traversal_stats.bvh_nodes_tested,
            self.traversal_stats.rays,
        );
        #[cfg(feature = "timing")]
        println!(
            "AABB intersection time: {:.3} ms",
            nanos_to_millis(self.traversal_stats.aabb_time_ns)
        );
        #[cfg(feature = "timing")]
        println!(
            "BVH recursion/other:     {:.3} ms",
            nanos_to_millis(bvh_other_time_ns)
        );
        #[cfg(feature = "timing")]
        println!(
            "Primitive intersections: {:.3} ms",
            nanos_to_millis(
                self.traversal_stats.bvh_primitive_time_ns + self.traversal_stats.primitive_time_ns
            )
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.frame_buffer.resize((width * height * 4) as usize, 0);
    }
}

#[cfg(feature = "timing")]
fn nanos_to_millis(nanoseconds: u128) -> f64 {
    nanoseconds as f64 / 1_000_000.0
}

fn percentage(value: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        value as f32 * 100.0 / total as f32
    }
}
