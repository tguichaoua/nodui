//! Types to provided size hint.

use std::ops::{Add, AddAssign};

/* -------------------------------------------------------------------------- */

/// The bounds on the length of a collection.
///
/// # Addition
///
/// The addition of two collections' [`SizeHint`] is equivalent to the [`SizeHint`]
/// of an chained iterator created from those two collections.
///
/// ```
/// # use nodui_core::SizeHint;
/// let a = [0, 1, 2];
/// let b = [3, 4];
///
/// let sh_a = SizeHint::of(&a);
/// let sh_b = SizeHint::of(&b);
///
/// let chained = a.iter().chain(b.iter());
///
/// assert_eq!(SizeHint::of_iter(&chained), sh_a + sh_b);
/// ```
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
    /// The size hint of a [slice](core::slice).
    #[must_use]
    #[inline]
    pub fn of<Slice, T>(slice: &Slice) -> Self
    where
        Slice: ?Sized + AsRef<[T]>,
    {
        SizeHint::exact(slice.as_ref().len())
    }

    /// The size hint of an [`Iterator`].
    #[must_use]
    #[inline]
    pub fn of_iter<I>(x: &I) -> Self
    where
        I: ?Sized + Iterator,
    {
        let (lower, upper) = x.size_hint();
        SizeHint {
            min: lower,
            max: upper,
        }
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
