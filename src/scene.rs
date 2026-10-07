use crate::elements::{Hittable, Light};
use crate::maths::{build_bvh, BvhNode, Hit, Ray, TraversalStats};
#[cfg(feature = "timing")]
use std::time::Instant;

pub struct Scene {
    pub bvh: Option<BvhNode>,
    pub unbounded_objects: Vec<usize>,
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
    pub fn rebuild_acceleration(&mut self) {
        let mut bounded_indices = Vec::new();
        let mut unbounded_indices = Vec::new();
        for (index, object) in self.objects.iter().enumerate() {
            if object.bounding_box().is_some() {
                bounded_indices.push(index);
            } else {
                unbounded_indices.push(index);
            }
        }
        self.bvh = (!bounded_indices.is_empty()).then(|| build_bvh(&self.objects, bounded_indices));
        self.unbounded_objects = unbounded_indices;
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut stats = TraversalStats::default();
        self.intersect_with_stats(ray, &mut stats)
    }

    pub fn intersect_with_stats(&self, ray: &Ray, stats: &mut TraversalStats) -> Option<Hit> {
        stats.rays += 1;
        let mut closest_hit = match &self.bvh {
            Some(bvh) => {
                #[cfg(feature = "timing")]
                let bvh_start = Instant::now();
                stats.aabb_tests += 1;
                #[cfg(feature = "timing")]
                let aabb_start = Instant::now();
                let entry = bvh.bounding_box().hit_distance(ray);
                #[cfg(feature = "timing")]
                {
                    stats.aabb_time_ns += aabb_start.elapsed().as_nanos();
                }
                let result = match entry {
                    Some(entry) => {
                        bvh.intersect_with_stats(&self.objects, ray, f32::INFINITY, entry, stats)
                    }
                    None => {
                        stats.aabb_candidates_eliminated += bvh.object_count();
                        None
                    }
                };
                #[cfg(feature = "timing")]
                {
                    stats.bvh_time_ns += bvh_start.elapsed().as_nanos();
                }
                result
            }
            None => None,
        };

        for &index in &self.unbounded_objects {
            stats.primitive_tests += 1;
            #[cfg(feature = "timing")]
            let primitive_start = Instant::now();
            if let Some(hit) = self.objects[index].intersect(ray) {
                let is_closer = match &closest_hit {
                    None => true,
                    Some(current) => hit.t < current.t,
                };
                if is_closer {
                    closest_hit = Some(hit);
                }
            }
            #[cfg(feature = "timing")]
            {
                stats.primitive_time_ns += primitive_start.elapsed().as_nanos();
            }
        }
        closest_hit
    }
}
