//! Blocking Mutex lock

/// Multithreaded blocking Mutex
pub mod sync {
    use std::{
        fmt,
        ops::{Deref, DerefMut},
        sync::{Mutex as Inner, MutexGuard as InnerGuard},
    };

    /// A multithreaded Mutex based on [`std::sync::Mutex`].
    pub struct Mutex<T: ?Sized>(Inner<T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<T> Mutex<T> {
        /// Creates a new mutex in an unlocked state ready for use.
        pub const fn new(val: T) -> Self {
            Self(Inner::new(val))
        }

        /// Get the inner [`std::sync::Mutex`].
        pub fn into_inner(self) -> Inner<T> {
            self.0
        }
    }

    impl<T: ?Sized> Mutex<T> {
        /// Acquires a mutex, blocking the current thread until it is able to do
        /// so.
        ///
        /// See [`std::sync::Mutex::lock`] for detail.
        ///
        /// # Panics
        ///
        /// This function might panic when called if the lock is already held by
        /// the current thread or is poisoned (some thread panicked while
        /// holding the lock).
        pub fn lock(&self) -> MutexGuard<'_, T> {
            MutexGuard(self.0.lock().unwrap())
        }
    }

    /// An RAII implementation of a "scoped lock" of a mutex. When this
    /// structure is dropped (falls out of scope), the lock will be
    /// unlocked.
    pub struct MutexGuard<'a, T: ?Sized>(InnerGuard<'a, T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<'a, T: ?Sized> MutexGuard<'a, T> {
        /// Get the inner [`std::sync::MutexGuard`].
        pub fn into_inner(self) -> InnerGuard<'a, T> {
            self.0
        }
    }

    impl<'a, T> Deref for MutexGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a, T> DerefMut for MutexGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T: Send> crate::AssertMt for Mutex<T> {}
}

/// Singlethreaded blocking Mutex
pub mod unsync {
    use std::{
        cell::{RefCell as Inner, RefMut as InnerGuard},
        fmt,
        ops::{Deref, DerefMut},
    };

    /// A singlethreaded Mutex based on [`std::cell::RefCell`].
    pub struct Mutex<T: ?Sized>(Inner<T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<T> Mutex<T> {
        /// Creates a new mutex in an unlocked state ready for use.
        pub const fn new(val: T) -> Self {
            Self(Inner::new(val))
        }

        /// Get the inner [`std::cell::RefCell`].
        pub fn into_inner(self) -> Inner<T> {
            self.0
        }
    }

    impl<T: ?Sized> Mutex<T> {
        /// Acquires a mutex.
        ///
        /// See [`std::cell::RefCell::borrow_mut`] for detail.
        ///
        /// # Panics
        ///
        /// Panics if the value is currently borrowed.
        pub fn lock(&self) -> MutexGuard<'_, T> {
            MutexGuard(self.0.borrow_mut())
        }
    }

    /// An RAII implementation of a "scoped lock" of a mutex. When this
    /// structure is dropped (falls out of scope), the lock will be
    /// unlocked.
    pub struct MutexGuard<'a, T: ?Sized>(InnerGuard<'a, T>);

    impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<'a, T: ?Sized> MutexGuard<'a, T> {
        /// Get the inner [`std::cell::RefMut`].
        pub fn into_inner(self) -> InnerGuard<'a, T> {
            self.0
        }
    }

    impl<'a, T> Deref for MutexGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a, T> DerefMut for MutexGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
