/// Multithreaded BiLock
pub mod sync {
    super::impl_bilock!(sync);

    unsafe impl<T: Send> Send for Inner<T> {}
    unsafe impl<T: Send> Sync for Inner<T> {}

    impl<T: Send> crate::AssertMt for BiLock<T> {}
    impl<T: Send> crate::AssertMt for BiLockAcquire<'_, T> {}
    impl<T: Send> crate::AssertMt for BiLockGuard<'_, T> {}
}

/// Singlethreaded BiLock
pub mod unsync {
    super::impl_bilock!(unsync);
}

macro_rules! impl_bilock {
    ($sync:ident) => {
        use std::{
            cell::UnsafeCell,
            fmt::Debug,
            future::Future,
            ops::{Deref, DerefMut},
            pin::Pin,
            task::{Context, Poll},
        };

        use crate::$sync::{flag::Flag, shared::Shared, waker_slot::WakerSlot};

        /// A lock shared by two parties.
        pub struct BiLock<T>(Shared<Inner<T>>);

        impl<T> Debug for BiLock<T>
        where
            T: Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("BiLock")
                    .field("locked", &self.0.locked.get())
                    .finish()
            }
        }

        impl<T> BiLock<T> {
            /// Creates a new `BiLock` wrapping the supplied data, returning two
            /// handles to it.
            pub fn new(data: T) -> (Self, Self) {
                let inner = Shared::new(Inner {
                    data: UnsafeCell::new(data),
                    waiter: WakerSlot::new(),
                    locked: Flag::new(false),
                });
                (Self(inner.clone()), Self(inner))
            }

            /// Acquires the lock, returning a future that resolves to a guard
            pub fn lock(&self) -> BiLockAcquire<'_, T> {
                BiLockAcquire { inner: &self.0 }
            }

            /// Attempts to join two `BiLock`s into their original data.
            pub fn try_join(self, other: Self) -> Option<T> {
                if Shared::ptr_eq(&self.0, &other.0) {
                    drop(other);
                    let value = Shared::try_unwrap(self.0)
                        .map_err(|_| ())
                        .expect("BiLock is still shared")
                        .data
                        .into_inner();
                    Some(value)
                } else {
                    None
                }
            }

            /// Joins two `BiLock`s into their original data.
            #[allow(unused)]
            pub fn join(self, other: Self) -> T {
                if let Some(value) = self.try_join(other) {
                    value
                } else {
                    #[cold]
                    fn panic_unrelated() -> ! {
                        panic!("Unrelated `BiLock` passed to `BiLock::join`.")
                    }

                    panic_unrelated()
                }
            }
        }

        /// Future for acquiring a [`BiLock`]
        pub struct BiLockAcquire<'a, T> {
            inner: &'a Inner<T>,
        }

        impl<'a, T> Future for BiLockAcquire<'a, T> {
            type Output = BiLockGuard<'a, T>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let this = self.get_mut();
                if this.inner.locked.swap(true) {
                    this.inner.waiter.register(cx.waker());
                    Poll::Pending
                } else {
                    Poll::Ready(BiLockGuard { inner: this.inner })
                }
            }
        }

        struct Inner<T: ?Sized> {
            locked: Flag,
            waiter: WakerSlot,
            data: UnsafeCell<T>,
        }

        /// An RAII guard returned by a successful call to [`BiLock::lock`]
        pub struct BiLockGuard<'a, T: ?Sized> {
            inner: &'a Inner<T>,
        }

        impl<T: ?Sized> Deref for BiLockGuard<'_, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                unsafe { &*self.inner.data.get() }
            }
        }

        impl<T: ?Sized> DerefMut for BiLockGuard<'_, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *self.inner.data.get() }
            }
        }

        impl<T: ?Sized> Drop for BiLockGuard<'_, T> {
            fn drop(&mut self) {
                self.inner.locked.swap(false);
                self.inner.waiter.wake();
            }
        }
    };
}

use impl_bilock;
