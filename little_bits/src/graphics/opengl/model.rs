#[path = "helpers.rs"] mod helpers;
use helpers::*;

use std::mem;
use memoffset::offset_of;

use crate::resources::Mesh;
use crate::graphics::opengl::material::GLMaterial;

pub struct GLModel {
    pub meshes: Vec<GLMesh>,
    pub materials: Vec<GLMaterial>,
}

impl GLModel {
    pub fn new(meshes: Vec<GLMesh>, materials: Vec<GLMaterial>) -> Self {
        GLModel {
            meshes: meshes,
            materials: materials
        }
    }

    pub fn bounds(&self) -> (Float3, Float3) {
        let mut min = self.meshes[0].min;
        let mut max = self.meshes[0].max;

        for mesh in &self.meshes {
            min = min.min(mesh.min);
            max = max.max(mesh.max);
        }

        (min, max)
    }
}

pub struct GLMesh {
    vao: Option<GLVAO>,
    vbo: Option<GLVBO>,
    ebo: Option<GLEBO>,

    index_count: usize,
    material_idx: usize,

    min: Float3,
    max: Float3
}

#[repr(C)]
pub struct GLVertex {
    position: Float3,
    normal: Float3,
    tex_coord: Float2,
    tangent: Float4
}

impl GLMesh {
    pub fn new(mesh: &Mesh) -> Self {
        let vao = GLVAO::new();
        let vbo = GLVBO::new();
        let ebo = GLEBO::new();

        let mut vertices: Vec<GLVertex> = Vec::with_capacity(mesh.vertices.len());
        for i in 0..mesh.vertices.len() {
            vertices.push(GLVertex {
                position: mesh.vertices[i].position,
                normal: mesh.vertices[i].normal,
                tex_coord: mesh.vertices[i].tex_coord,
                tangent: mesh.vertices[i].tangent
            });
        }
        let mut indices: Vec<u32> = mesh.indices.clone();

        vao.bind(); {
            let vertex_size = mem::size_of::<GLVertex>();

            vbo.bind();
            vbo.set_data(vertex_size * vertices.len(), vertices.as_mut_ptr() as *mut c_void);

            gl_vertex_attrib_ptr(0, 3, vertex_size, offset_of!(GLVertex, position) as *const c_void);
            gl_vertex_attrib_ptr(1, 3, vertex_size, offset_of!(GLVertex, normal) as *const c_void);
            gl_vertex_attrib_ptr(2, 2, vertex_size, offset_of!(GLVertex, tex_coord) as *const c_void);
            gl_vertex_attrib_ptr(3, 4, vertex_size, offset_of!(GLVertex, tangent) as *const c_void);

            gl_enable_vertex_attrib_array(0);
            gl_enable_vertex_attrib_array(1);
            gl_enable_vertex_attrib_array(2);
            gl_enable_vertex_attrib_array(3);

            ebo.bind();
            ebo.set_data(mem::size_of::<u32>() * mesh.indices.len(), indices.as_mut_ptr() as *mut c_void);
        } vao.unbind();

        GLMesh {
            vao: Some(vao),
            vbo: Some(vbo),
            ebo: Some(ebo),
            index_count: indices.len(),
            material_idx: mesh.material_idx,
            min: mesh.min,
            max: mesh.max
        }
    }

    fn vao(&self) -> &GLVAO {
        self.vao.as_ref().unwrap()
    }

    pub fn material_idx(&self) -> usize {
        self.material_idx
    }

    pub fn draw(&self) {
        self.vao().bind(); {
            gl_draw_elems(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT);
        } self.vao().unbind();
    }
}

impl Drop for GLMesh {
    fn drop(&mut self) {
        self.vbo = None;
        self.ebo = None;
        self.vao = None;
    }
}