impl<T: crate::HeapSize> crate::HeapSize for protobuf2::SingularField<T> {
    fn heap_size(&self) -> usize {
        match self.as_ref() {
            Some(t) => t.heap_size(),
            None => 0,
        }
    }
}

impl<T: crate::HeapSize> crate::HeapSize for protobuf2::SingularPtrField<T> {
    fn heap_size(&self) -> usize {
        match self.as_ref() {
            // SingularPtrField is a wrapper of `Option<Box<T>>`.
            Some(t) => t.memory_size(),
            None => 0,
        }
    }
}

impl<T: crate::HeapSize> crate::HeapSize for protobuf2::RepeatedField<T> {
    fn heap_size(&self) -> usize {
        let cap_bytes = self.capacity() * core::mem::size_of::<T>();
        if self.is_empty() {
            cap_bytes
        } else {
            // Prefer an approximation of its actually heap size, because we
            // want the time complexity to be O(1).
            self.len() * self[0].heap_size() + cap_bytes
        }
    }
}

impl_heap_size! {
    zero
    protobuf2::UnknownFields,
    protobuf2::CachedSize,
}
