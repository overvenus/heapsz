use heapuse_derive::Heap;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(Heap)]
pub struct WrongAttr {
    #[heap_size(with)]
    pub a: allowlisting::FullAllow,
}

fn main() {}