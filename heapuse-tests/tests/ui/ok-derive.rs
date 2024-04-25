use heapuse::HeapSize;
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

#[derive(Heap)]
pub struct G1<T: HeapSize> {
    #[heap(add)]
    pub a: T,
}

#[derive(Heap)]
pub struct G2<T: HeapSize> {
    #[heap(add)]
    pub a: Vec<T>,
}

fn main() {}
