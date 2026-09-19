use crate::elements::{Hittable, Light};
use crate::maths::{BvhNode, Hit, Ray, TraversalStats};

pub struct Scene {
    pub bvh: Option<BvhNode>,
    pub unbounded_objects: Vec<Box<dyn Hittable>>,
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            bvh: None,
            unbounded_objects: Vec::new(),
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}

impl Scene {
    pub fn object_count(&self) -> usize {
        self.bvh.as_ref().map_or(0, BvhNode::object_count) + self.unbounded_objects.len()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut stats = TraversalStats::default();
        self.intersect_with_stats(ray, &mut stats)
    }

    pub fn intersect_with_stats(&self, ray: &Ray, stats: &mut TraversalStats) -> Option<Hit> {
        stats.rays += 1;
        let mut closest_hit = match &self.bvh {
            Some(bvh) => {
                stats.aabb_tests += 1;
                match bvh.bounding_box().unwrap().hit_distance(ray) {
                    Some(entry) => bvh.intersect_with_stats(ray, f32::INFINITY, entry, stats),
                    None => {
                        stats.aabb_candidates_eliminated += bvh.object_count();
                        None
                    }
                }
            }
            None => None,
        };

        for object in &self.unbounded_objects {
            stats.primitive_tests += 1;
            if let Some(hit) = object.intersect(ray) {
                let is_closer = match &closest_hit {
                    None => true,
                    Some(current) => hit.t < current.t,
                };
                if is_closer {
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
