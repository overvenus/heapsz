use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct UnknownAttr {
    #[heap(add)]
    #[heap(add)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
