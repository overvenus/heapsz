//! Examples of `#[derive(heapuse::Heap)]`.

macro_rules! decl_mod {
    ($(
        mod $name:ident;
    )+) => {
        $(
            decl_mod! {
                origin concat!("../examples/", stringify!($name),".rs"),
                expand concat!("../tests/expand/", stringify!($name),".stdout"),
                mod $name
            }
        )+
    };

    (origin $origin: expr, expand $expand: expr, mod $name: ident) => {
        /// ```ignore
        #[doc = include_str!($origin)]
        /// ```
        ///
        /// <details>
        ///
        /// <summary><b>👉 Expand it!</b></summary>
        ///
        /// ```ignore
        #[doc = include_str!($expand)]
        /// ```
        ///
        /// </details>
        pub mod $name {}
    };
}

decl_mod! {
    mod allowlisting;
    mod blank;
    mod blocklisting;
    mod enums;
    mod tuples;
    mod with;
}
