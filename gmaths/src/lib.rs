#![allow(dead_code)]

pub mod traits;
pub use traits::*;

pub mod vec;
pub use vec::*;
pub mod mat;
pub use mat::*;
pub mod quat;
pub use quat::*;
pub mod transform;
pub use transform::*;

/*****************************************************************************
*                               TYPES
******************************************************************************/

pub type Float2 = Vector2<f32>;
pub type Float3 = Vector3<f32>;
pub type Float4 = Vector4<f32>;

pub type Int2 = Vector2<i32>;
pub type Int3 = Vector3<i32>;
pub type Int4 = Vector4<i32>;

pub type UInt2 = Vector2<u32>;
pub type UInt3 = Vector3<u32>;
pub type UInt4 = Vector4<u32>;

pub type Float4x4 = Matrix4<f32>;

pub type Quat = Quaternion<f32>;

pub type Transform = Transform3D<f32>;