//! Color representation.

/// A color in the `sRGBA` space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Color([u8; 4]);

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::TRANSPARENT
    }
}

impl Color {
    /// Creates a [`Color`] from the red, green, blue and alpha components.
    #[inline]
    #[must_use]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    /// Creates a [`Color`] from the red, green and blue components.
    ///
    /// The alpha is set to `255`.
    #[inline]
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }

    /// The red, green, blue and alpha components of the color.
    #[inline]
    #[must_use]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        let Self([r, g, b, a]) = self;
        (r, g, b, a)
    }

    /// Gets a readonly reference of the color as an sRGBA array.
    #[inline]
    #[must_use]
    pub fn as_array(&self) -> &[u8; 4] {
        &self.0
    }

    /// Gets a mutable reference of the color as an sRGBA array.
    #[inline]
    #[must_use]
    pub fn as_array_mut(&mut self) -> &mut [u8; 4] {
        &mut self.0
    }
}

#[allow(missing_docs)]
impl Color {
    pub const TRANSPARENT: Color = Color::from_rgba(0, 0, 0, 0);

    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const GREEN: Color = Color::from_rgb(0, 255, 0);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);
    pub const CYAN: Color = Color::from_rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
    pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::from_rgba(r, g, b, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    #[inline]
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::from_rgb(r, g, b)
    }
}
