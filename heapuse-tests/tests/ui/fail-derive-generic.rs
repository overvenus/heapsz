use heapuse_derive::Heap;

#[derive(Heap)]
pub struct G1<T> {
    #[heap_size]
    pub a: T,
}

fn main() {}
