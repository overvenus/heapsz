mod primitive {
    impl_heap_size! {
        zero
        bool,
        u8, i8,
        u16, i16,
        u32, i32,
        u64, i64,
        u128, i128,
        usize, isize,
        f32, f64,
        (), char,
    }

    impl<T: crate::HeapSize, const N: usize> crate::HeapSize for [T; N] {
        fn heap_size(&self) -> usize {
            if !self.is_empty() {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self[0].heap_size()
            } else {
                0
            }
        }
    }

    impl<A: crate::HeapSize, B: crate::HeapSize> crate::HeapSize for (A, B) {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size()
        }
    }

    impl<A: crate::HeapSize, B: crate::HeapSize, C: crate::HeapSize> crate::HeapSize for (A, B, C) {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
        }
    }

    impl<A: crate::HeapSize, B: crate::HeapSize, C: crate::HeapSize, D: crate::HeapSize>
        crate::HeapSize for (A, B, C, D)
    {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size() + self.2.heap_size() + self.3.heap_size()
        }
    }

    impl<
        A: crate::HeapSize,
        B: crate::HeapSize,
        C: crate::HeapSize,
        D: crate::HeapSize,
        E: crate::HeapSize,
    > crate::HeapSize for (A, B, C, D, E)
    {
        fn heap_size(&self) -> usize {
            self.0.heap_size()
                + self.1.heap_size()
                + self.2.heap_size()
                + self.3.heap_size()
                + self.4.heap_size()
        }
    }

    impl<
        A: crate::HeapSize,
        B: crate::HeapSize,
        C: crate::HeapSize,
        D: crate::HeapSize,
        E: crate::HeapSize,
        F: crate::HeapSize,
    > crate::HeapSize for (A, B, C, D, E, F)
    {
        fn heap_size(&self) -> usize {
            self.0.heap_size()
                + self.1.heap_size()
                + self.2.heap_size()
                + self.3.heap_size()
                + self.4.heap_size()
                + self.5.heap_size()
        }
    }

    impl<
        A: crate::HeapSize,
        B: crate::HeapSize,
        C: crate::HeapSize,
        D: crate::HeapSize,
        E: crate::HeapSize,
        F: crate::HeapSize,
        G: crate::HeapSize,
    > crate::HeapSize for (A, B, C, D, E, F, G)
    {
        fn heap_size(&self) -> usize {
            self.0.heap_size()
                + self.1.heap_size()
                + self.2.heap_size()
                + self.3.heap_size()
                + self.4.heap_size()
                + self.5.heap_size()
                + self.6.heap_size()
        }
    }
}

mod cell {
    impl_heap_size! {
        container
        self,
        core::cell::Cell<T> => {
            // SAFETY: This can cause data races if a separate thread calls
            // Cell::replace, but `Cell` is `!Sync` so this won't happen.
            unsafe { &*self.as_ptr() }
        },
        /// Return the number of bytes it owns in heap.
        ///
        /// # Panics
        ///
        /// Panics if the value is currently mutably borrowed.
        core::cell::RefCell<T> => {
           &*self.borrow()
        },
    }
}

mod marker {
    impl_heap_size! {
        zero
        core::marker::PhantomPinned,
    }

    impl<T: ?Sized> crate::HeapSize for core::marker::PhantomData<T> {
        fn heap_size(&self) -> usize {
            0
        }
    }
}

mod time {
    impl_heap_size! {
        zero
        core::time::Duration,
    }
}

mod option_result {
    impl<T: crate::HeapSize> crate::HeapSize for core::option::Option<T> {
        fn heap_size(&self) -> usize {
            match self {
                core::option::Option::Some(t) => t.heap_size(),
                core::option::Option::None => 0,
            }
        }
    }

    impl<T: crate::HeapSize, E> crate::HeapSize for core::result::Result<T, E> {
        fn heap_size(&self) -> usize {
            match self {
                core::result::Result::Ok(t) => t.heap_size(),
                core::result::Result::Err(_) => 0,
            }
        }
    }
}

mod net {
    impl_heap_size! {
        zero
        core::net::IpAddr,
        core::net::Ipv4Addr,
        core::net::Ipv6Addr,
        core::net::SocketAddr,
        core::net::SocketAddrV4,
        core::net::SocketAddrV6,
    }
}

mod sync {
    impl_heap_size! {
        zero
        core::sync::atomic::AtomicBool,
        core::sync::atomic::AtomicI8,
        core::sync::atomic::AtomicU8,
        core::sync::atomic::AtomicI16,
        core::sync::atomic::AtomicU16,
        core::sync::atomic::AtomicI32,
        core::sync::atomic::AtomicU32,
        core::sync::atomic::AtomicI64,
        core::sync::atomic::AtomicU64,
        core::sync::atomic::AtomicIsize,
        core::sync::atomic::AtomicUsize,
        core::sync::atomic::Ordering,
    }
}

mod task {
    impl<T: crate::HeapSize> crate::HeapSize for core::task::Poll<T> {
        fn heap_size(&self) -> usize {
            match self {
                core::task::Poll::Ready(t) => t.heap_size(),
                core::task::Poll::Pending => 0,
            }
        }
    }
}
