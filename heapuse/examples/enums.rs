use with::{remote::*, *};

#[allow(dead_code)]
#[path = "with.rs"]
mod with;

#[derive(heapuse_derive::Heap)]
pub enum Enum {
    A(#[heap_size] usize),
    B(#[heap_size] usize, usize),
    C(#[heap_size(with = "remote_inner_heap_size")] Inner, usize),
    D {
        #[heap_size(with = "remote_inner_heap_size")]
        inner: Inner,
        #[heap_size]
        size1: usize,
        size2: usize,
    },
}

#[derive(heapuse_derive::Heap)]
#[heap_size]
pub enum EnumAll {
    A(usize),
    B(usize, usize),
    C(#[heap_size(with = "remote_inner_heap_size")] Inner, usize),
    D {
        #[heap_size(with = "remote_inner_heap_size")]
        inner: Inner,
        size1: usize,
        size2: usize,
    },
}

#[derive(heapuse_derive::Heap)]
#[heap_size]
pub enum EnumSkip {
    A(#[heap_size(skip)] Inner, usize),
    B {
        #[heap_size(skip)]
        inner: Inner,
        size1: usize,
        size2: usize,
    },
    #[heap_size(skip)]
    C {
        inner: Inner,
    },
}

fn main() {}
