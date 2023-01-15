use crate::maths::*;

pub struct Image {
    pub data: *mut u8,
    pub dimensions: Int2,
    pub channel_count: i32
}