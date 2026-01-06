//! A library that provides both sync and unsync versions of common
//! synchronization primitives.
//!
//! # Example
//!
//! If you're a library author, a common pattern is to provide a feature gate
//! that let users to choose whether they want multithread or not:
//!
//! ```toml
//! # cargo.toml
//! [dependencies]
//! synchrony = { version = "0.1.0", feature = ["mutex"] }
//!
//! [features]
//! sync_foo = []
//! ```
//!
//! and in your code:
//!
//! ```ignore
//! #[cfg(feature = "sync_foo")]
//! use synchrony::sync;
//! #[cfg(not(feature = "sync_foo"))]
//! use synchrony::unsync as sync;
//!
//! struct Foo {
//!     lock: sync::mutex::Mutex,
//!     count: sync::atomic::AtomicUsize,
//! }
//! ```
//!
//! Or you can also hand-pick sync/unsync primitives:
//!
//! ```ignore
//! use synchrony::*;
//!
//! let unsync_lock = unsync::bilock::BiLock::new(42);
//! let sync_counter = sync::atomic::AtomicUsize::new(42);
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "bilock")]
mod bilock;
#[cfg(feature = "event")]
mod event;
#[cfg(feature = "mutex")]
mod mutex;
#[cfg(feature = "waker_slot")]
mod waker_slot;

mod atomic;
mod flag;
mod mutex_blocking;
mod shared;

/// Multithreaded version of primitives
pub mod sync {
    /// Multithreaded `Watch` channel based on [`see`].
    #[doc(inline)]
    #[cfg(feature = "watch")]
    pub use see::sync as watch;

    #[doc(inline)]
    #[cfg(feature = "bilock")]
    pub use crate::bilock::sync as bilock;
    #[doc(inline)]
    #[cfg(feature = "event")]
    pub use crate::event::sync as event;
    #[doc(inline)]
    #[cfg(feature = "mutex")]
    pub use crate::mutex::sync as mutex;
    #[doc(inline)]
    #[cfg(feature = "waker_slot")]
    pub use crate::waker_slot::sync as waker_slot;
    #[doc(inline)]
    pub use crate::{
        atomic::sync as atomic, flag::sync as flag, mutex_blocking::sync as mutex_blocking,
        shared::sync as shared,
    };
}

/// Singlethreaded version of primitives
pub mod unsync {
    /// Singlethreaded `Watch` channel based on [`see`].
    #[doc(inline)]
    #[cfg(feature = "watch")]
    pub use see::unsync as watch;

    #[doc(inline)]
    #[cfg(feature = "bilock")]
    pub use crate::bilock::unsync as bilock;
    #[doc(inline)]
    #[cfg(feature = "event")]
    pub use crate::event::unsync as event;
    #[doc(inline)]
    #[cfg(feature = "mutex")]
    pub use crate::mutex::unsync as mutex;
    #[doc(inline)]
    #[cfg(feature = "waker_slot")]
    pub use crate::waker_slot::unsync as waker_slot;
    #[doc(inline)]
    pub use crate::{
        atomic::unsync as atomic, flag::unsync as flag, mutex_blocking::unsync as mutex_blocking,
        shared::unsync as shared,
    };
}

/// A trait to assert that a type is `Send + Sync`.
#[allow(dead_code)]
trait AssertMt: Send + Sync {}
