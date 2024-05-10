mod vec_box_string {
    use std as alloc;

    // Do not impl HeapSize for [T], because type coercions make it error-prone.
    // E.g., Vec[u8] may be casted to &[u8] which does not own any byte in heap.
    impl<T: crate::HeapSize> crate::HeapSize for alloc::vec::Vec<T> {
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

    impl<T: crate::HeapSize> crate::HeapSize for alloc::boxed::Box<T> {
        fn heap_size(&self) -> usize {
            // A Box<T> stores the entire T in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for alloc::boxed::Box<[T]> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self.len() * T::memory_size(&self[0])
            }
        }
    }

    impl crate::HeapSize for alloc::string::String {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }
}

mod borrow {
    use std as alloc;

    impl<B> crate::HeapSize for alloc::borrow::Cow<'_, B>
    where
        B: ?Sized + ToOwned,
        <B as ToOwned>::Owned: crate::HeapSize,
    {
        fn heap_size(&self) -> usize {
            match self {
                alloc::borrow::Cow::Borrowed(_) => 0,
                alloc::borrow::Cow::Owned(b) => b.heap_size(),
            }
        }
    }
}

mod collections {
    use std as alloc;

    impl<T: crate::HeapSize> crate::HeapSize for alloc::collections::VecDeque<T> {
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

    impl<K: crate::HeapSize, V: crate::HeapSize> crate::HeapSize
        for alloc::collections::BTreeMap<K, V>
    {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                let kv = self.iter().next().unwrap();
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * (kv.0.memory_size() + kv.1.memory_size())
            }
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for alloc::collections::BTreeSet<T> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                let t = self.iter().next().unwrap();
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * t.memory_size()
            }
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for alloc::collections::LinkedList<T> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                let t = self.iter().next().unwrap();
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * t.memory_size()
            }
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for alloc::collections::BinaryHeap<T> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * core::mem::size_of::<T>();
            if self.is_empty() {
                cap_bytes
            } else {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self.peek().unwrap().heap_size() + cap_bytes
            }
        }
    }
}

mod rc {
    use std as alloc;

    impl<T: crate::HeapSize> crate::HeapSize for alloc::rc::Rc<T> {
        fn heap_size(&self) -> usize {
            // Rc is a pointer to a Box<T>, so the entire T is in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: ?Sized> crate::HeapSize for alloc::rc::Weak<T> {
        /// Always return 0, because it does not own `T`.
        fn heap_size(&self) -> usize {
            0
        }
    }
}

mod sync {
    use std as alloc;

    impl<T: crate::HeapSize> crate::HeapSize for alloc::sync::Arc<T> {
        fn heap_size(&self) -> usize {
            // Arc is a pointer to a Box<T>, so the entire T is in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for alloc::sync::Arc<[T]> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self.len() * T::memory_size(&self[0])
            }
        }
    }

    impl<T: ?Sized> crate::HeapSize for alloc::sync::Weak<T> {
        /// Always return 0, because it does not own `T`.
        fn heap_size(&self) -> usize {
            0
        }
    }
}
