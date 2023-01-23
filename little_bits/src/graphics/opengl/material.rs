use crate::graphics::opengl::*;
use crate::maths::*;
use crate::resources::Material;

pub struct GLMaterial {
    pub base_color_factor: Float4,
    pub base_color_texture: Option<GLTexture2D>,

    pub normal_scale: f32,
    pub normal_texture: Option<GLTexture2D>,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<GLTexture2D>,

    pub occlusion_strength: f32,
    pub occlusion_texture: Option<GLTexture2D>,

    pub emissive_factor: Float3,
    pub emissive_texture: Option<GLTexture2D>
}

impl GLMaterial {
    pub fn new(material: &Material) -> Self {
        GLMaterial {
            base_color_factor: material.base_color_factor,
            base_color_texture: GLTexture2D::new(&material.base_color_texture),
            normal_scale: material.normal_scale,
            normal_texture: GLTexture2D::new(&material.normal_texture),
            metallic_factor: material.metallic_factor,
            roughness_factor: material.roughness_factor,
            metallic_roughness_texture: GLTexture2D::new(&material.metallic_roughness_texture),
            occlusion_strength: material.occlusion_strength,
            occlusion_texture: GLTexture2D::new(&material.occlusion_texture),
            emissive_factor: material.emissive_factor,
            emissive_texture: GLTexture2D::new(&material.emissive_texture)
        }
    }

    pub fn bind(&self, shader_program: &mut GLShaderProgram) {
        if let Some(base_color_texture) = &self.occlusion_texture {
            base_color_texture.bind(0);
            shader_program.set_sampler_slot(&String::from("baseColorMap"), 0);
        }
        if let Some(normal_texture) = &self.normal_texture {
            normal_texture.bind(1);
            shader_program.set_sampler_slot(&String::from("normalMap"), 1);
        }
        if let Some(mr_texture) = &self.metallic_roughness_texture {
            mr_texture.bind(2);
            shader_program.set_sampler_slot(&String::from("metallicRoughnessMap"), 2);
        }
        if let Some(occlusion_texture) = &self.occlusion_texture {
            occlusion_texture.bind(3);
            shader_program.set_sampler_slot(&String::from("occlusionMap"), 3);
        }
        if let Some(emissive_texture) = &self.emissive_texture {
            emissive_texture.bind(4);
            shader_program.set_sampler_slot(&String::from("emissiveMap"), 4);
        }
    }
}