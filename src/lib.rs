use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock, Weak};

//
// Atomic Reference Box
//

/// A thread-safe pointer type that uniquely owns a heap allocation of type T,
/// but allows immutable (weak) references to be created and shared.
///
/// CRITICAL:
/// The current implementation is flawed. See Deref implementation for reason.
pub struct AtomicRefBox<T> {
    inner: Arc<Inner<T>>,
}

impl<T> AtomicRefBox<T> {
    pub fn new(value: T) -> AtomicRefBox<T> {
        AtomicRefBox {
            inner: Arc::new(Inner {
                lock: RwLock::new(Some(value)),
            }),
        }
    }

    /// Immutably borrows the inner value.
    pub fn borrow(this: &Self) -> AtomicRef<T> {
        todo!()
    }

    /// Mutably borrows the inner value.
    pub fn borrow_mut(this: &Self) -> AtomicMutRef<T> {
        // Similar problem as directly deref-ing + extra issues:
        // When a mutable reference is produced, the weak references
        // should not be able to borrow until the mutable reference is dropped
        // This logically involves somehow holding onto the write lock until
        // the mutable reference is dropped (how to detect this?)
        todo!()
    }

    pub fn to_weak(this: &Self) -> AtomicWeakRef<T> {
        AtomicWeakRef::new(Arc::downgrade(&this.inner))
    }

    /// Extracts the inner value from the box.
    ///
    /// This function may block until all weak references
    /// have finished actively borrowing the value.
    pub fn into_inner(this: Self) -> T {
        this.inner.lock.write().unwrap().take().unwrap()
    }
}

impl<T> Deref for AtomicRefBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // When using a RwLock, this method cannot be implemented
        // without acquiring a read lock.
        // But when the inner value is returned, the lock is released
        // and a lifetime error occurs
        todo!()
    }
}

impl<T> AsRef<T> for AtomicRefBox<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

struct Inner<T> {
    lock: RwLock<Option<T>>,
}

//
// Atomic Immutable Reference
//

pub struct AtomicRef<'a, T> {
    marker: PhantomData<&'a mut T>, // TODO temp
}

impl<'a, T> Deref for AtomicRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

//
// Atomic Mutable Reference
//

pub struct AtomicMutRef<'a, T> {
    marker: PhantomData<&'a mut T>, // TODO temp
}

impl<'a, T> Deref for AtomicMutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<'a, T> DerefMut for AtomicMutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

//
// Atomic Weak Reference
//

pub struct AtomicWeakRef<T> {
    weak: Weak<Inner<T>>,
}

impl<T> AtomicWeakRef<T> {
    fn new(weak: Weak<Inner<T>>) -> AtomicWeakRef<T> {
        AtomicWeakRef { weak }
    }

    pub fn borrow<R>(&self, op: impl FnOnce(&T) -> R) -> Option<R> {
        let Some(w) = self.weak.upgrade() else {
            return None;
        };
        let read_binding = w.lock.read().unwrap();
        let Some(t) = read_binding.deref() else {
            return None;
        };
        Some(op(t))
    }
}
