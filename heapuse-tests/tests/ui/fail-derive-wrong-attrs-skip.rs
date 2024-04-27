use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct WrongSkip1 {
    #[heap_size(skip)]
    pub a: primitive::StructPrimitives,
}

#[derive(Heap)]
pub enum WrongSkip2 {
    #[heap_size(skip)]
    A(usize),
    B(usize),
}

fn main() {}
