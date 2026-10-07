#[cfg(feature = "timing")]
use std::time::Instant;

use crate::elements::{Hittable, Material};

#[derive(Debug, Default, Clone, Copy)]
pub struct TraversalStats {
    pub rays: usize,
    pub bvh_nodes_tested: usize,
    pub aabb_tests: usize,
    pub aabb_candidates_eliminated: usize,
    pub bvh_candidates_eliminated: usize,
    pub primitive_tests: usize,
    #[cfg(feature = "timing")]
    pub aabb_time_ns: u128,
    #[cfg(feature = "timing")]
    pub bvh_time_ns: u128,
    #[cfg(feature = "timing")]
    pub bvh_primitive_time_ns: u128,
    #[cfg(feature = "timing")]
    pub primitive_time_ns: u128,
}

pub fn build_bvh(objects: &[Box<dyn Hittable>], mut indices: Vec<usize>) -> BvhNode {
    if indices.len() == 1 {
        let index = indices[0];
        let bbox = objects[index].bounding_box().unwrap();

        return BvhNode::Leaf {
            bbox,
            index,
            object_count: 1,
        };
    };
    let boxes: Vec<Aabb> = indices
        .iter()
        .map(|&index| objects[index].bounding_box().unwrap())
        .collect();

    let bbox = Aabb::surrounding_all(&boxes);
    let extent = bbox.extend();
    let axis = if extent.x > extent.y && extent.x > extent.z {
        0
    } else if extent.y > extent.z {
        1
    } else {
        2
    };
    indices.sort_by(|&a, &b| {
        axis_value(&*objects[a], axis)
            .partial_cmp(&axis_value(&*objects[b], axis))
            .unwrap()
    });
    let mid = indices.len() / 2;
    let right_indices = indices.split_off(mid);
    let left = build_bvh(objects, indices);
    let right = build_bvh(objects, right_indices);
    let object_count = left.object_count() + right.object_count();
    BvhNode::Node {
        bbox,
        left: Box::new(left),
        right: Box::new(right),
        object_count,
    }
}

#[derive(Debug)]
pub enum BvhNode {
    Leaf {
        bbox: Aabb,
        index: usize,
        object_count: usize,
    },
    Node {
        bbox: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        object_count: usize,
    },
}

impl BvhNode {
    pub fn object_count(&self) -> usize {
        match self {
            BvhNode::Leaf { object_count, .. } | BvhNode::Node { object_count, .. } => {
                *object_count
            }
        }
    }

    pub(crate) fn intersect_with_stats(
        &self,
        objects: &[Box<dyn Hittable>],
        ray: &Ray,
        max_t: f32,
        entry: f32,
        stats: &mut TraversalStats,
    ) -> Option<Hit> {
        stats.bvh_nodes_tested += 1;
        if entry >= max_t {
            stats.bvh_candidates_eliminated += self.object_count();
            return None;
        }

        match self {
            BvhNode::Leaf { index, .. } => {
                stats.primitive_tests += 1;
                #[cfg(feature = "timing")]
                let start = Instant::now();
                let hit = objects[*index].intersect(ray);
                #[cfg(feature = "timing")]
                {
                    stats.bvh_primitive_time_ns += start.elapsed().as_nanos();
                }

                match hit {
                    Some(hit) if hit.t < max_t => Some(hit),
                    _ => None,
                }
            }

            BvhNode::Node { left, right, .. } => {
                let left_distance = self.child_entry(left, ray, stats);
                let right_distance = self.child_entry(right, ray, stats);

                match (left_distance, right_distance) {
                    (Some(left_t), Some(right_t)) => {
                        if left_t < right_t {
                            let left_hit =
                                left.intersect_with_stats(objects, ray, max_t, left_t, stats);

                            let new_max_t = match &left_hit {
                                Some(hit) => hit.t,
                                None => max_t,
                            };
                            let right_hit = if right_t < new_max_t {
                                right.intersect_with_stats(objects, ray, new_max_t, right_t, stats)
                            } else {
                                stats.bvh_candidates_eliminated += right.object_count();
                                None
                            };
                            match (left_hit, right_hit) {
                                (Some(left), Some(right)) => {
                                    if left.t < right.t {
                                        Some(left)
                                    } else {
                                        Some(right)
                                    }
                                }
                                (Some(hit), None) => Some(hit),
                                (None, Some(hit)) => Some(hit),
                                (None, None) => None,
                            }
                        } else {
                            let right_hit =
                                right.intersect_with_stats(objects, ray, max_t, right_t, stats);

                            let new_max_t = match &right_hit {
                                Some(hit) => hit.t,
                                None => max_t,
                            };
                            let left_hit = if left_t < new_max_t {
                                left.intersect_with_stats(objects, ray, new_max_t, left_t, stats)
                            } else {
                                stats.bvh_candidates_eliminated += left.object_count();
                                None
                            };

                            match (left_hit, right_hit) {
                                (Some(left), Some(right)) => {
                                    if left.t < right.t {
                                        Some(left)
                                    } else {
                                        Some(right)
                                    }
                                }
                                (Some(hit), None) => Some(hit),
                                (None, Some(hit)) => Some(hit),
                                (None, None) => None,
                            }
                        }
                    }

                    (Some(left_t), None) => {
                        left.intersect_with_stats(objects, ray, max_t, left_t, stats)
                    }

                    (None, Some(right_t)) => {
                        right.intersect_with_stats(objects, ray, max_t, right_t, stats)
                    }

                    (None, None) => None,
                }
            }
        }
    }

    fn child_entry(&self, child: &BvhNode, ray: &Ray, stats: &mut TraversalStats) -> Option<f32> {
        stats.aabb_tests += 1;
        #[cfg(feature = "timing")]
        let start = Instant::now();
        let entry = child.bounding_box().hit_distance(ray);
        #[cfg(feature = "timing")]
        {
            stats.aabb_time_ns += start.elapsed().as_nanos();
        }
        match entry {
            Some(entry) => Some(entry),
            None => {
                stats.aabb_candidates_eliminated += child.object_count();
                None
            }
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        match self {
            BvhNode::Leaf { bbox, .. } => *bbox,
            BvhNode::Node { bbox, .. } => *bbox,
        }
    }
}

fn slab_intersection(
    origin: f32,
    direction: f32,
    inv_direction: f32,
    min: f32,
    max: f32,
) -> Option<(f32, f32)> {
    if direction == 0.0 {
        if origin < min || origin > max {
            return None;
        }
        return Some((f32::NEG_INFINITY, f32::INFINITY));
    }
    let t1 = (min - origin) * inv_direction;
    let t2 = (max - origin) * inv_direction;
    return Some((t1.min(t2), t1.max(t2)));
}

fn axis_value(object: &dyn Hittable, axis: usize) -> f32 {
    let bbox = object.bounding_box().unwrap();
    let center = bbox.min.add(bbox.max).scale(0.5);
    if axis == 0 {
        center.x
    } else if axis == 1 {
        center.y
    } else {
        center.z
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit_distance(&self, ray: &Ray) -> Option<f32> {
        let (entry_x, exit_x) = match slab_intersection(
            ray.origin.x,
            ray.direction.x,
            ray.inv_direction.x,
            self.min.x,
            self.max.x,
        ) {
            Some(interval) => interval,
            None => return None,
        };
        let (entry_y, exit_y) = match slab_intersection(
            ray.origin.y,
            ray.direction.y,
            ray.inv_direction.y,
            self.min.y,
            self.max.y,
        ) {
            Some(interval) => interval,
            None => return None,
        };
        let (entry_z, exit_z) = match slab_intersection(
            ray.origin.z,
            ray.direction.z,
            ray.inv_direction.z,
            self.min.z,
            self.max.z,
        ) {
            Some(interval) => interval,
            None => return None,
        };
        let entry = entry_x.max(entry_y).max(entry_z);
        let exit = exit_x.min(exit_y).min(exit_z);
        if exit >= 0.0 && entry <= exit {
            return Some(entry);
        } else {
            return None;
        }
    }
    pub(crate) fn surrounding(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            min: Vec3 {
                x: a.min.x.min(b.min.x),
                y: a.min.y.min(b.min.y),
                z: a.min.z.min(b.min.z),
            },
            max: Vec3 {
                x: a.max.x.max(b.max.x),
                y: a.max.y.max(b.max.y),
                z: a.max.z.max(b.max.z),
            },
        }
    }
    fn extend(&self) -> Vec3 {
        self.max.sub(self.min)
    }
    fn surrounding_all(boxes: &[Aabb]) -> Aabb {
        let mut result = boxes[0];
        for bbox in &boxes[1..] {
            result = Aabb::surrounding(&result, bbox);
        }
        result
    }
    fn hit(&self, ray: &Ray) -> bool {
        self.hit_distance(ray).is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Color {
    pub fn from_brightness(brightness: f32, material: &Material) -> Color {
        Color {
            r: (brightness * material.color.r as f32) as u8,
            g: (brightness * material.color.g as f32) as u8,
            b: (brightness * material.color.b as f32) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    pub fn scale(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Vec3 {
        let length = self.length();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec3_eq(actual: Vec3, expected: Vec3) {
        let epsilon = 1e-5;
        assert!(
            (actual.x - expected.x).abs() < epsilon
                && (actual.y - expected.y).abs() < epsilon
                && (actual.z - expected.z).abs() < epsilon,
            "expected ({}, {}, {}), got ({}, {}, {})",
            expected.x,
            expected.y,
            expected.z,
            actual.x,
            actual.y,
            actual.z
        );
    }

    #[test]
    fn cross_product_of_unit_axes() {
        let i = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let j = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let k = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        assert_vec3_eq(i.cross(j), k);
        assert_vec3_eq(j.cross(k), i);
        assert_vec3_eq(k.cross(i), j);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Vec3 {
        self.origin.add(self.direction.scale(t))
    }
}

pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}
