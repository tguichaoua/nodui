//! Rendering for nodes.

/* -------------------------------------------------------------------------- */

use egui::Color32;

/// The layout for the body part of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NodeLayout {
    /// Render the sockets into a single column.
    Single,

    /// Render the sockets into two column based of their [`NodeSide`](crate::NodeSide).
    #[default]
    Double,
}

/* -------------------------------------------------------------------------- */

/// An header for a node.
pub enum Header {
    /// No header.
    None,
    /// A simple header with a title.
    Title(TitleHeader),
}

impl From<TitleHeader> for Header {
    #[inline]
    fn from(value: TitleHeader) -> Self {
        Header::Title(value)
    }
}

/* -------------------------------------------------------------------------- */

/// A simple header with a title.
pub struct TitleHeader {
    /// The text of the title.
    pub text: String,
    /// The color of the title.
    ///
    /// Note: [`Color32::PLACEHOLDER`] will be replace by [`egui::Visuals::text_color()`].
    pub text_color: Color32,
    /// The background color of the header.
    ///
    /// Note: [`Color32::PLACEHOLDER`] will be replace by the node body's color.
    pub background_color: Color32,
}

impl Default for TitleHeader {
    #[inline]
    fn default() -> Self {
        Self {
            text: String::new(),
            text_color: Color32::PLACEHOLDER,
            background_color: Color32::PLACEHOLDER,
        }
    }
}

impl TitleHeader {
    /// Create a [`TitleHeader`].
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// The color of the title.
    #[must_use]
    #[inline]
    pub fn text_color(mut self, color: impl Into<Color32>) -> Self {
        self.text_color = color.into();
        self
    }

    /// The background color of the header.
    #[must_use]
    #[inline]
    pub fn background_color(mut self, color: impl Into<Color32>) -> Self {
        self.background_color = color.into();
        self
    }
}

/* -------------------------------------------------------------------------- */
