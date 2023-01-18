extern crate gltf;
extern crate stb_image;

use std::rc::Rc;
use std::fs;
use std::ffi::CString;

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
    fn process_node(node: &gltf::Node, buffers: &Vec<gltf::buffer::Data>, images: &Vec<gltf::image::Data>, meshes: &mut Vec<Mesh>, materials: &mut Vec<Material>) {
        let (translation, rotation, scale) = node.transform().decomposed();
        let translation = Float3::new(translation[0], translation[1], translation[2]);
        let rotation = Quaternion::new(rotation[3], rotation[0], rotation[1], rotation[2]); // Correct order?!?!?!?
        let scale = Float3::new(scale[0], scale[1], scale[2]);

        match node.mesh() {
            Some(mesh) => {
                for primitive in mesh.primitives() {
                    if primitive.mode() == gltf::mesh::Mode::Triangles {
                        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                        let bounds = primitive.bounding_box();
                        let min = Float3::from(&bounds.min);
                        let max = Float3::from(&bounds.max);

                        let positions = {
                            let iter = reader
                                .read_positions()
                                .expect("Failed to process mesh node. (Vertices must have positions)");

                            iter.map(|arr| -> Float3 { Float3::from(&arr) }).collect::<Vec<_>>()
                        };

                        let mut vertices: Vec<Vertex> = positions
                            .into_iter()
                            .map(|position| {
                                Vertex {
                                    position: Vector3::from(position),
                                    ..Vertex::default()
                                }
                        }).collect();

                        if let Some(normals) = reader.read_normals() {
                            for (i, normal) in normals.enumerate() {
                                vertices[i].normal = Float3::from(&normal);
                            }
                        }

                        if let Some(tangents) = reader.read_tangents() {
                            for (i, tangent) in tangents.enumerate() {
                                vertices[i].tangent = Float4::from(&tangent);
                            }
                        }

                        let mut tex_coord_channel = 0;
                        while let Some(tex_coords) = reader.read_tex_coords(tex_coord_channel) {
                            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                                match tex_coord_channel {
                                    0 => vertices[i].tex_coord = Float2::from(&tex_coord),
                                    1 => vertices[i].tex_coord_1 = Float2::from(&tex_coord),
                                    _ => {}
                                }
                            }

                            tex_coord_channel += 1;
                        }

                        if let Some(colors) = reader.read_colors(0) {
                            let colors = colors.into_rgba_f32();
                            for (i, color) in colors.enumerate() {
                                vertices[i].color = Float4::from(&color);
                            }
                        }

                        let indices = reader
                            .read_indices()
                            .map(|read_indices| {
                                read_indices.into_u32().collect::<Vec<_>>()
                            }).expect("Failed to process mesh node. (Indices are required)");
                        
                        meshes.push(Mesh {
                            vertices: vertices,
                            indices: indices,
                            min: min,
                            max: max,
                            material_idx: 0
                        });
                    } else {
                        panic!("Failed to process mesh node. (Trying to parse a non-triangle)");
                    }
                }
            },
            None => {}
        };
    }

    pub fn get_model(&mut self, asset_path: String) -> Rc<Model> {
        match self.model_manager.get(&asset_path) {
            Some(resource) => resource,
            None => {
                let mut meshes = Vec::new();
                let mut materials = Vec::new();

                let (document, buffers, images) = gltf::import(asset_path.clone()).expect("Failed to get model.");
                if document.nodes().len() > 0 {
                    Self::process_node(document.nodes().next().as_ref().unwrap(), &buffers, &images, &mut meshes, &mut materials);
                }

                let resource = Rc::new(Model {
                    meshes: meshes,
                    materials: materials
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

                    assert!(!data.is_null(), "Failed to read image.");

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