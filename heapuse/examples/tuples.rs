use heapuse_derive::HeapSize;

#[allow(dead_code)]
#[path = "allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct Unit;

#[derive(HeapSize)]
pub struct Tuple1(#[heap_size] allowlisting::FullAllow);

#[derive(HeapSize)]
#[heap_size]
pub struct Tuple2(
    allowlisting::FullAllow,
    #[heap_size(skip)] allowlisting::PartialAllow,
);

fn main() {}
