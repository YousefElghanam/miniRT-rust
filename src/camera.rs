use crate::input::Input;
use crate::scene::Vec3;

pub struct Camera {
    sensetivity: f32,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn update(&mut self, input: &Input) {
        self.yaw += input.mouse_delta_x as f32 * self.sensetivity;
        self.pitch -= input.mouse_delta_y as f32 * self.sensetivity;
        self.yaw = self.yaw.clamp(-1.57, 1.57);
        self.pitch = self.pitch.clamp(-1.57, 1.57);
        println!("camera: yaw({}) == pitch({})", self.yaw, self.pitch);
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
        }
    }
}
