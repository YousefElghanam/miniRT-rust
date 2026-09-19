use crate::elements::{Hittable, Light};
use crate::maths::{BvhNode, Hit, Ray};

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
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit = match &self.bvh {
            Some(bvh) => bvh.intersect(ray),
            None => None,
        };

        for object in &self.unbounded_objects {
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
