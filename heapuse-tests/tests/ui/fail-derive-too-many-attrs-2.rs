use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct UnknownAttr {
    #[heap_size]
    #[heap_size(with = "mod::a")]
    pub a: primitive::StructPrimitives,
}

fn main() {}
