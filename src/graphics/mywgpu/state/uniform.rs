use super::camera::Camera;

#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone)] // This is so we can store this in a buffer
pub(crate) struct Uniforms {
    view_proj: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub(crate) fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
 

 