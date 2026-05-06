use std::{fmt::Debug, sync::atomic::Ordering};

crate::cfg_loom! {
    use std::cell::Cell;
}

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
    #[cfg(not(loom))]
    pub const fn new(val: bool) -> Self {
        Self { v: Cell::new(val) }
    }

    /// Creates a new [`AtomicBool`]
    #[cfg(loom)]
    pub fn new(val: bool) -> Self {
        Self { v: Cell::new(val) }
    }

    /// Returns a mutable reference to the underlying boolean.
    #[cfg(not(loom))]
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

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function
    /// returned `Some(_)`, else `Err(previous_value)`.
    pub fn fetch_update<F>(&self, _: Ordering, _: Ordering, mut f: F) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
    {
        let curr = self.v.get();
        if let Some(new) = f(curr) {
            self.v.set(new);
            Ok(curr)
        } else {
            Err(curr)
        }
    }

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function
    /// returned `Some(_)`, else `Err(previous_value)`.
    pub fn try_update(
        &self,
        _: Ordering,
        _: Ordering,
        mut f: impl FnMut(bool) -> Option<bool>,
    ) -> Result<bool, bool> {
        let curr = self.v.get();
        if let Some(new) = f(curr) {
            self.v.set(new);
            Ok(curr)
        } else {
            Err(curr)
        }
    }

    /// Fetches the value, and applies a function to it that returns a new
    /// value. Returns the previous_value.
    pub fn update(&self, _: Ordering, _: Ordering, mut f: impl FnMut(bool) -> bool) -> bool {
        let curr = self.v.get();
        let new = f(curr);
        self.v.set(new);
        curr
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
            #[cfg(not(loom))]
            #[doc = concat!("Creates a new [`", stringify!($t), "`]")]
            pub const fn new(val: $i) -> Self {
                Self { v: Cell::new(val) }
            }

            #[cfg(loom)]
            #[doc = concat!("Creates a new [`", stringify!($t), "`]")]
            pub fn new(val: $i) -> Self {
                Self { v: Cell::new(val) }
            }

            /// Returns a mutable reference to the underlying integer.
            #[cfg(not(loom))]
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


            /// Fetches the value, and applies a function to it that returns an optional
            /// new value. Returns a `Result` of `Ok(previous_value)` if the function
            /// returned `Some(_)`, else `Err(previous_value)`.
            pub fn fetch_update<F>(&self, _: Ordering, _: Ordering, mut f: F) -> Result<$i, $i>
            where
                F: FnMut($i) -> Option<$i>,
            {
                let curr = self.v.get();
                if let Some(new) = f(curr) {
                    self.v.set(new);
                    Ok(curr)
                } else {
                    Err(curr)
                }
            }

            /// Fetches the value, and applies a function to it that returns an optional
            /// new value. Returns a `Result` of `Ok(previous_value)` if the function
            /// returned `Some(_)`, else `Err(previous_value)`.
            pub fn try_update(
                &self,
                _: Ordering,
                _: Ordering,
                mut f: impl FnMut($i) -> Option<$i>,
            ) -> Result<$i, $i> {
                let curr = self.v.get();
                if let Some(new) = f(curr) {
                    self.v.set(new);
                    Ok(curr)
                } else {
                    Err(curr)
                }
            }

            /// Fetches the value, and applies a function to it that returns a new value. Returns the
            /// previous_value.
            pub fn update(&self, _: Ordering, _: Ordering, mut f: impl FnMut($i) -> $i) -> $i {
                let curr = self.v.get();
                let new = f(curr);
                self.v.set(new);
                curr
            }

            /// Adds to the current value, returning the previous value.
            pub fn fetch_add(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get().wrapping_add(val))
            }

            /// Subtract from the current value, returning the previous value.
            pub fn fetch_sub(&self, val: $i, _: Ordering) -> $i {
                self.v.replace(self.v.get().wrapping_sub(val))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering::Relaxed;

    #[test]
    fn atomic_u16_fetch_add_wraps() {
        let a = AtomicU16::new(u16::MAX);
        let old = a.fetch_add(1, Relaxed);
        assert_eq!(old, u16::MAX);
        assert_eq!(a.load(Relaxed), 0);
    }

    #[test]
    fn atomic_u16_fetch_sub_wraps() {
        let a = AtomicU16::new(0);
        let old = a.fetch_sub(1, Relaxed);
        assert_eq!(old, 0);
        assert_eq!(a.load(Relaxed), u16::MAX);
    }

    #[test]
    fn atomic_u8_fetch_add_wraps() {
        let a = AtomicU8::new(u8::MAX);
        assert_eq!(a.fetch_add(1, Relaxed), u8::MAX);
        assert_eq!(a.load(Relaxed), 0);
    }

    #[test]
    fn atomic_i16_fetch_add_wraps() {
        let a = AtomicI16::new(i16::MAX);
        assert_eq!(a.fetch_add(1, Relaxed), i16::MAX);
        assert_eq!(a.load(Relaxed), i16::MIN);
    }
}
