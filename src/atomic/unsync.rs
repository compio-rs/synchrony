use std::{cell::Cell, fmt::Debug, sync::atomic::Ordering};

atomic_int!(AtomicU8(u8));
atomic_int!(AtomicU16(u16));
atomic_int!(AtomicU32(u32));
atomic_int!(AtomicU64(u64));
atomic_int!(AtomicUsize(usize));
atomic_int!(AtomicI8(i8));
atomic_int!(AtomicI16(i16));
atomic_int!(AtomicI32(i32));
atomic_int!(AtomicI64(i64));
atomic_int!(AtomicIsize(isize));

/// A singlethreaded [`AtomicBool`] based on [`Cell`](std::cell::Cell)
///
/// All [`Ordering`] passed into the functions are ignored since no actual
/// atomicity is needed.
///
/// [`AtomicBool`]: std::sync::atomic::AtomicBool
pub struct AtomicBool {
    v: Cell<bool>,
}

impl From<bool> for AtomicBool {
    fn from(val: bool) -> Self {
        Self::new(val)
    }
}

impl Default for AtomicBool {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Debug for AtomicBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.v.get(), f)
    }
}

impl AtomicBool {
    /// Creates a new [`AtomicBool`]
    pub const fn new(val: bool) -> Self {
        Self { v: Cell::new(val) }
    }

    /// Returns a mutable reference to the underlying boolean.
    pub fn get_mut(&mut self) -> &mut bool {
        self.v.get_mut()
    }

    /// Load the current value.
    pub fn load(&self, _: Ordering) -> bool {
        self.v.get()
    }

    /// Store a value.
    pub fn store(&self, val: bool, _: Ordering) {
        self.v.set(val)
    }

    /// Stores a value into the atomic boolean, returning the previous value.
    pub fn swap(&self, val: bool, _: Ordering) -> bool {
        self.v.replace(val)
    }

    /// Stores a value into the atomic boolean if the current value is the same
    /// as the `current` value.
    ///
    /// Returns `Ok(old)` if the exchange was successful, or
    /// `Err(old)` otherwise.
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        _: Ordering,
        _: Ordering,
    ) -> Result<bool, bool> {
        let old = self.v.get();
        if old == current {
            self.v.set(new);
            Ok(old)
        } else {
            Err(old)
        }
    }

    /// Stores a value into the atomic boolean if the current value is the same
    /// as the `current` value.
    ///
    /// Returns `Ok(old)` if the exchange was successful, or `Err(old)`
    /// otherwise.
    ///
    /// This is identical to `compare_exchange` in this single-threaded
    /// implementation.
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Bitwise "and" with the current value.
    ///
    /// Performs a bitwise "and" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    pub fn fetch_and(&self, val: bool, _: Ordering) -> bool {
        let old = self.v.get();
        self.v.set(old & val);
        old
    }

    /// Bitwise "nand" with the current value.
    ///
    /// Performs a bitwise "nand" operation on the current value and the
    /// argument `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    pub fn fetch_nand(&self, val: bool, _: Ordering) -> bool {
        let old = self.v.get();
        self.v.set(!(old & val));
        old
    }

    /// Bitwise "not" with the current value.
    ///
    /// Performs a bitwise "not" operation on the current value and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    pub fn fetch_not(&self, _: Ordering) -> bool {
        let old = self.v.get();
        self.v.set(!old);
        old
    }

    /// Bitwise "or" with the current value.
    ///
    /// Performs a bitwise "or" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    pub fn fetch_or(&self, val: bool, _: Ordering) -> bool {
        let old = self.v.get();
        self.v.set(old | val);
        old
    }

    /// Bitwise "xor" with the current value.
    ///
    /// Performs a bitwise "xor" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    pub fn fetch_xor(&self, val: bool, _: Ordering) -> bool {
        let old = self.v.get();
        self.v.set(old ^ val);
        old
    }
}

macro_rules! atomic_int {
    ($t:ident($i:ty)) => {
        #[doc = concat!("A singlethreaded [`", stringify!($t), "`] based on [`Cell`](std::cell::Cell)\n\n")]
        /// All [`Ordering`] passed into the functions are ignored since no actual
        /// atomicity is needed.
        #[doc = concat!("\n\n[`", stringify!($t), "`]: std::sync::atomic::", stringify!($t))]
        #[repr(transparent)]
        pub struct $t {
            v: Cell<$i>,
        }

        impl From<$i> for $t {
            fn from(val: $i) -> Self {
                Self::new(val)
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self::new(0)
            }
        }

        impl Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.v.get(), f)
            }
        }

        impl $t {
            #[doc = concat!("Creates a new [`", stringify!($t), "`]")]
            pub const fn new(val: $i) -> Self {
                Self { v: Cell::new(val) }
            }

            /// Returns a mutable reference to the underlying integer.
            pub fn get_mut(&mut self) -> &mut $i {
                self.v.get_mut()
            }

            /// Load the current value.
            pub fn load(&self, _: Ordering) -> $i {
                self.v.get()
            }

            /// Store a value.
            pub fn store(&self, val: $i, _: Ordering) {
                self.v.set(val)
            }

            /// Stores a value into the atomic integer, returning the previous value.
            pub fn swap(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(val)
            }

            /// Stores a value into the atomic integer if the current value is the same
            /// as the `current` value.
            pub fn compare_exchange(
                &self,
                current: $i,
                new: $i,
                _: Ordering,
                _: Ordering,
            ) -> Result<$i, $i> {
                let old = self.v.get();
                if old == current {
                    self.v.set(new);
                    Ok(old)
                } else {
                    Err(old)
                }
            }

            /// Stores a value into the atomic integer if the current value is the same
            /// as the `current` value.
            ///
            /// This is identical to `compare_exchange` in this single-threaded
            /// implementation.
            pub fn compare_exchange_weak(
                &self,
                current: $i,
                new: $i,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$i, $i> {
                self.compare_exchange(current, new, success, failure)
            }

            /// Adds to the current value, returning the previous value.
            pub fn fetch_add(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get() + val)
            }

            /// Subtract to the current value, returning the previous value.
            pub fn fetch_sub(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get() - val)
            }

            /// Bitwise "and" with the current value.
            ///
            /// Performs a bitwise "and" operation on the current value and the argument
            /// `val`, and sets the new value to the result.
            ///
            /// Returns the previous value.
            pub fn fetch_and(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get() & val)
            }

            /// Bitwise "nand" with the current value.
            ///
            /// Performs a bitwise "nand" operation on the current value and the
            /// argument `val`, and sets the new value to the result.
            ///
            /// Returns the previous value.
            pub fn fetch_nand(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(!(self.v.get() & val))
            }

            /// Bitwise "or" with the current value.
            ///
            /// Performs a bitwise "or" operation on the current value and the argument
            /// `val`, and sets the new value to the result.
            ///
            /// Returns the previous value.
            pub fn fetch_or(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get() | val)
            }

            /// Bitwise "xor" with the current value.
            ///
            /// Performs a bitwise "xor" operation on the current value and the argument
            /// `val`, and sets the new value to the result.
            ///
            /// Returns the previous value.
            pub fn fetch_xor(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get() ^ val)
            }

            /// Maximum with the current value.
            ///
            /// Finds the maximum of the current value and the argument `val`, and
            /// sets the new value to the result.
            pub fn fetch_max(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get().max(val))
            }

            /// Minimum with the current value.
            ///
            /// Finds the minimum of the current value and the argument `val`, and
            /// sets the new value to the result.
            pub fn fetch_min(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get().min(val))
            }
        }
    };
}

use atomic_int;
