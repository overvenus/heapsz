use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct G1<T> {
    #[heap(add)]
    pub a: T,
}

fn main() {}
