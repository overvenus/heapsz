#![doc = include_str!("../README.md")]

#[cfg(docsrs)]
pub mod examples;

/// Query the number of bytes of an object.
pub trait HeapSize {
    /// Return the number of bytes it owns on the heap.
    fn heap_size(&self) -> usize;

    /// Return the number of bytes it owns on both the heap and the stack.
    fn memory_size(&self) -> usize
    where
        Self: Sized,
    {
        core::mem::size_of::<Self>() + self.heap_size()
    }
}

mod impls;
