mod draw_model;

use anyhow::*;
pub use draw_model::DrawModel;
use std::path::Path;
use wgpu::util::DeviceExt;

use super::texture;
pub trait Vertex: Copy + bytemuck::Pod + bytemuck::Zeroable {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}
unsafe impl bytemuck::Pod for ModelVertex {}
unsafe impl bytemuck::Zeroable for ModelVertex {}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Model {
    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<Self> {
        let (obj_models, obj_materials) = tobj::load_obj(path.as_ref(), true)?;

        // We're assuming that the texture files are stored with the obj file
        let containing_folder = path.as_ref().parent().context("Directory has no parent")?;

        let materials = obj_materials
            .into_iter()
            .map(|material| parse_material(device, queue, layout, containing_folder, material))
            .collect::<Result<Vec<_>>>()?;

        let meshes = obj_models
            .into_iter()
            .map(|model| parse_mesh(device, path.as_ref(), model.mesh, model.name))
            .collect::<Vec<_>>();

        Ok(Self { meshes, materials })
    }
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        // TODO: maybe find a way to make this work
        // let attrs = wgpu::vertex_attr_array![
        //     0 => Float3,
        //     1 => Float2,
        //     2 => Float3
        // ];
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

fn parse_material(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    containing_folder: &Path,
    material: tobj::Material,
) -> Result<Material> {
    let diffuse_path = material.diffuse_texture;
    let diffuse_texture =
        texture::Texture::load(device, queue, containing_folder.join(diffuse_path))?;

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
            },
        ],
        label: None,
    });
    Ok(Material {
        name: material.name,
        diffuse_texture,
        bind_group,
    })
}

fn parse_mesh(device: &wgpu::Device, path: &Path, mesh: tobj::Mesh, name: String) -> Mesh {
    let mut vertices = Vec::new();
    for i in 0..mesh.positions.len() / 3 {
        vertices.push(ModelVertex {
            position: [
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            ],
            tex_coords: [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]],
            normal: [
                mesh.normals[i * 3],
                mesh.normals[i * 3 + 1],
                mesh.normals[i * 3 + 2],
            ],
        });
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Vertex Buffer", path)),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsage::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&format!("{:?} Index Buffer", path)),
        contents: bytemuck::cast_slice(&mesh.indices),
        usage: wgpu::BufferUsage::INDEX,
    });

    Mesh {
        name,
        vertex_buffer,
        index_buffer,
        num_elements: mesh.indices.len() as u32,
        material: mesh.material_id.unwrap_or(0),
    }
}
