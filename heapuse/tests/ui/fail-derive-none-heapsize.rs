use heapuse_derive::HeapSize;

struct NoneHeapSize {
    a: (),
}

#[derive(HeapSize)]
pub struct BadStruct {
    #[heap_size]
    pub a: NoneHeapSize,
}

fn main() {}
