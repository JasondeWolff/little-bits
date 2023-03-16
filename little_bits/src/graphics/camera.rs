use crate::gmaths::*;
use crate::app;

#[derive(Clone)]
pub struct Camera {
    translation: Float3,
    rotation: Quat,

    aspect_ratio: Option<f32>,
    fov: f32,
    near: f32,
    far: f32,

    proj_dirty: bool,
    proj_matrix: Float4x4,
    view_dirty: bool,
    view_matrix: Float4x4,
}

#[repr(C)]
pub struct CLCamera {
    position: Float4,
	lower_left_corner: Float4,
	horizontal: Float4,
	vertical: Float4
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            translation: Float3::default(),
            rotation: Quat::identity(),
            aspect_ratio: None,
            fov: 60.0,
            near: 0.1,
            far: 300.0,
            proj_dirty: true,
            proj_matrix: Float4x4::identity(),
            view_dirty: false,
            view_matrix: Float4x4::identity(),
        }
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.proj_dirty = true;
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.proj_dirty = true;
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.proj_dirty = true;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: Option<f32>) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_proj_matrix(&mut self) -> Float4x4 {
        if self.proj_dirty {
            let aspect_ratio = self.aspect_ratio.unwrap_or_else(|| -> f32 {
                let dimensions = app().graphics().dimensions();
                dimensions.x as f32 / dimensions.y as f32
            });
            
            self.proj_matrix = Float4x4::perspective(self.fov, aspect_ratio, self.near, self.far);
        }

        self.proj_matrix
    }

    pub fn get_translation(&self) -> Float3 {
        self.translation
    }

    pub fn get_rotation(&self) -> Quat {
        self.rotation
    }
    
    pub fn set_translation(&mut self, translation: Float3) {
        self.translation = translation;
        self.view_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.view_dirty = true;
    }

    pub fn translate(&mut self, translation: Float3) {
        self.set_translation(self.translation + translation);
    }

    pub fn get_view_matrix(&mut self) -> Float4x4 {
        if self.view_dirty {
            let mut rot = Float4x4::from(self.rotation);
            rot.transpose();
            // self.view_matrix = rot * Float4x4::translation(-self.translation);
            self.view_matrix = Float4x4::translation(-self.translation) * Float4x4::from(self.rotation);
            self.view_dirty = false;
        }

        self.view_matrix
    }
}

impl CLCamera {
    pub fn new(position: Float3, forward: Float3, fov: f32, aspect_ratio: f32) -> Self {
        let w = forward;
        let u = Float3::up().cross(w).normalized();
        let v = w.cross(u);

        let h = (fov.to_radians() * 0.5).tan();
        let height = 2.0 * h;
        let width = aspect_ratio * height;

        let horizontal = u * width;
        let vertical = v * height;
        let lower_left_corner = position - horizontal * 0.5 - vertical * 0.5 - w;

        CLCamera {
            position: Float4::new(position.x, position.y, position.z, 0.0),
            lower_left_corner: Float4::new(lower_left_corner.x, lower_left_corner.y, lower_left_corner.z, 0.0),
            horizontal: Float4::new(horizontal.x, horizontal.y, horizontal.z, 0.0),
            vertical: Float4::new(vertical.x, vertical.y, vertical.z, 0.0)
        }
    }
}