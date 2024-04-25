use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct WrongAttr {
    #[heap(with)]
    pub a: primitive::StructPrimitives,
}

fn main() {}
