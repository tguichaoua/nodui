use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct SizeHint {
    min: usize,
    max: Option<usize>,
}

impl SizeHint {
    #[must_use]
    #[inline]
    pub fn min(&self) -> usize {
        self.min
    }
    #[must_use]
    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.max
    }

    #[must_use]
    #[inline]
    pub fn of<T>(x: &[T]) -> Self {
        Self::exact(x.len())
    }

    #[must_use]
    #[inline]
    pub fn of_iter<I: Iterator>(iter: &I) -> Self {
        let (min, max) = iter.size_hint();
        Self { min, max }
    }

    #[must_use]
    #[inline]
    pub fn exact(count: usize) -> Self {
        Self {
            min: count,
            max: Some(count),
        }
    }

    #[must_use]
    #[inline]
    pub fn at_least(count: usize) -> Self {
        Self {
            min: count,
            max: None,
        }
    }

    #[must_use]
    #[inline]
    pub fn at_most(count: usize) -> Self {
        Self {
            min: 0,
            max: Some(count),
        }
    }
}

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
