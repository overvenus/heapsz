use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct WrongAttr {
    #[heap(add = "some::mod")]
    pub a: primitive::StructPrimitives,
}

fn main() {}
