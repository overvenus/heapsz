use heapuse_derive::HeapSize;

#[derive(HeapSize)]
union Union {
    f1: u32,
    f2: f32,
}

fn main() {}
