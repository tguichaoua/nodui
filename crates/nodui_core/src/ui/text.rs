//! UI representation of a text.

use super::Color;

/* -------------------------------------------------------------------------- */

/// Defines how a text should be rendered.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TextUi {
    /// The text to render.
    pub text: String,

    /// The color of the text.
    ///
    /// If not set, use the default color.
    pub color: Option<Color>,
}

impl TextUi {
    /// Creates a [`TextUi`].
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        Self { text, color: None }
    }

    /// Sets the text color.
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl<'a> From<&'a str> for TextUi {
    #[inline]
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl<'a> From<&'a String> for TextUi {
    #[inline]
    fn from(text: &String) -> Self {
        Self::new(text)
    }
}

impl From<String> for TextUi {
    #[inline]
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl<'a, C> From<(&'a str, C)> for TextUi
where
    C: Into<Color>,
{
    #[inline]
    fn from((text, color): (&str, C)) -> Self {
        Self::new(text).with_color(color)
    }
}

impl<'a, C> From<(&'a String, C)> for TextUi
where
    C: Into<Color>,
{
    #[inline]
    fn from((text, color): (&String, C)) -> Self {
        Self::new(text).with_color(color)
    }
}

impl<C> From<(String, C)> for TextUi
where
    C: Into<Color>,
{
    #[inline]
    fn from((text, color): (String, C)) -> Self {
        Self::new(text).with_color(color)
    }
}

/* -------------------------------------------------------------------------- */
