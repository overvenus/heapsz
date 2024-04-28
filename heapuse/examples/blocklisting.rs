use heapuse_derive::Heap;

#[derive(Heap)]
#[heap_size]
pub struct FullBlock {
    #[heap_size(skip)]
    pub field_bool: bool,
    #[heap_size(skip)]
    pub field_u8: u8,
    #[heap_size(skip)]
    pub field_u16: u16,
    #[heap_size(skip)]
    pub field_u32: u32,
    #[heap_size(skip)]
    pub field_u64: u64,
    #[heap_size(skip)]
    pub field_u128: u128,
    #[heap_size(skip)]
    pub field_usize: usize,
    #[heap_size(skip)]
    pub field_f32: f32,
    #[heap_size(skip)]
    pub field_f64: f64,
}

#[derive(Heap)]
#[heap_size]
pub struct PartialBlock {
    #[heap_size(skip)]
    pub field_bool: bool,
    #[heap_size(skip)]
    pub field_u8: u8,
    #[heap_size(skip)]
    pub field_u16: u16,
    #[heap_size(skip)]
    pub field_u32: u32,
    #[heap_size(skip)]
    pub field_u64: u64,
    pub field_u128: u128,
    pub field_usize: usize,
    pub field_f32: f32,
    pub field_f64: f64,
}

fn main() {}
