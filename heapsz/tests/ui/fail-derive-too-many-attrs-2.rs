use heapsz_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct UnknownAttr {
    #[heap_size]
    #[heap_size(with = "mod::a")]
    pub a: allowlisting::FullAllow,
}

fn main() {}
