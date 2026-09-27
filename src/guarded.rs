use std::ops::{Deref, DerefMut};

pub struct Guarded<T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    value: T,
    on_change: F,
}

impl<T, F> Guarded<T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    pub fn new(value: T, on_change: F) -> Self {
        Self { value, on_change }
    }

    /// Borrow mutably. On drop, if the value actually changed,
    /// `on_change` is called exactly once.
    pub fn guard(&mut self) -> Guard<'_, T, F> {
        Guard {
            original: self.value.clone(),
            value: &mut self.value,
            on_change: &mut self.on_change,
        }
    }

    pub fn read(&self) -> &T {
        &self.value
    }

    /// Non-guarded mutation; still fires invalidation on real change.
    pub fn set(&mut self, v: T) {
        if self.value != v {
            self.value = v;
            (self.on_change)();
        }
    }
}

pub struct Guard<'a, T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    original: T,
    value: &'a mut T,
    on_change: &'a mut F,
}

impl<'a, T, F> Drop for Guard<'a, T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    fn drop(&mut self) {
        if *self.value != self.original {
            (self.on_change)();
        }
    }
}

impl<'a, T, F> Deref for Guard<'a, T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    type Target = T;
    fn deref(&self) -> &T { self.value }
}

impl<'a, T, F> DerefMut for Guard<'a, T, F>
where
    T: Clone + PartialEq,
    F: FnMut(),
{
    fn deref_mut(&mut self) -> &mut T { self.value }
}