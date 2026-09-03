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
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Object {
    pub obj_type: ObjType,
    pub scale: u8,
    pub position: Point,
    pub direction: Vec3,
}

pub struct Scene {
    pub spot_lights: Vec<Object>,
    pub spheres: Vec<Object>,
    pub planes: Vec<Object>,
    pub cylinders: Vec<Object>,
    pub cones: Vec<Object>,
}

impl Default for Scene {
    fn default() -> Self {
        unimplemented!("implement default scene");
    }
}
