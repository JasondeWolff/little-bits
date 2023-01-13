use crate::maths::*;

pub struct Mesh {
    pub positions: Vec<Float3>,
    pub normals: Vec<Float3>,
    pub tex_coords: Vec<Float2>,

    pub indices: Vec<u32>
}

pub struct Model {
    pub meshes: Vec<Mesh>
}