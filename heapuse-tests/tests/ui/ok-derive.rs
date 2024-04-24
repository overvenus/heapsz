use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct T1 {
    pub a: primitive::StructPrimitives,
}

#[derive(Heap)]
pub struct T2 {
    #[heap(add)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
