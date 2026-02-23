/// Multithreaded notifier
pub mod sync {
    super::impl_notify!(sync);

    impl crate::AssertMt for Notify {}
    impl crate::AssertMt for NotifyHandle {}
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
        struct NotifyImpl(Shared<Inner>);

        impl NotifyImpl {
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

        impl Future for NotifyImpl {
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

        /// An event that won't wake until [`NotifyHandle::notify`] is called
        /// successfully.
        #[derive(Debug)]
        pub struct Notify {
            flag: NotifyImpl,
        }

        impl Default for Notify {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Notify {
            /// Create [`Notify`].
            pub fn new() -> Self {
                Self {
                    flag: NotifyImpl::new(),
                }
            }

            /// Get a notify handle.
            pub fn handle(&self) -> NotifyHandle {
                NotifyHandle::new(self.flag.clone())
            }

            /// Returns whether the event has been notified.
            pub fn notified(&self) -> bool {
                self.flag.notified()
            }

            /// Wait for [`NotifyHandle::notify`] to be called.
            pub async fn wait(self) {
                self.flag.await
            }
        }

        /// A wake up handle to [`Notify`].
        pub struct NotifyHandle {
            flag: NotifyImpl,
        }

        impl NotifyHandle {
            fn new(flag: NotifyImpl) -> Self {
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
