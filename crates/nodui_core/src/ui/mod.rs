//! Defines how elements should be rendered in the ui editor.

mod color;
mod node;
mod socket;
mod text;

pub use color::Color;
pub use node::*;
pub use socket::*;
pub use text::TextUi;

/* -------------------------------------------------------------------------- */

/// Padding space on the four sides of an element.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Padding {
    #[allow(missing_docs)]
    pub left: f32,
    #[allow(missing_docs)]
    pub right: f32,
    #[allow(missing_docs)]
    pub top: f32,
    #[allow(missing_docs)]
    pub bottom: f32,
}

impl Padding {
    #[allow(missing_docs)]
    pub const ZERO: Self = Self {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    };

    /// Apply the same space on the four sides.
    #[inline]
    #[must_use]
    pub const fn same(margin: f32) -> Self {
        Self {
            left: margin,
            right: margin,
            top: margin,
            bottom: margin,
        }
    }

    /// Paddings with the same size on opposing sides.
    #[inline]
    #[must_use]
    pub const fn symmetric(x: f32, y: f32) -> Self {
        Self {
            left: x,
            right: x,
            top: y,
            bottom: y,
        }
    }
}

/* -------------------------------------------------------------------------- */

/// Describes the width and color of a line.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Stroke {
    /// The width of the line.
    pub width: f32,
    /// The color of the line.
    pub color: Color,
}

impl Stroke {
    /// No stroke.
    pub const NONE: Self = Self {
        width: 0.0,
        color: Color::TRANSPARENT,
    };

    /// Creates a [`Stroke`].
    #[inline]
    pub fn new(width: impl Into<f32>, color: impl Into<Color>) -> Self {
        Self {
            width: width.into(),
            color: color.into(),
        }
    }
}

impl Default for Stroke {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl<C> From<(f32, C)> for Stroke
where
    C: Into<Color>,
{
    #[inline]
    fn from((width, color): (f32, C)) -> Self {
        Self::new(width, color)
    }
}

/* -------------------------------------------------------------------------- */
