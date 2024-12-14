//! Graph editor's response.

use crate::{Pos, RenderedSocket, Viewport};

/* -------------------------------------------------------------------------- */

/// The result of rendering a [`GraphEditor`][crate::GraphEditor].
pub struct GraphResponse<S> {
    /// The viewport of the editor.
    ///
    /// Use it for coordinates conversion.
    pub viewport: Viewport,
    /// The [`Response`][egui::Response] of the editor.
    pub response: egui::Response,
    /// The sockets that have been rendered.
    pub sockets: Vec<RenderedSocket<S>>,
    /// Whether the user create a new connection.
    pub connection: Option<(S, S)>,
    /// The position of the viewport.
    pub position: Pos,
}

/* -------------------------------------------------------------------------- */
