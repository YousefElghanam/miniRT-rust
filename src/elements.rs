use crate::maths::{Aabb, Color, Hit, Ray, Vec3};

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

#[derive(Debug)]
pub struct Object {
    pub obj_type: ObjType,
    pub scale: u8,
    pub position: Point,
    pub direction: Vec3,
}

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    // pub intensity: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
}

pub trait Positioned: std::fmt::Debug {
    fn position(&self) -> Vec3;
    fn set_position(&mut self, position: Vec3);
}

pub trait Hittable: std::fmt::Debug + Positioned {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn bounding_box(&self) -> Option<Aabb>;
}

#[derive(Debug)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denominator = ray.direction.dot(self.normal);

        if denominator.abs() < 0.0001 {
            return None;
        }

        let t = self.point.sub(ray.origin).dot(self.normal) / denominator;

        if t <= 0.0 {
            return None;
        }

        let point = ray.at(t);

        Some(Hit {
            t,
            point,
            normal: self.normal,
            material: self.material,
        })
    }
    fn bounding_box(&self) -> Option<Aabb> {
        return None;
    }
}

impl Positioned for Plane {
    fn position(&self) -> Vec3 {
        self.point
    }
    fn set_position(&mut self, position: Vec3) {
        self.point = position;
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin.sub(self.center);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        let t = if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return None;
        };

        let point = ray.at(t);
        let normal = point.sub(self.center).normalize();

        Some(Hit {
            t,
            point,
            normal,
            material: self.material,
        })
    }
    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3 {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
                z: self.center.z - self.radius,
            },
            max: Vec3 {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
                z: self.center.z + self.radius,
            },
        })
    }
}

impl Positioned for Sphere {
    fn position(&self) -> Vec3 {
        self.center
    }
    fn set_position(&mut self, position: Vec3) {
        self.center = position;
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub material: Material,
}

impl Triangle {
    pub fn bounding_box(&self) -> Aabb {
        let min = Vec3 {
            x: self.vertices[0]
                .x
                .min(self.vertices[1].x)
                .min(self.vertices[2].x),
            y: self.vertices[0]
                .y
                .min(self.vertices[1].y)
                .min(self.vertices[2].y),
            z: self.vertices[0]
                .z
                .min(self.vertices[1].z)
                .min(self.vertices[2].z),
        };
        let max = Vec3 {
            x: self.vertices[0]
                .x
                .max(self.vertices[1].x)
                .max(self.vertices[2].x),
            y: self.vertices[0]
                .y
                .max(self.vertices[1].y)
                .max(self.vertices[2].y),
            z: self.vertices[0]
                .z
                .max(self.vertices[1].z)
                .max(self.vertices[2].z),
        };
        Aabb { min, max }
    }
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let edge1 = self.vertices[1].sub(self.vertices[0]);
        let edge2 = self.vertices[2].sub(self.vertices[0]);
        let pvec = ray.direction.cross(edge2);
        let determinant = edge1.dot(pvec);

        if determinant.abs() < 1e-6 {
            return None;
        }

        let inverse_determinant = 1.0 / determinant;
        let tvec = ray.origin.sub(self.vertices[0]);
        let u = tvec.dot(pvec) * inverse_determinant;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let qvec = tvec.cross(edge1);
        let v = ray.direction.dot(qvec) * inverse_determinant;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(qvec) * inverse_determinant;
        if t <= 0.0 {
            return None;
        }

        let normal = edge1.cross(edge2).normalize();
        Some(Hit {
            t,
            point: ray.at(t),
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box())
    }
}

impl Positioned for Triangle {
    fn position(&self) -> Vec3 {
        self.vertices[0]
            .add(self.vertices[1])
            .add(self.vertices[2])
            .scale(1.0 / 3.0)
    }

    fn set_position(&mut self, position: Vec3) {
        let delta = position.sub(self.position());
        for vertex in &mut self.vertices {
            *vertex = vertex.add(delta);
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub bounds: Aabb,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Option<Self> {
        let mut bounds = triangles.first()?.bounding_box();
        for triangle in &triangles[1..] {
            bounds = Aabb::surrounding(&bounds, &triangle.bounding_box());
        }
        Some(Self { triangles, bounds })
    }
}

impl Hittable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_hit = None;
        for triangle in &self.triangles {
            if let Some(hit) = triangle.intersect(ray) {
                if closest_hit
                    .as_ref()
                    .map_or(true, |current: &Hit| hit.t < current.t)
                {
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounds)
    }
}

impl Positioned for Mesh {
    fn position(&self) -> Vec3 {
        self.bounds.min.add(self.bounds.max).scale(0.5)
    }

    fn set_position(&mut self, position: Vec3) {
        let delta = position.sub(self.position());
        for triangle in &mut self.triangles {
            for vertex in &mut triangle.vertices {
                *vertex = vertex.add(delta);
            }
        }
        self.bounds.min = self.bounds.min.add(delta);
        self.bounds.max = self.bounds.max.add(delta);
    }
}
