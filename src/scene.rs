use crate::elements::{Hittable, Light};
use crate::maths::{Hit, Ray, Vec3};

#[derive(Debug)]
pub enum ObjType {
    LIGHT,
    SPHERE,
    PLANE,
    CYLINDER,
    CONE,
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// To encapsulate Point's attributes (make them private), use the following:
// impl Point {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Self { x, y, z }
//     }
//     pub fn x(&self) -> f32 {
//         self.x
//     }
//     pub fn y(&self) -> f32 {
//         self.y
//     }
//     pub fn z(&self) -> f32 {
//         self.z
//     }
// }

#[derive(Debug)]
pub struct Object {
    pub obj_type: ObjType,
    pub scale: u8,
    pub position: Point,
    pub direction: Vec3,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;

        for object in &self.objects {
            if let Some(hit) = object.intersect(ray) {
                let is_closer = match &closest_hit {
                    None => true,
                    Some(current) => hit.t < current.t,
                };
                if is_closer {
                    closest_hit = Some(hit);
                };
            }
        }
        closest_hit
    }
}
