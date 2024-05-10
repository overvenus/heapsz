use std::sync::Arc;

use heapsz::HeapSize;
use heapsz_derive::HeapSize;

#[path = "../../examples/allowlisting.rs"]
mod allowlisting;

#[derive(HeapSize)]
pub struct T1 {
    pub a: allowlisting::FullAllow,
}

#[derive(HeapSize)]
pub struct T2 {
    #[heap_size]
    pub a: allowlisting::FullAllow,
}

#[derive(HeapSize)]
pub struct G1<T: HeapSize> {
    #[heap_size]
    pub a: T,
}

#[derive(HeapSize)]
pub struct G2<T: HeapSize> {
    #[heap_size]
    pub a: Vec<T>,
}

#[derive(HeapSize)]
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

#[derive(HeapSize)]
#[heap_size]
pub struct A1<T: HeapSize> {
    #[heap_size]
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Box<[T]>,
    pub d: Box<[T; 5]>,
    pub e: Arc<[T]>,
    pub f: Arc<[T; 5]>,
}

#[derive(HeapSize)]
#[heap_size]
pub enum E1 {
    V1(usize, Vec<usize>),
    V2 { a: usize, b: Vec<usize> },
}

#[derive(HeapSize)]
pub enum E2 {
    V1(#[heap_size] usize, Vec<usize>),
    V2 {
        a: usize,
        #[heap_size]
        b: Vec<usize>,
    },
}

#[derive(HeapSize)]
#[heap_size]
pub enum E3 {
    V1(usize, Vec<usize>),
    #[heap_size(skip)]
    V2 {
        a: usize,
        b: Vec<usize>,
    },
}

#[derive(HeapSize)]
#[heap_size]
pub enum E4 {}

#[derive(HeapSize)]
#[heap_size]
pub enum E5 {
    A = 0,
    B = 1,
}

fn main() {}
