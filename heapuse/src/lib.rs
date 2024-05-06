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
        core::mem::size_of::<Self>() + self.heap_size()
    }
}

mod impls;
