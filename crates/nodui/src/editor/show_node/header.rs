//! Node's header.

use egui::Color32;

/* -------------------------------------------------------------------------- */

/// An header for a node.
pub(super) enum Header {
    /// No header.
    None,
    /// A simple header with a title.
    Title(TitleHeader),
}

/// A simple header with a title.
pub(super) struct TitleHeader {
    /// The text of the title.
    pub text: String,
    /// The color of the title.
    pub text_color: Color32,
    /// The background color of the header.
    pub background: Color32,
}

/* -------------------------------------------------------------------------- */
