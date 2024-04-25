use heapuse_derive::Heap;

struct NoneHeapSize {
    a: ()
}

#[derive(Heap)]
pub struct BadStruct {
    #[heap(add)]
    pub a: NoneHeapSize,
}

fn main() {}
