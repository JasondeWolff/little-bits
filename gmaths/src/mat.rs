use std::ops::{Mul, Index, IndexMut, AddAssign, DivAssign};
use std::mem;
use rand::{Rand, Rng};
use num::{Float};

use crate::traits::*;
use crate::Quaternion;
use crate::Vector3;

macro_rules! t(
    ($v: expr) => (
        T::from($v).unwrap()
    )
);

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Matrix4<T: Float> {
    pub elems: [T; 16]
}

/*****************************************************************************
*                               CONSTRUCTION
******************************************************************************/

impl<T: Default + Float> Default for Matrix4<T> {
    #[inline]
    fn default() -> Matrix4<T> {
        Matrix4 {
            elems: [T::default(); 16]
        }
    }
}

impl<T: Float> Matrix4<T> {
    #[inline]
    pub fn new(elems: Vec<T>) -> Matrix4<T> {
        Matrix4 {
            elems: elems.try_into().unwrap_or_else(|_| panic!("Failed to make new Matrix4."))
        }
    }
}

impl<T: Float + Default> Matrix4<T> {
    #[inline]
    pub fn identity() -> Matrix4<T> {
        let mut mat4 = Self::default();

        *mat4.at_mut(0, 0) = t!(1.0);
        *mat4.at_mut(1, 1) = t!(1.0);
        *mat4.at_mut(2, 2) = t!(1.0);
        *mat4.at_mut(3, 3) = t!(1.0);
        mat4
    }

    #[inline]
    pub fn orthographic(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Matrix4<T> {
        let mut mat4 = Self::identity();

        *mat4.at_mut(0, 0) = t!(2.0) / (right - left);
        *mat4.at_mut(1, 1) = t!(2.0) / (top - bottom);
        *mat4.at_mut(2, 2) = t!(-2.0) / (far - near);
        *mat4.at_mut(3, 0) = (left + right) / (right - left);
        *mat4.at_mut(3, 1) = (bottom + top) / (top - bottom);
        *mat4.at_mut(3, 2) = -(far + near) / (far - near);
        mat4
    }

    #[inline]
    pub fn perspective(fov: T, aspect_ratio: T, near: T, far: T) -> Matrix4<T> {
        let mut mat4 = Self::identity();

        let q = t!(1.0) / (t!(0.5) * fov).to_radians().tan();
        let a = q / aspect_ratio;
        let b = (near + far) / (near - far);
        let c = (t!(2.0) * near * far) / (near - far);

        *mat4.at_mut(0, 0) = a;
        *mat4.at_mut(1, 1) = q;
        *mat4.at_mut(2, 2) = b;
        *mat4.at_mut(2, 3) = t!(-1.0);
        *mat4.at_mut(3, 2) = c;
        mat4
    }

    #[inline]
    pub fn translation(amount: Vector3<T>) -> Matrix4<T> {
        let mut mat4 = Self::identity();
        *mat4.at_mut(3, 0) = amount.x;
        *mat4.at_mut(3, 1) = amount.y;
        *mat4.at_mut(3, 2) = amount.z;
        mat4
    }

    #[inline]
    pub fn rotation(angle: T, axis: Vector3<T>) -> Matrix4<T> {
        let mut mat4 = Self::identity();

        let r = angle.to_radians();
        let c = r.cos();
        let s = r.sin();
        let omc = t!(1.0) - c;

        *mat4.at_mut(0, 0) = axis.x * omc + c;
		*mat4.at_mut(0, 1) = axis.y * axis.x * omc + axis.z * s;
		*mat4.at_mut(0, 2) = axis.z * axis.x * omc - axis.y * s;
		*mat4.at_mut(1, 0) = axis.x * axis.y * omc - axis.z * s;
		*mat4.at_mut(1, 1) = axis.y * omc + c;
		*mat4.at_mut(1, 2) = axis.y * axis.z * omc + axis.x * s;
		*mat4.at_mut(2, 0) = axis.x * axis.z * omc + axis.y * s;
		*mat4.at_mut(2, 1) = axis.y * axis.z * omc - axis.x * s;
		*mat4.at_mut(2, 2) = axis.z * omc + c;
        mat4
    }

    #[inline]
    pub fn look_at(eye: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Matrix4<T> where T: Float + Default + AddAssign<T> + DivAssign<T> {
        let mut mat4 = Self::identity();

        let forward = (eye - target).normalized();
        let right = up.cross(forward).normalized();
        let up = forward.cross(right);
        
        *mat4.at_mut(0, 0) = right.x;
		*mat4.at_mut(0, 1) = up.x;
		*mat4.at_mut(0, 2) = forward.x;
		*mat4.at_mut(1, 0) = right.y;
		*mat4.at_mut(1, 1) = up.y;
		*mat4.at_mut(1, 2) = forward.y;
		*mat4.at_mut(2, 0) = right.z;
		*mat4.at_mut(2, 1) = up.z;
		*mat4.at_mut(2, 2) = forward.z;
        *mat4.at_mut(3, 0) = -right.dot(eye);
        *mat4.at_mut(3, 1) = -up.dot(eye);
        *mat4.at_mut(3, 2) = -forward.dot(eye);
        
        mat4
    }

    #[inline]
    pub fn scale(amount: Vector3<T>) -> Matrix4<T> {
        let mut mat4 = Self::identity();
        *mat4.at_mut(0, 0) = amount.x;
        *mat4.at_mut(1, 1) = amount.y;
        *mat4.at_mut(2, 2) = amount.z;
        mat4
    }
}

/*****************************************************************************
*                               MODIFIERS
******************************************************************************/

impl<T: Default + Float> Clear for Matrix4<T> {
    #[inline]
    fn clear(&mut self) {
        self.elems = [T::default(); 16];
    }
}

impl<T: Rand + Float> Rand for Matrix4<T> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Matrix4<T> {
        let mut elems = Vec::with_capacity(16);
        for _ in 0..16 {
            elems.push(Rand::rand(rng));
        }
        Self::new(elems)
    }
}

impl<T: Float> Matrix4<T> {
    pub fn invert(&mut self) {
        let elems: [T; 16] = [
            self[5] * self[10] * self[15] - self[5] * self[11] * self[14] - self[9] * self[6] * self[15] +
			 self[9] * self[7] * self[14] + self[13] * self[6] * self[11] - self[13] * self[7] * self[10],
			-self[1] * self[10] * self[15] + self[1] * self[11] * self[14] + self[9] * self[2] * self[15] -
			 self[9] * self[3] * self[14] - self[13] * self[2] * self[11] + self[13] * self[3] * self[10],
			 self[1] * self[6] * self[15] - self[1] * self[7] * self[14] - self[5] * self[2] * self[15] +
			 self[5] * self[3] * self[14] + self[13] * self[2] * self[7] - self[13] * self[3] * self[6],
			-self[1] * self[6] * self[11] + self[1] * self[7] * self[10] + self[5] * self[2] * self[11] -
			 self[5] * self[3] * self[10] - self[9] * self[2] * self[7] + self[9] * self[3] * self[6],
			-self[4] * self[10] * self[15] + self[4] * self[11] * self[14] + self[8] * self[6] * self[15] -
			 self[8] * self[7] * self[14] - self[12] * self[6] * self[11] + self[12] * self[7] * self[10],
			 self[0] * self[10] * self[15] - self[0] * self[11] * self[14] - self[8] * self[2] * self[15] +
			 self[8] * self[3] * self[14] + self[12] * self[2] * self[11] - self[12] * self[3] * self[10],
			-self[0] * self[6] * self[15] + self[0] * self[7] * self[14] + self[4] * self[2] * self[15] -
			 self[4] * self[3] * self[14] - self[12] * self[2] * self[7] + self[12] * self[3] * self[6],
			 self[0] * self[6] * self[11] - self[0] * self[7] * self[10] - self[4] * self[2] * self[11] +
			 self[4] * self[3] * self[10] + self[8] * self[2] * self[7] - self[8] * self[3] * self[6],
			 self[4] * self[9] * self[15] - self[4] * self[11] * self[13] - self[8] * self[5] * self[15] +
			 self[8] * self[7] * self[13] + self[12] * self[5] * self[11] - self[12] * self[7] * self[9],
			-self[0] * self[9] * self[15] + self[0] * self[11] * self[13] + self[8] * self[1] * self[15] -
			 self[8] * self[3] * self[13] - self[12] * self[1] * self[11] + self[12] * self[3] * self[9],
			 self[0] * self[5] * self[15] - self[0] * self[7] * self[13] - self[4] * self[1] * self[15] +
			 self[4] * self[3] * self[13] + self[12] * self[1] * self[7] - self[12] * self[3] * self[5],
			-self[0] * self[5] * self[11] + self[0] * self[7] * self[9] + self[4] * self[1] * self[11] -
			 self[4] * self[3] * self[9] - self[8] * self[1] * self[7] + self[8] * self[3] * self[5],
			-self[4] * self[9] * self[14] + self[4] * self[10] * self[13] + self[8] * self[5] * self[14] -
			 self[8] * self[6] * self[13] - self[12] * self[5] * self[10] + self[12] * self[6] * self[9],
			 self[0] * self[9] * self[14] - self[0] * self[10] * self[13] - self[8] * self[1] * self[14] +
			 self[8] * self[2] * self[13] + self[12] * self[1] * self[10] - self[12] * self[2] * self[9],
			-self[0] * self[5] * self[14] + self[0] * self[6] * self[13] + self[4] * self[1] * self[14] -
			 self[4] * self[2] * self[13] - self[12] * self[1] * self[6] + self[12] * self[2] * self[5],
			 self[0] * self[5] * self[10] - self[0] * self[6] * self[9] - self[4] * self[1] * self[10] +
			 self[4] * self[2] * self[9] + self[8] * self[1] * self[6] - self[8] * self[2] * self[5]
        ];

        let det = self[0] * elems[0] + self[1] * elems[4] + self[2] * elems[8] + self[3] * elems[12];
        if det != t!(0.0)
        {
            let inv_det = t!(1.0) / det;
            for i in 0..16 {
                self[i] = elems[i] * inv_det;
            }
        }
    }

    pub fn transpose(&mut self) {
        let tmp = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                *self.at_mut(i, j) = tmp.at(j, i);
            }
        }
    }
}

/*****************************************************************************
*                               CONVERSION
******************************************************************************/

impl<T: Float> AsRef<[T; 16]> for Matrix4<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 16] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T: Float> AsMut<[T; 16]> for Matrix4<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 16] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<'a, T: Float> From<&'a [T; 16]> for &'a Matrix4<T> {
    #[inline]
    fn from(arr: &'a [T; 16]) -> &'a Matrix4<T> {
        unsafe {
            mem::transmute(arr)
        }
    }
}

impl<'a, T: Float> From<&'a mut [T; 16]> for &'a mut Matrix4<T> {
    #[inline]
    fn from(arr: &'a mut [T; 16]) -> &'a mut Matrix4<T> {
        unsafe {
            mem::transmute(arr)
        }
    }
}

impl<'a, T: Clone + Float> From<&'a [T; 16]> for Matrix4<T> {
    #[inline]
    fn from(arr: &'a [T; 16]) -> Matrix4<T> {
        let vref: &Matrix4<T> = From::from(arr);
        vref.clone()
    }
}

impl<'a, T: Float + Default> From<Quaternion<T>> for Matrix4<T> {
    #[inline]
    fn from(quat: Quaternion<T>) -> Matrix4<T> {
        let mut mat4 = Self::identity();

        mat4[0] = t!(1.0) - t!(2.0) * quat.y * quat.y - t!(2.0) * quat.z * quat.z;
        mat4[1] = t!(2.0) * quat.x * quat.y - t!(2.0) * quat.w * quat.z;
        mat4[2] = t!(2.0) * quat.x * quat.z + t!(2.0) * quat.w * quat.y;
        mat4[4] = t!(2.0) * quat.x * quat.y + t!(2.0) * quat.w * quat.z;
        mat4[5] = t!(1.0) - t!(2.0) * quat.x * quat.x - t!(2.0) * quat.z * quat.z;
        mat4[6] = t!(2.0) * quat.y * quat.z - t!(2.0) * quat.w * quat.x;
        mat4[8] = t!(2.0) * quat.x * quat.z - t!(2.0) * quat.w * quat.y;
        mat4[9] = t!(2.0) * quat.y * quat.z + t!(2.0) * quat.w * quat.x;
        mat4[10] = t!(1.0) - t!(2.0) * quat.x * quat.x - t!(2.0) * quat.y * quat.y;

        mat4
    }
}

/*****************************************************************************
*                               INDEXING
******************************************************************************/

impl<N: Float, T> Index<T> for Matrix4<N> where [N]: Index<T> {
    type Output = <[N] as Index<T>>::Output;

    fn index(&self, i: T) -> &<[N] as Index<T>>::Output {
        &self.as_ref()[i]
    }
}

impl<N: Float, T> IndexMut<T> for Matrix4<N> where [N]: IndexMut<T> {
    fn index_mut(&mut self, i: T) -> &mut <[N] as Index<T>>::Output {
        &mut self.as_mut()[i]
    }
}

impl<T: Float> Matrix4<T> {
    pub fn at(&self, x: usize, y: usize) -> T {
        self.elems[x * 4 + y]
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.elems[x * 4 + y]
    }
}

/*****************************************************************************
*                               OPERATORS
******************************************************************************/

impl<T: Float + Default + Copy + Mul<T, Output = T>> Mul<Matrix4<T>> for Matrix4<T> {
    type Output = Matrix4<T>;

    #[inline]
    fn mul(self, right: Matrix4<T>) -> Matrix4<T> {
        let mut result = Matrix4::default();
        for y in 0..4 {
            for x in 0..4 {
                let mut sum: T = t!(0.0);
                for e in 0..4 {
                    sum = sum + self[x + e * 4] * right[e + y * 4];
                }
                result[x + y * 4] = sum;
            }
        }
        result
    }
}