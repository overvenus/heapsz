use heapuse_derive::Heap;

#[derive(Heap)]
pub struct Blank {
    pub field_bool: bool,
    pub field_u8: u8,
    pub field_u16: u16,
    pub field_u32: u32,
    pub field_u64: u64,
    pub field_u128: u128,
    pub field_usize: usize,
    pub field_f32: f32,
    pub field_f64: f64,
}

fn main() {}
