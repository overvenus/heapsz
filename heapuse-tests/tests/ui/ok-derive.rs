use heapuse::HeapSize;
use heapuse_derive::Heap;
use heapuse_example::*;

#[derive(Heap)]
pub struct T1 {
    pub a: primitive::StructPrimitives,
}

#[derive(Heap)]
pub struct T2 {
    #[heap(add)]
    pub a: primitive::StructPrimitives,
}

#[derive(Heap)]
pub struct G1<T: HeapSize> {
    #[heap(add)]
    pub a: T,
}

#[derive(Heap)]
pub struct G2<T: HeapSize> {
    #[heap(add)]
    pub a: Vec<T>,
}

#[derive(Heap)]
pub struct M1<T: HeapSize> {
    #[heap(add)]
    #[cfg(test)]
    pub a: Vec<T>,
    #[heap(add)]
    #[cfg(not(test))]
    pub b: Vec<T>,
    #[heap(add)]
    #[cfg(not(test))]
    pub c: Vec<T>,
}

#[derive(Heap)]
#[heap(all)]
pub struct A1<T: HeapSize> {
    #[heap(add)]
    pub a: Vec<T>,
    pub b: Vec<T>,
}

fn main() {}
