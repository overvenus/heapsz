pub mod remote {
    pub struct Inner(pub Box<u8>);
}

pub mod remote_inner_heap_size {
    use heapuse::HeapSize;

    use super::remote;

    pub fn heap_size(t: &remote::Inner) -> usize {
        t.0.heap_size()
    }
}

#[derive(heapuse_derive::HeapSize)]
pub struct With {
    #[heap_size(with = "remote_inner_heap_size")]
    pub inner: remote::Inner,
}

fn main() {}
