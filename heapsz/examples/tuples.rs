use heapsz_derive::HeapSize;

#[allow(dead_code)]
#[path = "allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct Unit;

#[derive(HeapSize)]
pub struct Tuple1(#[heap_size] pub allowlisting::FullAllow);

#[derive(HeapSize)]
#[heap_size]
pub struct Tuple2(
    pub allowlisting::FullAllow,
    #[heap_size(skip)] pub allowlisting::PartialAllow,
);

fn main() {}
