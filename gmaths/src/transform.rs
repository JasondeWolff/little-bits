use num::Float;

use crate::Vector3;
use crate::Quaternion;
use crate::Matrix4;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Transform3D<T: Float> {
    translation: Vector3<T>,
    rotation: Quaternion<T>,
    scale: Vector3<T>,

    model: Matrix4<T>,
    model_inv_trans: Matrix4<T>,
    model_dirty: bool
}

macro_rules! t(
    ($v: expr) => (
        T::from($v).unwrap()
    )
);

impl<T: Float + Default> Transform3D<T> {
    pub fn new() -> Self {
        Transform3D {
            translation: Vector3::default(),
            rotation: Quaternion::identity(),
            scale: Vector3::new(t!(1.0), t!(1.0), t!(1.0)),
            model: Matrix4::identity(),
            model_inv_trans: Matrix4::identity(),
            model_dirty: true
        }
    }

    pub fn get_translation(&self) -> Vector3<T> {
        self.translation
    }

    pub fn get_rotation(&self) -> Quaternion<T> {
        self.rotation
    }
    
    pub fn get_scale(&self) -> Vector3<T> {
        self.scale
    }

    pub fn set_translation(&mut self, translation: Vector3<T>) {
        self.translation = translation;
        self.model_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<T>) {
        self.rotation = rotation;
        self.model_dirty = true;
    }

    pub fn set_scale(&mut self, scale: Vector3<T>) {
        self.scale = scale;
        self.model_dirty = true;
    }

    pub fn translate(&mut self, translation: Vector3<T>) {
        self.set_translation(self.translation + translation);
    }

    pub fn scale(&mut self, scale: Vector3<T>) {
        self.set_scale(self.scale + scale);
    }

    pub fn get_matrix(&mut self) -> Matrix4<T> {
        if self.model_dirty {
            self.model = Matrix4::translation(self.translation) * Matrix4::from(self.rotation) * Matrix4::scale(self.scale);
            self.model_inv_trans = self.model.clone();
            self.model_inv_trans.invert();
            self.model_inv_trans.transpose();
            self.model_dirty = false;
        }

        self.model
    }

    pub fn get_inv_trans_matrix(&mut self) -> Matrix4<T> {
        if self.model_dirty {
            self.model = Matrix4::translation(self.translation) * Matrix4::from(self.rotation) * Matrix4::scale(self.scale);
            self.model_inv_trans = self.model.clone();
            self.model_inv_trans.invert();
            self.model_inv_trans.transpose();
            self.model_dirty = false;
        }

        self.model_inv_trans
    }
}