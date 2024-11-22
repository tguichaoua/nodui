//! Abstraction to collect items.

use std::marker::PhantomData;

/* -------------------------------------------------------------------------- */

/// An abstraction for element that can collect item of type `T`.
pub(crate) trait Collect<T> {
    /// Reserves capacity for at least `additional` more elements to be collected.
    fn reserve(&mut self, additional: usize);
    /// Collect one item.
    fn collect(&mut self, item: T);
}

/* -------------------------------------------------------------------------- */

/// An implementation of [`Collect<T>`] that do nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NoCollect<T>(PhantomData<T>);

impl<T> Default for NoCollect<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Collect<T> for NoCollect<T> {
    fn reserve(&mut self, _: usize) {}
    fn collect(&mut self, _: T) {}
}

/* -------------------------------------------------------------------------- */

impl<T> Collect<T> for Vec<T> {
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn collect(&mut self, item: T) {
        self.push(item);
    }
}

/* -------------------------------------------------------------------------- */
