use crate::gmaths::Int2;

#[derive(Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub dimensions: Int2,
    pub channel_count: i32
}