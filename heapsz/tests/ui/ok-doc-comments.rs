use heapsz_derive::HeapSize;

#[derive(HeapSize)]
pub struct T1 {
    /// This is a doc comments.
    /// This is another doc comments.
    pub a: usize,
}

#[derive(HeapSize)]
#[heap_size]
pub struct T2 {
    /// This is a doc comments.
    /// This is another doc comments.
    #[heap_size(skip)]
    pub a: usize,
}

#[derive(HeapSize)]
#[heap_size]
pub struct T3 {
    /// This is a doc comments.
    /// This is another doc comments.
    pub a: usize,
}

fn main() {}
