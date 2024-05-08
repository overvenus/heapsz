use heapsz_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct UnknownAttr {
    #[heap_size(unknown)]
    pub a: allowlisting::FullAllow,
}

fn main() {}
