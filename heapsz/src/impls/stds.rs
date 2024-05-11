mod hash {
    impl_heap_size! {zero std::hash::RandomState,}
}

mod collections {
    use std::{collections, mem};

    use crate::HeapSize;

    impl<K: HeapSize, V: HeapSize, S> HeapSize for collections::HashMap<K, V, S> {
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

    impl<T: HeapSize, S> HeapSize for collections::HashSet<T, S> {
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

    #[test]
    fn test_hash_map() {
        let mut map = collections::HashMap::with_capacity(10);
        map.insert(0u64, Box::new([0u64; 32]));
        assert_eq!(
            (map.capacity() * (mem::size_of::<u64>() + mem::size_of::<Box<[u64; 32]>>()))
                + mem::size_of::<[u64; 32]>(),
            map.heap_size()
        );
    }

    #[test]
    fn test_hash_set() {
        let mut set = collections::HashSet::with_capacity(10);
        set.insert(Box::new([0u64; 32]));
        assert_eq!(
            (set.capacity() * mem::size_of::<Box<[u64; 32]>>()) + mem::size_of::<[u64; 32]>(),
            set.heap_size()
        );
    }
}

mod io {
    use crate::HeapSize;

    impl<T: ?Sized> HeapSize for std::io::BufReader<T> {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }

    impl<T: ?Sized + std::io::Write> HeapSize for std::io::BufWriter<T> {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }

    #[test]
    fn test_buf_reader() {
        let s = std::io::BufReader::new(std::io::empty());
        assert_eq!(s.capacity(), s.heap_size());
        let s = std::io::BufReader::with_capacity(10, std::io::empty());
        assert_eq!(s.capacity(), s.heap_size());
    }

    #[test]
    fn test_buf_writer() {
        let s = std::io::BufWriter::new(std::io::empty());
        assert_eq!(s.capacity(), s.heap_size());
        let s = std::io::BufWriter::with_capacity(10, std::io::empty());
        assert_eq!(s.capacity(), s.heap_size());
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
    use crate::HeapSize;

    impl_heap_size! {
        zero
        std::path::Path,
    }

    impl HeapSize for std::path::PathBuf {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }

    #[test]
    fn test() {
        let s = std::path::PathBuf::new();
        assert_eq!(s.capacity(), s.heap_size());
        let s = std::path::PathBuf::with_capacity(10);
        assert_eq!(s.capacity(), s.heap_size());
    }
}

mod sync {
    use std::sync::{Mutex, RwLock};

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for Mutex<T> {
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

    impl<T: HeapSize> HeapSize for RwLock<T> {
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

    #[test]
    fn test_mutex() {
        let s = Mutex::new([0u64; 32]);
        assert_eq!([0u64; 32].memory_size(), s.heap_size());

        let s = Mutex::new(Box::new([0u64; 32]));
        assert_eq!(HeapSize::memory_size(&Box::new([0u64; 32])), s.heap_size());
    }

    #[test]
    fn test_rwlock() {
        let s = RwLock::new([0u64; 32]);
        assert_eq!([0u64; 32].memory_size(), s.heap_size());

        let s = RwLock::new(Box::new([0u64; 32]));
        assert_eq!(HeapSize::memory_size(&Box::new([0u64; 32])), s.heap_size());
    }
}

mod time {
    impl_heap_size! {
        zero
        std::time::Instant,
        std::time::SystemTime,
    }

    #[test]
    fn test() {
        use crate::HeapSize;
        let s = std::time::Instant::now();
        assert_eq!(0, s.heap_size());
        let s = std::time::Duration::default();
        assert_eq!(0, s.heap_size());
    }
}

#[test]
fn test_heap_size() {
    use std::collections::HashMap;

    use crate::HeapSize;

    let mut vu8 = Vec::with_capacity(16);
    assert_eq!(vu8.heap_size(), 16);
    vu8.extend([1u8, 2, 3]);
    assert_eq!(vu8.heap_size(), 16);

    let ovu8 = Some(vu8);
    assert_eq!(ovu8.heap_size(), 16);

    let ovu82 = (ovu8, Some(Vec::<u8>::with_capacity(16)));
    assert_eq!(ovu82.heap_size(), 16 * 2);

    let mut mu8u64 = HashMap::<u8, u64>::default();
    mu8u64.reserve(16);
    assert_eq!(mu8u64.heap_size(), mu8u64.capacity() * (1 + 8));

    let mut mu8vu64 = HashMap::<u8, Vec<u64>>::default();
    mu8vu64.reserve(16);
    mu8vu64.insert(1, Vec::with_capacity(2));
    mu8vu64.insert(2, Vec::with_capacity(2));
    assert_eq!(
        mu8vu64.heap_size(),
        mu8vu64.capacity() * (1 + std::mem::size_of::<Vec<u64>>())
            + 2 * (Vec::<u64>::with_capacity(2).heap_size())
    );
}
