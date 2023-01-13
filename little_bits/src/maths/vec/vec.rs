use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use std::mem;
use rand::{Rand, Rng};
use num::{Float};
use std::f64::consts::PI;
use std::f64::consts::FRAC_PI_2;

use crate::traits::*;
use crate::maths::Quaternion;

#[path = "vec_macros.rs"] mod vec_macros;
use vec_macros::*;

/*****************************************************************************
*                               STRUCTS
******************************************************************************/

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

/*****************************************************************************
*                               GLOBAL FUNCS
******************************************************************************/

pub fn dot<V : Dot<V, N>, N>(a: V, b: V) -> N {
    a.dot(b)
}

pub fn cross<V : Cross<V, N>, N>(a: V, b: V) -> N {
    a.cross(b)
}

pub fn normalize<V : Normalize<N>, N>(a: &mut V) {
    a.normalize();
}

pub fn normalized<V : Normalize<V>, N>(a: V) -> V {
    a.normalized()
}

pub fn distance<V: Distance<V, N>, N>(a: V, b: V) -> N {
    a.distance(b)
}

pub fn lerp<V: Lerp<V, V, N>, N>(a: V, b: V, t: N) -> V {
    a.lerp(b, t)
}

/*****************************************************************************
*                               IMPLEMENTATION
******************************************************************************/

// Vector2
default_impl!(Vector2, x, y);
new_impl!(Vector2, x, y);
clear_impl!(Vector2, x, y);
rand_impl!(Vector2, x, y);
conversion_impl!(Vector2, 2);
index_impl!(Vector2);

add_impl!(Vector2, x, y);
scalar_add_impl!(Vector2, x, y);
sub_impl!(Vector2, x, y);
scalar_sub_impl!(Vector2, x, y);
mul_impl!(Vector2, x, y);
scalar_mul_impl!(Vector2, x, y);
div_impl!(Vector2, x, y);
scalar_div_impl!(Vector2, x, y);
neg_impl!(Vector2, x, y);

magnitude_impl!(Vector2, x, y);
dot_impl!(Vector2, x, y);
normalize_impl!(Vector2, x, y);
distance_impl!(Vector2, x, y);
lerp_impl!(Vector2, x, y);

// Vector3
default_impl!(Vector3, x, y, z);
new_impl!(Vector3, x, y, z);
clear_impl!(Vector3, x, y, z);
rand_impl!(Vector3, x, y, z);
conversion_impl!(Vector3, 2);
index_impl!(Vector3);

add_impl!(Vector3, x, y, z);
scalar_add_impl!(Vector3, x, y, z);
sub_impl!(Vector3, x, y, z);
scalar_sub_impl!(Vector3, x, y, z);
mul_impl!(Vector3, x, y, z);
scalar_mul_impl!(Vector3, x, y, z);
div_impl!(Vector3, x, y, z);
scalar_div_impl!(Vector3, x, y, z);
neg_impl!(Vector3, x, y, z);

magnitude_impl!(Vector3, x, y, z);
dot_impl!(Vector3, x, y, z);
cross_impl!(Vector3, x, y, z);
normalize_impl!(Vector3, x, y, z);
distance_impl!(Vector3, x, y, z);
lerp_impl!(Vector3, x, y, z);

macro_rules! t(
    ($v: expr) => (
        T::from($v).unwrap()
    )
);

impl<'a, T: Float + Default> From<Quaternion<T>> for Vector3<T> {
    fn from(quat: Quaternion<T>) -> Vector3<T> {
        let mut euler = Self::default();

        let sinr_cosp = t!(2.0) * (quat.w * quat.x + quat.y * quat.z);
        let cosr_cosp = t!(1.0) - t!(2.0) * (quat.x * quat.x + quat.y * quat.y);
        euler.x = sinr_cosp.atan2(cosr_cosp).to_degrees();

        let sinp = t!(2.0) * (quat.w * quat.y - quat.z * quat.x);
        if sinp.abs() >= t!(1.0) {
            euler.y = t!(FRAC_PI_2).copysign(sinp).to_degrees();
        }
        else {
            euler.y = sinp.asin().to_degrees();
        }

        let siny_cosp = t!(2.0) * (quat.w * quat.z + quat.x * quat.y);
        let cosy_cosp = t!(1.0) - t!(2.0) * (quat.y * quat.y + quat.z * quat.z);
        euler.z = siny_cosp.atan2(cosy_cosp).to_degrees();

        euler
    }
}

// Vector4
default_impl!(Vector4, x, y, z, w);
new_impl!(Vector4, x, y, z, w);
clear_impl!(Vector4, x, y, z, w);
rand_impl!(Vector4, x, y, z, w);
conversion_impl!(Vector4, 2);
index_impl!(Vector4);

add_impl!(Vector4, x, y, z, w);
scalar_add_impl!(Vector4, x, y, z, w);
sub_impl!(Vector4, x, y, z, w);
scalar_sub_impl!(Vector4, x, y, z, w);
mul_impl!(Vector4, x, y, z, w);
scalar_mul_impl!(Vector4, x, y, z, w);
div_impl!(Vector4, x, y, z, w);
scalar_div_impl!(Vector4, x, y, z, w);
neg_impl!(Vector4, x, y, z, w);

magnitude_impl!(Vector4, x, y, z, w);
dot_impl!(Vector4, x, y, z, w);
normalize_impl!(Vector4, x, y, z, w);
distance_impl!(Vector4, x, y, z, w);
lerp_impl!(Vector4, x, y, z, w);