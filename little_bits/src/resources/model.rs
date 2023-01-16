use std::rc::Rc;

use crate::maths::*;

use crate::resources::Image;

pub struct Material {
    pub diffuse_map: Option<Rc<Image>>
}

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

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub min: Float3,
    pub max: Float3,

    pub material_idx: usize
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}