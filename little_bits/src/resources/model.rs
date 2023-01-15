use std::rc::Rc;

use crate::maths::*;

use crate::resources::Image;

pub struct Material {
    pub diffuse_map: Option<Rc<Image>>
}

pub struct Mesh {
    pub positions: Vec<Float3>,
    pub normals: Vec<Float3>,
    pub tex_coords: Vec<Float2>,

    pub indices: Vec<u32>
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}