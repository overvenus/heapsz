use heapuse::HeapSize;
use heapuse_derive::Heap;

#[path = "../../../heapuse/examples/allowlisting.rs"]
mod allowlisting;

#[derive(Heap)]
pub struct T1 {
    pub a: allowlisting::FullAllow,
}

#[derive(Heap)]
pub struct T2 {
    #[heap_size]
    pub a: allowlisting::FullAllow,
}

#[derive(Heap)]
pub struct G1<T: HeapSize> {
    #[heap_size]
    pub a: T,
}

#[derive(Heap)]
pub struct G2<T: HeapSize> {
    #[heap_size]
    pub a: Vec<T>,
}

#[derive(Heap)]
pub struct M1<T: HeapSize> {
    #[heap_size]
    #[cfg(test)]
    pub a: Vec<T>,
    #[heap_size]
    #[cfg(not(test))]
    pub b: Vec<T>,
    #[heap_size]
    #[cfg(not(test))]
    pub c: Vec<T>,
}

#[derive(Heap)]
#[heap_size]
pub struct A1<T: HeapSize> {
    #[heap_size]
    pub a: Vec<T>,
    pub b: Vec<T>,
}

#[derive(Heap)]
#[heap_size]
pub enum E1 {
    V1(usize, Vec<usize>),
    V2 { a: usize, b: Vec<usize> },
}

#[derive(Heap)]
pub enum E2 {
    V1(#[heap_size] usize, Vec<usize>),
    V2 {
        a: usize,
        #[heap_size]
        b: Vec<usize>,
    },
}

#[derive(Heap)]
#[heap_size]
pub enum E3 {
    V1(usize, Vec<usize>),
    #[heap_size(skip)]
    V2 {
        a: usize,
        b: Vec<usize>,
    },
}

fn main() {}
