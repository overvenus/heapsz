use heapuse_derive::Heap;

struct NoneHeapSize {
    a: (),
}

#[derive(Heap)]
pub struct BadStruct {
    #[heap_size]
    pub a: NoneHeapSize,
}

fn main() {}
