/// Multithreaded notifier
pub mod sync {
    super::impl_notify!(sync);

    impl crate::AssertMt for AsyncFlag {}
    impl crate::AssertMt for AsyncFlagHandle {}
}

/// Singlethreaded notifier
pub mod unsync {
    super::impl_notify!(unsync);
}

macro_rules! impl_notify {
    ($sync:ident) => {
        use std::{
            pin::Pin,
            task::{Context, Poll},
        };

        use crate::$sync::{flag::Flag, shared::Shared, waker_slot::WakerSlot};

        #[derive(Debug)]
        struct Inner {
            waker: WakerSlot,
            set: Flag,
        }

        #[derive(Debug, Clone)]
        struct AsyncFlagImpl(Shared<Inner>);

        impl AsyncFlagImpl {
            pub fn new() -> Self {
                Self(Shared::new(Inner {
                    waker: WakerSlot::new(),
                    set: Flag::new(false),
                }))
            }

            pub fn notify(&self) {
                self.0.set.swap(true);
                self.0.waker.wake();
            }

            pub fn notified(&self) -> bool {
                self.0.set.get()
            }
        }

        impl Future for AsyncFlagImpl {
            type Output = ();

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                // quick check to avoid registration if already done.
                if self.0.set.get() {
                    return Poll::Ready(());
                }

                self.0.waker.register(cx.waker());

                // Need to check condition **after** `register` to avoid a race
                // condition that would result in lost notifications.
                if self.0.set.get() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }

        /// An event that won't wake until [`AsyncFlagHandle::notify`] is called
        /// successfully.
        #[derive(Debug)]
        pub struct AsyncFlag {
            flag: AsyncFlagImpl,
        }

        impl Default for AsyncFlag {
            fn default() -> Self {
                Self::new()
            }
        }

        impl AsyncFlag {
            /// Create [`AsyncFlag`].
            pub fn new() -> Self {
                Self {
                    flag: AsyncFlagImpl::new(),
                }
            }

            /// Get a handle to notify the flag.
            pub fn handle(&self) -> AsyncFlagHandle {
                AsyncFlagHandle::new(self.flag.clone())
            }

            /// Returns whether the event has been notified.
            pub fn notified(&self) -> bool {
                self.flag.notified()
            }

            /// Wait for [`AsyncFlagHandle::notify`] to be called.
            pub async fn wait(self) {
                self.flag.await
            }
        }

        /// A wake up handle to [`AsyncFlag`].
        pub struct AsyncFlagHandle {
            flag: AsyncFlagImpl,
        }

        impl AsyncFlagHandle {
            fn new(flag: AsyncFlagImpl) -> Self {
                Self { flag }
            }

            /// Notify the event.
            pub fn notify(self) {
                self.flag.notify()
            }
        }
    };
}

use impl_notify;
