impl crate::HeapSize for bytes::Bytes {
    fn heap_size(&self) -> usize {
        self.len()
    }
}
