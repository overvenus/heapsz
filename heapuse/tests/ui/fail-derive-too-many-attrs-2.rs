use heapuse_derive::Heap;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(Heap)]
pub struct UnknownAttr {
    #[heap_size]
    #[heap_size(with = "mod::a")]
    pub a: allowlisting::FullAllow,
}

fn main() {}
