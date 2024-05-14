use heapsz_derive::HeapSize;

#[derive(HeapSize)]
pub struct T1 {
    #[heap_size]
    pub a: usize,
    #[heap_size]
    #[cfg(feature = "bytes")]
    pub b: Vec<u8>,
    #[heap_size]
    #[cfg(not(feature = "bytes"))]
    pub c: Vec<u8>,
}

fn main() {}
