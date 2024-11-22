//! Types to provided size hint.

use std::ops::{Add, AddAssign};

/* -------------------------------------------------------------------------- */

/// The bounds on the length of a collection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeHint {
    /// The lower limit.
    pub min: usize,

    /// The upper limit.
    ///
    /// A [`None`] here means that either there is no known upper bound,
    /// or the upper bound is larger than [`usize`].
    pub max: Option<usize>,
}

impl SizeHint {
    /// The size hint of a [slice](core::slice) or an [`Iterator`].
    #[must_use]
    #[inline]
    pub fn of<T>(x: &T) -> Self
    where
        T: SizeHintOf + ?Sized,
    {
        SizeHintOf::size_hint(x)
    }

    /// A size hint that indicates an exact `length`.
    #[must_use]
    #[inline]
    pub fn exact(length: usize) -> Self {
        Self {
            min: length,
            max: Some(length),
        }
    }

    /// A size hint that indicates a minimum length of `count`.
    #[must_use]
    #[inline]
    pub fn at_least(count: usize) -> Self {
        Self {
            min: count,
            max: None,
        }
    }

    /// A size hint that indicates a maximum length of `count`.
    #[must_use]
    #[inline]
    pub fn at_most(count: usize) -> Self {
        Self {
            min: 0,
            max: Some(count),
        }
    }
}

/* -------------------------------------------------------------------------- */

impl Add for SizeHint {
    type Output = SizeHint;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        SizeHint {
            min: self.min.saturating_add(rhs.min),
            max: match (self.max, rhs.max) {
                (Some(x), Some(y)) => x.checked_add(y),
                _ => None,
            },
        }
    }
}

impl AddAssign for SizeHint {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/* -------------------------------------------------------------------------- */

/// Implementation for [`SizeHint::of`].
pub trait SizeHintOf {
    /// Returns the [`SizeHint`] for [`Self`]
    fn size_hint(&self) -> SizeHint;
}

impl<T> SizeHintOf for [T] {
    #[inline]
    fn size_hint(&self) -> SizeHint {
        SizeHint::exact(self.len())
    }
}

impl<I> SizeHintOf for I
where
    I: Iterator,
{
    #[inline]
    fn size_hint(&self) -> SizeHint {
        let (lower, upper) = self.size_hint();
        SizeHint {
            min: lower,
            max: upper,
        }
    }
}

/* -------------------------------------------------------------------------- */
