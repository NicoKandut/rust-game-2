use linalg::{Matrix4, Vector3};
use std::f32;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector3,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::new(-4., -4., 0.),
            pitch: 0.,
            yaw: 0.,
        }
    }
}

impl Camera {
    const PI_HALF: f32 = f32::consts::PI / 2.;
    const DEG_TO_RAD_FACTOR: f32 = f32::consts::PI / 180.0;
    const MIN_PITCH: f32 = -Self::PI_HALF + 0.0001;
    const MAX_PITCH: f32 = Self::PI_HALF - 0.0001;

    pub fn up(&self) -> Vector3 {
        Vector3::new(0., 0., 1.)
    }

    pub fn forward(&self) -> Vector3 {
        Vector3::new(self.yaw.cos(), self.yaw.sin(), 0.)
    }

    pub fn right(&self) -> Vector3 {
        Vector3::new(
            (self.yaw - Self::PI_HALF).cos(),
            (self.yaw - Self::PI_HALF).sin(),
            0.,
        )
    }

    pub fn direction(&self) -> Vector3 {
        Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        )
    }

    pub(crate) fn look_at(&self) -> Matrix4 {
        Matrix4::look_at(
            self.position,
            self.position + self.direction(),
            self.up(),
        )
    }

    pub fn add_pitch(&mut self, p: f32) {
        self.pitch += p * Self::DEG_TO_RAD_FACTOR;
        self.pitch = self.pitch.clamp(Self::MIN_PITCH, Self::MAX_PITCH);
    }

    pub fn add_yaw(&mut self, y: f32) {
        self.yaw += y * Self::DEG_TO_RAD_FACTOR;
        self.yaw %= 2. * f32::consts::PI;
    }
}