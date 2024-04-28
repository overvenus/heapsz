use core::mem;

#[cfg(docsrs)]
pub mod examples;

/// Query the number of bytes of an object.
pub trait HeapSize {
    /// Return the number of bytes it owns in heap.
    ///
    /// N.B. the implementation should be performant, as it is often called on
    /// performance critical path.
    fn heap_size(&self) -> usize;

    fn memory_size(&self) -> usize
    where
        Self: Sized,
    {
        mem::size_of::<Self>() + self.heap_size()
    }
}

macro_rules! impl_zero_heap_size{
    ( $($typ: ty,)+ ) => {
        $(
            impl HeapSize for $typ {
                fn heap_size(&self) -> usize { 0 }
            }
        )+
    }
}

impl_zero_heap_size! {
    bool,
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize,
    f32, f64,
}

impl<A: HeapSize, B: HeapSize> HeapSize for (A, B) {
    fn heap_size(&self) -> usize {
        self.0.heap_size() + self.1.heap_size()
    }
}

impl<T: HeapSize> HeapSize for Option<T> {
    fn heap_size(&self) -> usize {
        match self {
            Some(t) => t.heap_size(),
            None => 0,
        }
    }
}

mod std_impl {
    use std::collections::HashMap;

    use super::*;

    // Do not impl HeapSize for [T], because type coercions make it error-prone.
    // E.g., Vec[u8] may be casted to &[u8] which does not own any byte in heap.
    impl<T: HeapSize> HeapSize for Vec<T> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * std::mem::size_of::<T>();
            if self.is_empty() {
                cap_bytes
            } else {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self[0].heap_size() + cap_bytes
            }
        }
    }

    impl<K: HeapSize, V: HeapSize> HeapSize for HashMap<K, V> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * (mem::size_of::<K>() + mem::size_of::<V>());
            if self.is_empty() {
                cap_bytes
            } else {
                let kv = self.iter().next().unwrap();
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                cap_bytes + self.len() * (kv.0.heap_size() + kv.1.heap_size())
            }
        }
    }
}
