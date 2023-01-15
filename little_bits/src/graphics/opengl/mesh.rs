#[path = "helpers.rs"] mod helpers;
use helpers::*;

use std::mem;
use memoffset::offset_of;

use crate::resources::Mesh;

pub struct GLMesh {
    vao: Option<GLVAO>,
    vbo: Option<GLVBO>,
    ebo: Option<GLEBO>,

    index_count: usize
}

#[repr(C)]
pub struct GLVertex {
    position: Float3,
    normal: Float3,
    tex_coord: Float2
}

impl GLMesh {
    pub fn new(mesh: &Mesh) -> Self {
        let vao = GLVAO::new();
        let vbo = GLVBO::new();
        let ebo = GLEBO::new();

        let mut vertices: Vec<GLVertex> = Vec::with_capacity(mesh.positions.len());
        for i in 0..mesh.positions.len() {
            vertices.push(GLVertex {
                position: mesh.positions[i],
                normal: mesh.normals[i],
                tex_coord: mesh.tex_coords[i]
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

            gl_enable_vertex_attrib_array(0);
            gl_enable_vertex_attrib_array(1);
            gl_enable_vertex_attrib_array(2);

            ebo.bind();
            ebo.set_data(mem::size_of::<u32>() * mesh.indices.len(), indices.as_mut_ptr() as *mut c_void);
        } vao.unbind();

        GLMesh {
            vao: Some(vao),
            vbo: Some(vbo),
            ebo: Some(ebo),
            index_count: indices.len()
        }
    }

    fn vao(&self) -> &GLVAO {
        self.vao.as_ref().unwrap()
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