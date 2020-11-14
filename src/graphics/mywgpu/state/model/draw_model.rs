use std::ops::Range;

use super::{Material, Mesh, Model};

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, uniforms, light);
    }
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_model(
        &mut self,
        model: &'a Model,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'a> for wgpu::RenderPass<'b>
where
    'a: 'b,
{
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, uniforms, light);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..));
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, &uniforms, &[]);
        self.set_bind_group(2, &light, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(
        &mut self,
        model: &'a Model,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        self.draw_model_instanced(model, 0..1, uniforms, light);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        uniforms: &'a wgpu::BindGroup,
        light: &'a wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone(), uniforms, light);
        }
    }
}
