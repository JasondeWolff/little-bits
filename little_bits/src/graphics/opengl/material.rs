use crate::graphics::opengl::*;
use crate::resources::Material;
use crate::Shared;

pub struct GLMaterial {
    pub base_color_texture: Option<GLTexture2D>,
    pub normal_texture: Option<GLTexture2D>,
    pub metallic_roughness_texture: Option<GLTexture2D>,
    pub occlusion_texture: Option<GLTexture2D>,
    pub emissive_texture: Option<GLTexture2D>,
    material_properties: Shared<Material>
}

impl GLMaterial {
    pub fn new(material: Shared<Material>) -> Self {
        GLMaterial {
            base_color_texture: GLTexture2D::new(&material.as_ref().base_color_texture),
            normal_texture: GLTexture2D::new(&material.as_ref().normal_texture),
            metallic_roughness_texture: GLTexture2D::new(&material.as_ref().metallic_roughness_texture),
            occlusion_texture: GLTexture2D::new(&material.as_ref().occlusion_texture),
            emissive_texture: GLTexture2D::new(&material.as_ref().emissive_texture),
            material_properties: material.clone()
        }
    }

    pub fn bind(&self, shader_program: &mut GLShaderProgram) {
        shader_program.set_float4(&String::from("material.baseColorFactor"), self.material_properties.as_ref().base_color_factor);
        if let Some(base_color_texture) = &self.base_color_texture {
            base_color_texture.bind(0);
            shader_program.set_sampler_slot(&String::from("material.baseColorMap"), 0);
            shader_program.set_bool(&String::from("material.hasBaseColorMap"), true);
        } else {
            shader_program.set_bool(&String::from("material.hasBaseColorMap"), false);
        }

        shader_program.set_float(&String::from("material.normalScale"), self.material_properties.as_ref().normal_scale);
        if let Some(normal_texture) = &self.normal_texture {
            normal_texture.bind(1);
            shader_program.set_sampler_slot(&String::from("material.normalMap"), 1);
            shader_program.set_bool(&String::from("material.hasNormalMap"), true);
        } else {
            shader_program.set_bool(&String::from("material.hasNormalMap"), false);
        }

        shader_program.set_float(&String::from("material.metallicFactor"), self.material_properties.as_ref().metallic_factor);
        shader_program.set_float(&String::from("material.roughnessFactor"), self.material_properties.as_ref().roughness_factor);
        if let Some(mr_texture) = &self.metallic_roughness_texture {
            mr_texture.bind(2);
            shader_program.set_sampler_slot(&String::from("material.metallicRoughnessMap"), 2);
            shader_program.set_bool(&String::from("material.hasMetallicRoughnessMap"), true);
        } else {
            shader_program.set_bool(&String::from("material.hasMetallicRoughnessMap"), false);
        }

        shader_program.set_float(&String::from("material.occlusionStrength"), self.material_properties.as_ref().occlusion_strength);
        if let Some(occlusion_texture) = &self.occlusion_texture {
            occlusion_texture.bind(3);
            shader_program.set_sampler_slot(&String::from("material.occlusionMap"), 3);
            shader_program.set_bool(&String::from("material.hasOcclusionMap"), true);
        } else {
            shader_program.set_bool(&String::from("material.hasOcclusionMap"), false);
        }

        shader_program.set_float3(&String::from("material.emissiveFactor"), self.material_properties.as_ref().emissive_factor);
        if let Some(emissive_texture) = &self.emissive_texture {
            emissive_texture.bind(4);
            shader_program.set_sampler_slot(&String::from("material.emissiveMap"), 4);
            shader_program.set_bool(&String::from("material.hasEmissiveMap"), true);
        } else {
            shader_program.set_bool(&String::from("material.hasEmissiveMap"), false);
        }
    }
}