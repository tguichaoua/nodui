//! Rendering options for the nodes.

use super::{Color, Padding, Stroke, TextUi};

/* -------------------------------------------------------------------------- */

/// Defines how a node should be rendered.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeUI {
    /// The header of the node.
    pub header: NodeHeader,

    /// The body of the node.
    pub body: NodeBody,

    /// The node's outline.
    pub outline: Stroke,
}

impl Default for NodeUI {
    #[inline]
    fn default() -> Self {
        Self {
            header: NodeHeader::default(),
            body: NodeBody::default(),
            outline: Stroke::new(1.0, Color::WHITE),
        }
    }
}

impl NodeUI {
    /// Sets the header of the node.
    #[inline]
    #[must_use]
    pub fn with_header(mut self, header: impl Into<NodeHeader>) -> Self {
        self.header = header.into();
        self
    }

    /// Sets the body of the node.
    #[inline]
    #[must_use]
    pub fn with_body(mut self, body: NodeBody) -> Self {
        self.body = body;
        self
    }

    /// Sets the node's outline.
    #[inline]
    #[must_use]
    pub fn with_outline(mut self, outline: impl Into<Stroke>) -> Self {
        self.outline = outline.into();
        self
    }
}

/* -------------------------------------------------------------------------- */

/// Defines how the node header should be rendered.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum NodeHeader {
    /// No header.
    #[default]
    None,

    /// A simple header with just a title.
    Title(TitleHeader),
}

/// A simple header for a node with just a title.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TitleHeader {
    /// The text of the title.
    pub title: TextUi,

    /// The background color of the header.
    pub background: Color,
}

impl From<TitleHeader> for NodeHeader {
    #[inline]
    fn from(value: TitleHeader) -> Self {
        NodeHeader::Title(value)
    }
}

impl TitleHeader {
    /// Creates a [`TitleHeader`].
    #[inline]
    pub fn new(title: impl Into<TextUi>, background: impl Into<Color>) -> Self {
        let title = title.into();
        let background = background.into();
        Self { title, background }
    }
}

/* -------------------------------------------------------------------------- */

/// Defines how the node body should be rendered.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeBody {
    /// The layout for the sockets.
    pub layout: NodeLayout,

    /// The background color.
    pub background_color: Color,

    /// The padding of the body.
    pub padding: Padding,

    /// The space between the two columns when `layout` is [`NodeLayout::Double`].
    pub column_gap: f32,
}

impl Default for NodeBody {
    #[inline]
    fn default() -> Self {
        Self {
            layout: NodeLayout::default(),
            background_color: Color::from_rgba(0, 0, 0, 170),
            padding: Padding::same(5.0),

            column_gap: 5.0,
        }
    }
}

/* -------------------------------------------------------------------------- */

/// The layout for the body part of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeLayout {
    /// Render the sockets into a single column.
    Single,

    /// Render the sockets into two column based of their [`NodeSide`](super::NodeSide).
    #[default]
    Double,
}

/* -------------------------------------------------------------------------- */
