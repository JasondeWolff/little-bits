use crate::maths::*;

use crate::resources::Image;
use crate::Shared;

#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub index: Option<usize>,

    pub base_color_factor: Float4,
    pub base_color_texture: Shared<Image>,

    pub normal_scale: f32,
    pub normal_texture: Shared<Image>,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Shared<Image>,

    pub occlusion_strength: f32,
    pub occlusion_texture: Shared<Image>,

    pub emissive_factor: Float3,
    pub emissive_texture: Shared<Image>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            name: String::from("default"),
            index: None,
            base_color_factor: Float4::new(1.0, 1.0, 1.0, 1.0),
            base_color_texture: Shared::empty(),
            normal_scale: 1.0,
            normal_texture: Shared::empty(),
            metallic_factor: 0.0,
            roughness_factor: 1.0,
            metallic_roughness_texture: Shared::empty(),
            occlusion_strength: 1.0,
            occlusion_texture: Shared::empty(),
            emissive_factor: Float3::default(),
            emissive_texture: Shared::empty(),
        }
    }
}

#[derive(Clone)]
pub struct Vertex {
    pub position: Float3,
    pub normal: Float3,
    pub tangent: Float4,
    pub tex_coord: Float2,
    pub tex_coord_1: Float2,
    pub color: Float4
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Float3::default(),
            normal: Float3::default(),
            tangent: Float4::default(),
            tex_coord: Float2::default(),
            tex_coord_1: Float2::default(),
            color: Float4::default()
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub min: Float3,
    pub max: Float3,

    pub material_idx: usize
}

#[derive(Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Shared<Material>>
}