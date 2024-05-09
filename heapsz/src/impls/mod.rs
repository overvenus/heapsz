macro_rules! impl_heap_size{
    ( zero $($typ: ty,)+ ) => {
        $(
            impl $crate::HeapSize for $typ {
                fn heap_size(&self) -> usize { 0 }
            }
        )+
    };
    ( container $self_: tt, $( $(#[$attr:meta])* $typ: ty => $heap_size_block: block,)+ ) => {
        $(
            impl<T: $crate::HeapSize> $crate::HeapSize for $typ {
                $(#[$attr])*
                fn heap_size(&$self_) -> usize {
                    let inner = $heap_size_block;
                    T::heap_size(inner)
                }
            }
        )+
    }
}

// Implement `HeapSize` for structs in the core crate.
mod cores;

// Implement `HeapSize` for structs in the alloc crate.
mod allocs;

// Implement `HeapSize` for structs in the std crate.
mod stds;

#[cfg(feature = "bytes")]
mod bytes;

#[cfg(feature = "protobuf2")]
mod protobuf2;
