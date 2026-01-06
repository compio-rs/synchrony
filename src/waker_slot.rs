//! A slot holds up to one waker for task wakeup.
//!
//! `sync` version is just [`futures_util::task::AtomicWaker`]; unsync version is a
//! hand-rolled singlethreaded version with similar API.

/// Multithreaded `WakerSlot` based on [`futures_util::task::AtomicWaker`].
pub mod sync {
    pub use futures_util::task::AtomicWaker as WakerSlot;

    impl crate::AssertMt for WakerSlot {}
}

/// Singlethreaded `WakerSlot`
pub mod unsync {
    use std::{cell::RefCell, task::Waker};

    /// A singlethreaded registry holds up to one waker for task wakeup.
    #[derive(Debug, Default)]
    pub struct WakerSlot {
        waker: RefCell<Option<Waker>>,
    }

    impl WakerSlot {
        /// Create a new [`WakerSlot`]
        pub const fn new() -> Self {
            Self {
                waker: RefCell::new(None),
            }
        }

        /// Register given waker
        pub fn register(&self, waker: &Waker) {
            let mut w = self.waker.borrow_mut();
            // Avoid unnecessary clone if two wakers point to the same task
            if w.as_ref().is_some_and(|x| x.will_wake(waker)) {
                return;
            }
            *w = Some(waker.clone())
        }

        /// Try to take the stored waker
        pub fn take(&self) -> Option<Waker> {
            self.waker.borrow_mut().take()
        }

        /// Wake currently stored waker
        pub fn wake(&self) {
            if let Some(waker) = self.take() {
                waker.wake()
            }
        }
    }
}
