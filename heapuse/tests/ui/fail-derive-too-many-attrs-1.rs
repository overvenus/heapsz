use heapuse_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct UnknownAttr {
    #[heap_size(with = "mod::a", with = "mod::b")]
    pub a: allowlisting::FullAllow,
}

fn main() {}
