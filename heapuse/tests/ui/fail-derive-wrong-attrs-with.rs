use heapuse_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct WrongAttr {
    #[heap_size(with)]
    pub a: allowlisting::FullAllow,
}

fn main() {}
