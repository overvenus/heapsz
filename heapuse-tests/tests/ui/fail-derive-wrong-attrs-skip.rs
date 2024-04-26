use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct WrongAttr {
    #[heap_size(skip)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
