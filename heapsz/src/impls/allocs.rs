mod vec_box_string {
    use alloc::{boxed::Box, string::String, vec::Vec};
    use std as alloc;

    use crate::HeapSize;

    // Do not impl HeapSize for [T], because type coercions make it error-prone.
    // E.g., Vec[u8] may be casted to &[u8] which does not own any byte in heap.
    impl<T: HeapSize> HeapSize for Vec<T> {
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

    impl<T: HeapSize> HeapSize for Box<T> {
        fn heap_size(&self) -> usize {
            // A Box<T> stores the entire T in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: HeapSize> HeapSize for Box<[T]> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self.len() * T::memory_size(&self[0])
            }
        }
    }

    impl HeapSize for String {
        fn heap_size(&self) -> usize {
            self.capacity()
        }
    }

    #[test]
    fn test_vec() {
        let mut v: Vec<u64> = Vec::with_capacity(100);
        assert_eq!(v.capacity() * core::mem::size_of::<u64>(), v.heap_size());

        v.push(1);
        assert_eq!(v.capacity() * core::mem::size_of::<u64>(), v.heap_size());

        let mut vv: Vec<Vec<u64>> = Vec::with_capacity(100);
        assert_eq!(
            vv.capacity() * core::mem::size_of::<Vec<u64>>(),
            vv.heap_size()
        );

        let v_heap_size = v.heap_size();
        vv.push(v);
        assert_eq!(
            vv.capacity() * core::mem::size_of::<Vec<u64>>() + v_heap_size,
            vv.heap_size()
        );
    }

    #[test]
    fn test_box() {
        let b = Box::new(100u64);
        assert_eq!(core::mem::size_of::<u64>(), b.heap_size());

        let mut v: Vec<u64> = Vec::with_capacity(100);
        v.push(1);
        let v_mem_size = v.memory_size();
        let bv = Box::new(v);
        assert_eq!(v_mem_size, bv.heap_size());

        let bdst: Box<[u64]> = Box::new([0u64; 100]) as _;
        assert_eq!(core::mem::size_of::<u64>() * bdst.len(), bdst.heap_size());
    }

    #[test]
    fn test_string() {
        let s = "abc";
        let mut string = s.to_owned();
        assert_eq!(3, string.heap_size());

        string.reserve(100);
        assert_eq!(string.capacity(), string.heap_size());
    }
}

mod borrow {
    use alloc::borrow::Cow;
    use std as alloc;

    use crate::HeapSize;

    impl<B> HeapSize for Cow<'_, B>
    where
        B: ?Sized + ToOwned,
        <B as ToOwned>::Owned: HeapSize,
    {
        fn heap_size(&self) -> usize {
            match self {
                alloc::borrow::Cow::Borrowed(_) => 0,
                alloc::borrow::Cow::Owned(b) => b.heap_size(),
            }
        }
    }

    #[test]
    fn test_cow() {
        let mut cow = Cow::Borrowed("abc");
        assert_eq!(0, cow.heap_size());
        cow.to_mut();
        assert_eq!(3, cow.heap_size());
    }
}

mod collections {
    use alloc::{
        collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
        mem::size_of,
    };
    use std as alloc;

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for VecDeque<T> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * size_of::<T>();
            if self.is_empty() {
                cap_bytes
            } else {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self[0].heap_size() + cap_bytes
            }
        }
    }

    impl<K: HeapSize, V: HeapSize> HeapSize for BTreeMap<K, V> {
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

    impl<T: HeapSize> HeapSize for BTreeSet<T> {
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

    impl<T: HeapSize> HeapSize for LinkedList<T> {
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

    impl<T: HeapSize> HeapSize for BinaryHeap<T> {
        fn heap_size(&self) -> usize {
            let cap_bytes = self.capacity() * size_of::<T>();
            if self.is_empty() {
                cap_bytes
            } else {
                // Prefer an approximation of its actually heap size, because we
                // want the time complexity to be O(1).
                self.len() * self.peek().unwrap().heap_size() + cap_bytes
            }
        }
    }

    #[test]
    fn test_vec_deque() {
        let mut v: VecDeque<Box<[u8; 8]>> = VecDeque::new();
        assert_eq!(0, v.heap_size());

        v.reserve(100);
        assert_eq!(size_of::<Box<[u8; 8]>>() * v.capacity(), v.heap_size());

        let b = Box::new([0; 8]);
        v.push_back(b.clone());
        assert_eq!(
            size_of::<Box<[u8; 8]>>() * v.capacity() + b.heap_size(),
            v.heap_size()
        );
    }

    #[test]
    fn test_btree() {
        let mut map: BTreeMap<u64, Box<[u8; 8]>> = BTreeMap::new();
        assert_eq!(0, map.heap_size());
        let b = Box::new([0; 8]);
        map.insert(0u64, b.clone());
        assert_eq!(size_of::<u64>() + b.memory_size(), map.heap_size());
        map.insert(1u64, b.clone());
        assert_eq!(2 * (size_of::<u64>() + b.memory_size()), map.heap_size());

        let mut set: BTreeSet<Box<[u8; 8]>> = BTreeSet::new();
        assert_eq!(0, set.heap_size());
        set.insert(b.clone());
        assert_eq!(b.memory_size(), set.heap_size());
    }

    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<Box<[u8; 8]>> = LinkedList::new();
        assert_eq!(0, list.heap_size());
        let b = Box::new([0; 8]);
        list.push_back(b.clone());
        list.push_back(b.clone());
        assert_eq!(2 * b.memory_size(), list.heap_size());
    }

    #[test]
    fn test_binary_heap() {
        let mut heap: BinaryHeap<Box<[u8; 8]>> = BinaryHeap::new();
        assert_eq!(0, heap.heap_size());
        heap.reserve(100);
        assert_eq!(
            size_of::<Box<[u8; 8]>>() * heap.capacity(),
            heap.heap_size()
        );

        let b = Box::new([0u8; 8]);
        heap.push(b.clone());
        assert_eq!(
            size_of::<Box<[u8; 8]>>() * heap.capacity() + b.heap_size(),
            heap.heap_size()
        )
    }
}

mod rc {
    use alloc::rc::{Rc, Weak};
    use std as alloc;

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for Rc<T> {
        fn heap_size(&self) -> usize {
            // Rc is a pointer to a Box<T>, so the entire T is in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: HeapSize> HeapSize for Rc<[T]> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self.len() * T::memory_size(&self[0])
            }
        }
    }

    impl<T: ?Sized> HeapSize for Weak<T> {
        /// Always return 0, because it does not own `T`.
        fn heap_size(&self) -> usize {
            0
        }
    }

    #[test]
    fn test_rc() {
        let b = Box::new([0u8; 8]);
        let rc = Rc::new(b.clone());
        assert_eq!(b.memory_size(), rc.heap_size());

        let weak = Rc::downgrade(&rc);
        assert_eq!(0, weak.heap_size());

        let rc: Rc<[u8]> = Rc::new([0u8; 8]) as _;
        assert_eq!(8, rc.heap_size());
    }
}

mod sync {
    use alloc::sync::{Arc, Weak};
    use std as alloc;

    use crate::HeapSize;

    impl<T: HeapSize> HeapSize for Arc<T> {
        fn heap_size(&self) -> usize {
            // Arc is a pointer to a Box<T>, so the entire T is in heap.
            T::memory_size(self.as_ref())
        }
    }

    impl<T: HeapSize> HeapSize for Arc<[T]> {
        fn heap_size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self.len() * T::memory_size(&self[0])
            }
        }
    }

    impl<T: ?Sized> HeapSize for Weak<T> {
        /// Always return 0, because it does not own `T`.
        fn heap_size(&self) -> usize {
            0
        }
    }

    #[test]
    fn test_arc() {
        let b = Box::new([0u8; 8]);
        let arc = Arc::new(b.clone());
        assert_eq!(b.memory_size(), arc.heap_size());

        let weak = Arc::downgrade(&arc);
        assert_eq!(0, weak.heap_size());

        let rc: Arc<[u8]> = Arc::new([0u8; 8]) as _;
        assert_eq!(8, rc.heap_size());
    }
}
