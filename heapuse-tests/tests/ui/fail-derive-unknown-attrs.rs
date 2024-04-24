use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct UnknownAttr {
    #[heap(unknown)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
