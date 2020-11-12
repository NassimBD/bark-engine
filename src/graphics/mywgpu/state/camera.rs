use super::input_controller::InputController;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
pub struct Camera {
    pub(crate) eye: cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up: cgmath::Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn update(&mut self, inputs: &InputController) {
        use cgmath::InnerSpace;
        let forward = self.target - self.eye;
        let forward_normalize = forward.normalize();
        let forward_magnitude = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if inputs.is_forward_pressed && forward_magnitude > inputs.speed {
            self.eye += forward_normalize * inputs.speed;
        }
        if inputs.is_backward_pressed {
            self.eye -= forward_normalize * inputs.speed;
        }

        let right = forward_normalize.cross(self.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = self.target - self.eye;
        let forward_mag = forward.magnitude();

        if inputs.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            self.eye = self.target - (forward + right * inputs.speed).normalize() * forward_mag;
        }
        if inputs.is_left_pressed {
            self.eye = self.target - (forward - right * inputs.speed).normalize() * forward_mag;
        }
    }
}
