use std::ops::{Mul, AddAssign, MulAssign, Index, IndexMut};
use std::mem;
use rand::{Rand, Rng};
use num::{Float};

use crate::traits::*;
use crate::Matrix4;
use crate::Vector3;

macro_rules! t(
    ($v: expr) => (
        T::from($v).unwrap()
    )
);

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Quaternion<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

/*****************************************************************************
*                               CONSTRUCTION
******************************************************************************/

impl<T: Default + Float> Default for Quaternion<T> {
    #[inline]
    fn default() -> Quaternion<T> {
        Quaternion {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default()
        }
    }
}

impl<T: Float> Quaternion<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Quaternion<T> {
        Quaternion {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
}

impl<T: Float> Quaternion<T> {
    #[inline]
    pub fn identity() -> Quaternion<T> {
        Quaternion {
            x: t!(0.0),
            y: t!(0.0),
            z: t!(0.0),
            w: t!(1.0)
        }
    }
}

impl<T: Float> Quaternion<T> {
    #[inline]
    pub fn axis_angle(axis: Vector3<T>, angle: T) -> Quaternion<T> {
        let rangle = angle.to_radians();
        let sha = (rangle * t!(0.5)).sin();

        Quaternion {
            x: axis.x * sha,
            y: axis.y * sha,
            z: axis.z * sha,
            w: (rangle * t!(0.5)).cos()
        }
    }
}

/*****************************************************************************
*                               MODIFIERS
******************************************************************************/

impl<T: Default + Float> Clear for Quaternion<T> {
    #[inline]
    fn clear(&mut self) {
        self.x = T::default();
        self.y = T::default();
        self.z = T::default();
        self.w = T::default();
    }
}

impl<T: Rand + Float> Rand for Quaternion<T> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Quaternion<T> {
        Self::new(Rand::rand(rng), Rand::rand(rng), Rand::rand(rng), Rand::rand(rng))
    }
}

/*****************************************************************************
*                               CONVERSION
******************************************************************************/

impl<T: Float> AsRef<[T; 4]> for Quaternion<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T: Float> AsMut<[T; 4]> for Quaternion<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<'a, T: Float> From<&'a [T; 4]> for &'a Quaternion<T> {
    #[inline]
    fn from(arr: &'a [T; 4]) -> &'a Quaternion<T> {
        unsafe {
            mem::transmute(arr)
        }
    }
}

impl<'a, T: Float> From<&'a mut [T; 4]> for &'a mut Quaternion<T> {
    #[inline]
    fn from(arr: &'a mut [T; 4]) -> &'a mut Quaternion<T> {
        unsafe {
            mem::transmute(arr)
        }
    }
}

impl<'a, T: Clone + Float> From<&'a [T; 4]> for Quaternion<T> {
    #[inline]
    fn from(arr: &'a [T; 4]) -> Quaternion<T> {
        let vref: &Quaternion<T> = From::from(arr);
        vref.clone()
    }
}

impl<'a, T: Float + Default> From<Matrix4<T>> for Quaternion<T> {
    fn from(mat4: Matrix4<T>) -> Quaternion<T> {
        let tr = mat4[0] + mat4[5] + mat4[10];

        if tr > T::default() {
            let s = (tr + t!(1.0)).sqrt() * t!(2.0);

            Quaternion {
                x: (mat4.at(2, 1) - mat4.at(1, 2)) / s,
                y: (mat4.at(0, 2) - mat4.at(2, 0)) / s,
                z: (mat4.at(1, 0) - mat4.at(0, 1)) / s,
                w: t!(0.25) * s
            }
        }
        else if mat4.at(0, 0) > mat4.at(1, 1) && mat4.at(0, 0) > mat4.at(2, 2) {
            let s = (t!(1.0) + mat4.at(0, 0) - mat4.at(1, 1) - mat4.at(2, 2)).sqrt() * t!(2.0);

            Quaternion {
                x: t!(0.25) * s,
                y: (mat4.at(0, 1) + mat4.at(1, 0)) / s,
                z: (mat4.at(0, 2) + mat4.at(2, 0)) / s,
                w: (mat4.at(2, 1) - mat4.at(1, 2)) / s
            }
        }
        else if mat4.at(1, 1) > mat4.at(2, 2) {
            let s = (t!(1.0) + mat4.at(1, 1) - mat4.at(0, 0) - mat4.at(2, 2)).sqrt() * t!(2.0);

            Quaternion {
                x: (mat4.at(0, 1) + mat4.at(1, 0)) / s,
                y: t!(0.25) * s,
                z: (mat4.at(1, 2) + mat4.at(2, 1)) / s,
                w: (mat4.at(0, 2) - mat4.at(2, 0)) / s
            }
        }
        else {
            let s = (t!(1.0) + mat4.at(2, 2) - mat4.at(0, 0) - mat4.at(1, 1)).sqrt() * t!(2.0);

            Quaternion {
                x: (mat4.at(0, 2) + mat4.at(2, 0)) / s,
                y: (mat4.at(1, 2) + mat4.at(2, 1)) / s,
                z: t!(0.25) * s,
                w: (mat4.at(1, 0) - mat4.at(0, 1)) / s
            }
        }
    }
}

// impl<'a, T: Float + Default> From<Vector3<T>> for Quaternion<T> {
//     fn from(euler: Vector3<T>) -> Quaternion<T> {
//         let half_euler = euler * t!(0.5);
//         let cr = (half_euler.x.to_radians()).cos();
//         let sr = (half_euler.x.to_radians()).sin();
//         let cy = (half_euler.z.to_radians()).cos();
//         let sy = (half_euler.z.to_radians()).sin();
//         let cp = (half_euler.y.to_radians()).cos();
//         let sp = (half_euler.y.to_radians()).sin();

//         Quaternion {
//             x: sr * cp * cy - cr * sp * sy,
//             y: cr * sp * cy + sr * cp * sy,
//             z: cr * cp * sy - sr * sp * cy,
//             w: cr * cp * cy + sr * sp * sy
//         }
//     }
// }

impl<'a, T: Float + Default> From<Vector3<T>> for Quaternion<T> {
    fn from(euler: Vector3<T>) -> Quaternion<T> {
        let qx = Quaternion::axis_angle(Vector3::right(), euler.x);
        let qy = Quaternion::axis_angle(Vector3::up(), euler.y);
        let qz = Quaternion::axis_angle(Vector3::forward(), euler.z);
        qx * qy * qz
    }
}

/*****************************************************************************
*                               INDEXING
******************************************************************************/

impl<N: Float, T> Index<T> for Quaternion<N> where [N]: Index<T> {
    type Output = <[N] as Index<T>>::Output;

    fn index(&self, i: T) -> &<[N] as Index<T>>::Output {
        &self.as_ref()[i]
    }
}

impl<N: Float, T> IndexMut<T> for Quaternion<N> where [N]: IndexMut<T> {
    fn index_mut(&mut self, i: T) -> &mut <[N] as Index<T>>::Output {
        &mut self.as_mut()[i]
    }
}

/*****************************************************************************
*                               OPERATORS
******************************************************************************/

impl<T: Float + Copy + Mul<T, Output = T>> Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    #[inline]
    fn mul(self, right: Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            x: self.w * right.x + self.x * right.w + self.y * right.z - self.z * right.y,
			y: self.w * right.y - self.x * right.z + self.y * right.w + self.z * right.x,
			z: self.w * right.z + self.x * right.y - self.y * right.x + self.z * right.w,
            w: self.w * right.w - self.x * right.x - self.y * right.y - self.z * right.z
        }
    }
}

impl<T: Float + Copy + MulAssign<T>> MulAssign<Quaternion<T>> for Quaternion<T> {
    #[inline]
    fn mul_assign(&mut self, right: Quaternion<T>) {
        let tmp = self.clone();

        self.x = tmp.w * right.x + tmp.x * right.w + tmp.y * right.z - tmp.z * right.y;
		self.y = tmp.w * right.y - tmp.x * right.z + tmp.y * right.w + tmp.z * right.x;
		self.z = tmp.w * right.z + tmp.x * right.y - tmp.y * right.x + tmp.z * right.w;
        self.w = tmp.w * right.w - tmp.x * right.x - tmp.y * right.y - tmp.z * right.z;
    }
}

impl<T: Float + Default + AddAssign<T> + Copy + Mul<T, Output = T>> Mul<Vector3<T>> for Quaternion<T> {
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, right: Vector3<T>) -> Vector3<T> {
        let qv = Vector3::<T>::new(self.x, self.y, self.z);
        let t = qv.cross(right) * t!(2.0);
        right + t * self.w + qv.cross(t)
    }
}