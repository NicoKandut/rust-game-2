use linalg::Vector3;

pub(crate) struct InputController {
    pub(crate) forward_pressed: bool,
    pub(crate) backward_pressed: bool,
    pub(crate) left_pressed: bool,
    pub(crate) right_pressed: bool,
    pub(crate) up_pressed: bool,
    pub(crate) down_pressed: bool,

    pub(crate) acceleration: Vector3,
    pub(crate) velocity: Vector3,
}

impl Default for InputController {
    fn default() -> Self {
        Self {
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            acceleration: Vector3::new(0.02,0.02,0.02),
            velocity: Vector3::new(0., 0., 0.),
        }
    }
}

impl InputController {
    pub fn tick(&mut self) {
        let x_factor = if self.forward_pressed { 1.0 } else if self.backward_pressed { -1.0 } else { 0.0 };
        let y_factor = if self.left_pressed { -1.0 } else if self.right_pressed { 1.0 } else { 0.0 };
        let z_factor = if self.up_pressed { -1.0 } else if self.down_pressed { 1.0 } else { 0.0 };


        *self.velocity.mut_x() = f32::clamp(self.velocity.x() + x_factor * self.acceleration.x(), -0.01, 0.01) * 0.95;
        *self.velocity.mut_y() = f32::clamp(self.velocity.y() + y_factor * self.acceleration.y(), -0.01, 0.01) * 0.95;
        *self.velocity.mut_z() = f32::clamp(self.velocity.z() + z_factor * self.acceleration.z(), -0.01, 0.01) * 0.95;
    }
}