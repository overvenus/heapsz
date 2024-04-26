use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct WrongAttr {
    #[heap_size(with)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
