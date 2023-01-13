use std::rc::Rc;
use assimp;
use std::fs;

use crate::system::*;
use crate::maths::*;

#[path = "model.rs"] pub mod model;
pub use model::*;

pub struct Resources {

}

impl System for Resources {
    fn init() -> Box<Resources> {
        Box::new(Resources {
            
        })
    }

    fn update(&mut self) {

    }
}

impl Resources {
    fn process_mesh(mesh: assimp::Mesh) -> Mesh {
        let mut positions: Vec<Float3> = Vec::with_capacity(mesh.num_vertices as usize);
        let mut normals: Vec<Float3> = Vec::with_capacity(mesh.num_vertices as usize);
        let mut tex_coords: Vec<Float2> = Vec::with_capacity(mesh.num_vertices as usize);
        let mut indices: Vec<u32> = Vec::with_capacity((mesh.num_faces * 3) as usize); // We always use triangles

        for position in mesh.vertex_iter() {
            positions.push(Float3::new(position.x, position.y, position.z));
        }

        for normal in mesh.normal_iter() {
            normals.push(Float3::new(normal.x, normal.y, normal.z));
        }

        for tex_coord in mesh.texture_coords_iter(0) { 
            tex_coords.push(Float2::new(tex_coord.x, tex_coord.y));
        }

        for face in mesh.face_iter() {
            let slice = unsafe { std::slice::from_raw_parts(face.indices, face.num_indices as usize) };
            for i in 0..face.num_indices {
                indices.push(slice[i as usize]);
            }
        }

        Mesh {
            positions: positions,
            normals: normals,
            tex_coords: tex_coords,
            indices: indices
        }
    }

    pub fn get_model(&mut self, asset_path: String) -> Rc<Model> {
        let mut importer = assimp::Importer::new();
        importer.join_identical_vertices(true);
        importer.triangulate(true);
        importer.generate_normals(|mut args| {
            args.enable = true;
            args.smooth = true;
        });
        importer.pre_transform_vertices(|mut args| {
            args.enable = true;
        });
        importer.gen_uv_coords(true);
        importer.optimize_meshes(true);

        let model = importer.read_file(asset_path.as_str()).expect("Failed to get model.");
        
        let mut meshes: Vec<Mesh> = Vec::with_capacity(model.num_meshes as usize);
        for mesh in 0..model.num_meshes {
            meshes.push(Self::process_mesh(model.mesh(mesh as usize).expect("Failed to get model.")));
        }

        Rc::new(Model {
            meshes: meshes
        })
    }

    pub fn get_text(&mut self, asset_path: String) -> Rc<String> {
        let contents = fs::read_to_string(asset_path).expect("Failed to read text file.");
        Rc::new(contents)
    }
}