/// Notify async tasks or threads.
pub mod sync {
    pub use event_listener::{Event, EventListener};

    impl crate::AssertMt for Event {}
    impl crate::AssertMt for EventListener {}
}
#[doc(inline)]
pub use local_event as unsync;
