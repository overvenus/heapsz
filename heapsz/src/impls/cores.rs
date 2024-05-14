mod primitive {
    use crate::HeapSize;

    impl_heap_size! {
        zero gen_test
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

    impl<T: HeapSize, const N: usize> HeapSize for [T; N] {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self[0].heap_size()
            }
        }
    }

    impl<A: HeapSize, B: HeapSize> HeapSize for (A, B) {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size()
        }
    }

    impl<A: HeapSize, B: HeapSize, C: HeapSize> HeapSize for (A, B, C) {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
        }
    }

    impl<A: HeapSize, B: HeapSize, C: HeapSize, D: HeapSize> HeapSize for (A, B, C, D) {
        fn heap_size(&self) -> usize {
            self.0.heap_size() + self.1.heap_size() + self.2.heap_size() + self.3.heap_size()
        }
    }

    impl<A: HeapSize, B: HeapSize, C: HeapSize, D: HeapSize, E: HeapSize> HeapSize for (A, B, C, D, E) {
        fn heap_size(&self) -> usize {
            self.0.heap_size()
                + self.1.heap_size()
                + self.2.heap_size()
                + self.3.heap_size()
                + self.4.heap_size()
        }
    }

    impl<A: HeapSize, B: HeapSize, C: HeapSize, D: HeapSize, E: HeapSize, F: HeapSize> HeapSize
        for (A, B, C, D, E, F)
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

    impl<A: HeapSize, B: HeapSize, C: HeapSize, D: HeapSize, E: HeapSize, F: HeapSize, G: HeapSize>
        HeapSize for (A, B, C, D, E, F, G)
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

    #[test]
    fn test_array() {
        let b = Box::new([0u8; 8]);
        let a = [b.clone(), b.clone(), b.clone(), b.clone(), b.clone()];
        assert_eq!(b.heap_size() * a.len(), a.heap_size());
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

    #[test]
    fn test_cell() {
        use crate::HeapSize;

        let b = Box::new([0u8; 8]);
        let c = core::cell::Cell::new(b.clone());
        assert_eq!(b.heap_size(), c.heap_size());
    }

    #[test]
    fn test_refcell() {
        use crate::HeapSize;

        let b = Box::new([0u8; 8]);
        let c = core::cell::RefCell::new(b.clone());
        assert_eq!(b.heap_size(), c.heap_size());
    }
}

mod marker {
    impl_heap_size! {
        zero gen_test
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
        zero gen_test
        core::time::Duration,
    }
}

mod option_result {
    use core::{option::Option, result::Result};

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for Option<T> {
        fn heap_size(&self) -> usize {
            match self {
                Option::Some(t) => t.heap_size(),
                Option::None => 0,
            }
        }
    }

    impl<T: HeapSize, E> HeapSize for Result<T, E> {
        /// Return the number of bytes it owns in heap.
        ///
        /// Note: Err heap size is ignored because, in most use cases, we only
        /// care about the Ok variant.
        fn heap_size(&self) -> usize {
            match self {
                Result::Ok(t) => t.heap_size(),
                Result::Err(_) => 0,
            }
        }
    }

    #[test]
    fn test_option() {
        let b = Box::new([0u8; 8]);
        let mut op = Option::Some(b.clone());
        assert_eq!(b.heap_size(), op.heap_size());
        op.take();
        assert_eq!(0, op.heap_size());
    }

    #[test]
    fn test_result() {
        let b = Box::new([0u8; 8]);
        let res = Result::Ok::<_, ()>(b.clone());
        assert_eq!(b.heap_size(), res.heap_size());
        let res = Result::Err::<(), _>(b);
        assert_eq!(0, res.heap_size());
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
        zero gen_test
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
    }
    impl_heap_size! {
        zero
        core::sync::atomic::Ordering,
    }
}

mod task {
    use core::task::Poll;

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for Poll<T> {
        fn heap_size(&self) -> usize {
            match self {
                Poll::Ready(t) => t.heap_size(),
                Poll::Pending => 0,
            }
        }
    }

    #[test]
    fn test_cell() {
        let b = Box::new([0u8; 8]);
        let p = Poll::Ready(b.clone());
        assert_eq!(b.heap_size(), p.heap_size());
        let p = Poll::Pending::<Box<[u8; 8]>>;
        assert_eq!(0, p.heap_size());
    }
}
