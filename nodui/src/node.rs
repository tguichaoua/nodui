//! Rendering for nodes.

/* -------------------------------------------------------------------------- */

use egui::{Color32, WidgetText};

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
    pub text: WidgetText,
    /// The background color of the header.
    ///
    /// Note: [`Color32::PLACEHOLDER`] will be replace by the node body's color.
    pub background_color: Color32,
}

impl TitleHeader {
    /// Creates a [`TitleHeader`].
    #[inline]
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            background_color: Color32::PLACEHOLDER,
        }
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
