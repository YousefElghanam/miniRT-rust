use crate::input::Input;
use crate::maths::Vec3;

pub struct Camera {
    sensetivity: f32,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
}

impl Camera {
    pub fn update(&mut self, input: &Input) {
        self.yaw += input.mouse_delta_x as f32 * self.sensetivity;
        self.pitch -= input.mouse_delta_y as f32 * self.sensetivity;
        self.pitch = self
            .pitch
            .clamp(-std::f32::consts::FRAC_2_PI, std::f32::consts::FRAC_2_PI);
        println!("camera: yaw({}) == pitch({})", self.yaw, self.pitch);
    }
    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let forward = Vec3 {
            x: self.yaw.sin() * self.pitch.cos(),
            y: self.pitch.sin(),
            z: -self.yaw.cos() * self.pitch.cos(),
        }
        .normalize();

        let right = Vec3 {
            x: self.yaw.cos(),
            y: 0.0,
            z: self.yaw.sin(),
        }
        .normalize();

        let up = forward.cross(right).normalize();

        (forward, right, up)
    }
    pub fn movement(&self, input: &Input, speed: f32, dt: f32) -> Vec3 {
        let forward = Vec3 {
            x: self.yaw.sin(),
            y: 0.0,
            z: -self.yaw.cos(),
        }
        .normalize();

        let right = Vec3 {
            x: self.yaw.cos(),
            y: 0.0,
            z: self.yaw.sin(),
        }
        .normalize();

        let mut movement = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        if input.key_w {
            movement = movement.add(forward);
        }
        if input.key_s {
            movement = movement.sub(forward);
        }
        if input.key_d {
            movement = movement.add(right);
        }
        if input.key_a {
            movement = movement.sub(right);
        }

        if movement.length() > 0.0 {
            movement = movement.normalize();
        }
        movement.scale(speed * dt)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            sensetivity: 0.002,
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pitch: 0.0,
            yaw: 0.0,
            fov: 90.0,
        }
    }
}
