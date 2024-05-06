mod hash {
    impl crate::HeapSize for std::hash::RandomState {
        fn heap_size(&self) -> usize {
            0
        }
    }
}
mod collections {
    use std::{collections, mem};

    impl<K: crate::HeapSize, V: crate::HeapSize, S> crate::HeapSize for collections::HashMap<K, V, S> {
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

    impl<T: crate::HeapSize, S> crate::HeapSize for collections::HashSet<T, S> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * mem::size_of::<T>();
            if self.is_empty() {
                cap_bytes
            } else {
                let k = self.iter().next().unwrap();
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                cap_bytes + self.len() * k.heap_size()
            }
        }
    }
}

mod io {
    impl<T> crate::HeapSize for std::io::BufReader<T> {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }

    impl<T: ?Sized + std::io::Write> crate::HeapSize for std::io::BufWriter<T> {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }
}

mod net {
    impl_heap_size! {
        zero
        std::net::TcpListener,
        std::net::TcpStream,
        std::net::UdpSocket,
        std::net::Shutdown,
    }
}

mod path {
    impl_heap_size! {
        zero
        std::path::Path,
    }

    impl crate::HeapSize for std::path::PathBuf {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }
}

mod sync {
    impl<T: crate::HeapSize> crate::HeapSize for std::sync::Mutex<T> {
        /// Return the number of bytes it owns in heap.
        ///
        /// # Panics
        ///
        /// This function might panic when called if 1) the lock is already held
        /// by the current thread or 2) the lock is
        /// [poisoned](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#errors).
        fn heap_size(&self) -> usize {
            T::memory_size(&*self.lock().unwrap())
        }
    }

    impl<T: crate::HeapSize> crate::HeapSize for std::sync::RwLock<T> {
        /// Return the number of bytes it owns in heap.
        ///
        /// # Panics
        ///
        /// This function might panic when called if 1) the lock is already held
        /// by the current thread or 2) the lock is
        /// [poisoned](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html#errors-1).
        fn heap_size(&self) -> usize {
            T::memory_size(&*self.read().unwrap())
        }
    }
}

mod time {
    impl_heap_size! {
        zero
        std::time::Instant,
        std::time::SystemTime,
    }
}
