use crate::maths::{Color, Hit, Ray, Vec3};

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
}

impl Positioned for Sphere {
    fn position(&self) -> Vec3 {
        self.center
    }
    fn set_position(&mut self, position: Vec3) {
        self.center = position;
    }
}
