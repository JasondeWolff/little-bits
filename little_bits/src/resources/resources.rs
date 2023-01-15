use assimp;
extern crate stb_image;

use std::rc::Rc;
use std::fs;
use std::ffi::CString;
use std::collections::HashMap;

use crate::system::*;
use crate::maths::*;

#[path = "resource_manager.rs"] pub mod resource_manager;
pub use resource_manager::*;

#[path = "model.rs"] pub mod model;
pub use model::*;

#[path = "image.rs"] pub mod image;
pub use image::*;

pub struct Resources {
    model_manager: ResourceManager<Model>,
    text_manager: ResourceManager<String>,
    image_manager: ResourceManager<Image>,

    pub kill_time: f32
}

impl System for Resources {
    fn init() -> Box<Resources> {
        Box::new(Resources {
            model_manager: ResourceManager::new(5.0),
            text_manager: ResourceManager::new(5.0),
            image_manager: ResourceManager::new(5.0),
            kill_time: 5.0
        })
    }

    fn update(&mut self) {
        self.model_manager.update();
        self.text_manager.update();
        self.image_manager.update();
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
        match self.model_manager.get(&asset_path) {
            Some(resource) => resource,
            None => {
                let mut importer = assimp::Importer::new();
                importer.join_identical_vertices(true);
                importer.triangulate(true);
                importer.generate_normals(|mut args| {
                    args.enable = true;
                    args.smooth = true;
                });
                importer.calc_tangent_space(|mut args| {
                    args.enable = true;
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

                let resource = Rc::new(Model {
                    meshes: meshes,
                    materials: Vec::new()
                });

                self.model_manager.insert(resource.clone(), asset_path);
                resource
            }
        }
    }

    pub fn get_text(&mut self, asset_path: String) -> Rc<String> {
        match self.text_manager.get(&asset_path) {
            Some(resource) => resource,
            None => {
                let contents = fs::read_to_string(asset_path.clone()).expect("Failed to read text file.");
                let resource = Rc::new(contents);

                self.text_manager.insert(resource.clone(), asset_path);
                resource
            }
        }
    }

    pub fn get_image(&mut self, asset_path: String) -> Rc<Image> {
        match self.image_manager.get(&asset_path) {
            Some(resource) => resource,
            None => {
                let c_asset_path = CString::new(asset_path.as_bytes()).unwrap();

                unsafe {
                    stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
            
                    let mut width = 0;
                    let mut height = 0;
                    let mut channels = 0;
                    let data = stb_image::stb_image::bindgen::stbi_load(
                c_asset_path.as_ptr(),
                        &mut width,
                        &mut height,
                        &mut channels,
                        0,
                    );

                    let resource = Rc::new(Image {
                        data: data,
                        dimensions: Int2::new(width, height),
                        channel_count: channels
                    });

                    self.image_manager.insert(resource.clone(), asset_path);
                    resource
                }
            }
        }
    }
}