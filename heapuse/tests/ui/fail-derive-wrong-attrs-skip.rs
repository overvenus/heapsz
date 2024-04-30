use heapuse_derive::Heap;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(Heap)]
pub struct WrongSkip1 {
    #[heap_size(skip)]
    pub a: allowlisting::FullAllow,
}

#[derive(Heap)]
pub enum WrongSkip2 {
    #[heap_size(skip)]
    A(usize),
    B(usize),
}

fn main() {}
