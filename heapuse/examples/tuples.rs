use heapuse_derive::Heap;

#[allow(dead_code)]
#[path = "allowlisting.rs"]
mod allowlisting;

#[derive(Heap)]
pub struct Unit;

#[derive(Heap)]
pub struct Tuple1(#[heap_size] allowlisting::FullAllow);

#[derive(Heap)]
#[heap_size]
pub struct Tuple2(
    allowlisting::FullAllow,
    #[heap_size(skip)] allowlisting::PartialAllow,
);

fn main() {}
