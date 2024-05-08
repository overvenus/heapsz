use heapsz_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct WrongSkip1 {
    #[heap_size(skip)]
    pub a: allowlisting::FullAllow,
}

#[derive(HeapSize)]
pub enum WrongSkip2 {
    #[heap_size(skip)]
    A(usize),
    B(usize),
}

fn main() {}
