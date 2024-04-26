macro_rules! test_zero_heap_size {
    (
        $($struct_name: ident,)+
    ) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $struct_name() {
                use heapuse::HeapSize;
                let fs = $struct_name ::default();
                assert_eq!(fs.heap_size(), 0);
            }
        )+
    }
}

macro_rules! decl_struct {
    (
        mod $mod_name: ident {
            $(
                struct $struct_name: ident {
                    $(
                        $field_name:ident : $field_type: ty,
                    )*
                }
            )+
        }
    ) => {
        pub mod $mod_name {
            pub mod full {
                $(
                    #[derive(Default, heapuse_derive::Heap)]
                    pub struct $struct_name {
                        $(
                            #[heap_size]
                            pub $field_name : $field_type,
                        )*
                    }
                )+

                test_zero_heap_size! {
                    $($struct_name,)+
                }

            }

            pub mod none {
                $(
                    #[derive(Default, heapuse_derive::Heap)]
                    pub struct $struct_name {
                        $(
                            pub $field_name : $field_type,
                        )*
                    }
                )+

                test_zero_heap_size! {
                    $($struct_name,)+
                }
            }

            pub mod all {
                $(
                    #[derive(Default, heapuse_derive::Heap)]
                    #[heap_size]
                    pub struct $struct_name {
                        $(
                            pub $field_name : $field_type,
                        )*
                    }
                )+

                test_zero_heap_size! {
                    $($struct_name,)+
                }
            }

            $(
                pub use full::$struct_name;
            )+
        }
    }
}

decl_struct! {
    mod primitive {
        struct StructBool {
            field_bool: bool,
        }

        struct StructPrimitives {
            field_bool: bool,
            field_u8: u8,
            field_i8: i8,
            field_u16: u16,
            field_i16: i16,
            field_u32: u32,
            field_i32: i32,
            field_u64: u64,
            field_i64: i64,
            field_u128: u128,
            field_i128: i128,
            field_usize: usize,
            field_isize: isize,
            field_f32: f32,
            field_f64: f64,
        }
    }
}

decl_struct! {
    mod container {
        struct StructHeapMap {
            field_map: std::collections::HashMap<usize, crate::primitive::StructBool>,
        }

        struct StructVec {
            field_vec: Vec<crate::primitive::StructPrimitives>,
        }

        struct StructContainers {
            field_vec: Vec<crate::primitive::StructPrimitives>,
            field_map: std::collections::HashMap<usize, crate::primitive::StructBool>,
        }
    }
}

mod unit_tuple {
    #[derive(Default, heapuse_derive::Heap)]
    struct StructUnit;

    #[derive(Default, heapuse_derive::Heap)]
    struct StructTuple1(
        crate::primitive::StructBool,
        crate::primitive::StructPrimitives,
    );

    #[derive(Default, heapuse_derive::Heap)]
    #[heap_size]
    struct StructTuple2(
        crate::primitive::StructBool,
        crate::primitive::StructPrimitives,
    );

    #[cfg(test)]
    mod tests {
        use super::*;

        test_zero_heap_size! {
            StructUnit, StructTuple1, StructTuple2,
        }
    }
}

pub mod with_attr {
    use self::remote::Inner;

    pub mod remote {
        #[derive(Default)]
        pub struct Inner(pub Box<u8>);
    }

    pub mod remote_inner_heap_size {
        use heapuse::HeapSize;

        use super::remote;

        pub fn heap_size(t: &remote::Inner) -> usize {
            t.0.heap_size()
        }
    }

    #[derive(Default, heapuse_derive::Heap)]
    pub struct Wtih {
        #[heap_size(with = "remote_inner_heap_size")]
        pub inner: Inner,
    }
}
