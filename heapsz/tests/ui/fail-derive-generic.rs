use heapsz_derive::HeapSize;

#[derive(HeapSize)]
pub struct G1<T> {
    #[heap_size]
    pub a: T,
}

fn main() {}
