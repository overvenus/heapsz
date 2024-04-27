use heapuse_derive::Heap;

#[derive(Heap)]
union Union {
    f1: u32,
    f2: f32,
}

fn main() {}
