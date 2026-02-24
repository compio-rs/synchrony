# Synchrony

A library that provides both sync and unsync versions of common synchronization primitives.

## Features

All of the following primitives are provided in both sync and unsync versions:

- Shared (`Rc`/`Arc`)
- Atomic Scalars
- Watch 
- Waker Slot (`AtomicWaker` and its unsync counterpart)
- Mutex
- Async Mutex
- BiLock
- Flag (specialized `AtomicBool`)
- Event (`event-listener` and `local-event`)
- Async Flag
