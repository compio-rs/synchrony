//! Boolean flags

/// Multithreaded boolean flag based on [`std::sync::atomic::AtomicBool`]
pub mod sync {
    super::impl_flag!(sync);

    impl crate::AssertMt for Flag {}
}

/// Singlethreaded boolean flag based on [`std::cell::Cell`]
pub mod unsync {
    super::impl_flag!(unsync);
}

macro_rules! impl_flag {
    ($sync:ident) => {
        use std::sync::atomic::Ordering;

        use crate::$sync::atomic::AtomicBool;

        /// A boolean flag
        pub struct Flag(AtomicBool);

        impl Flag {
            /// Create a new flag
            pub fn new(val: bool) -> Self {
                Flag(AtomicBool::new(val))
            }

            /// Get the current value
            pub fn get(&self) -> bool {
                self.0.load(Ordering::Acquire)
            }

            /// Stores a value into the bool, returning the previous value.
            pub fn swap(&self, val: bool) -> bool {
                self.0.swap(val, Ordering::AcqRel)
            }

            /// Flip the current value and return the new value
            pub fn flip(&self) -> bool {
                let mut current = self.get();
                loop {
                    let new = !current;
                    match self.0.compare_exchange_weak(
                        current,
                        new,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) => return new,
                        Err(previous) => current = previous,
                    }
                }
            }
        }
    };
}

use impl_flag;
