//! Contextual information for context menu of the graph editor.

use egui::Ui;
use nodui_core::adapter::{GraphAdapter, Pos};

/* -------------------------------------------------------------------------- */

/// A boxed dynamic callback for the visual editor context menu.
pub(crate) type ContextMenuContent<'a, G> = Box<dyn FnMut(&mut Ui, MenuContext<'_, G>) + 'a>;

/// Contextual information for graph editor context menu.
pub struct MenuContext<'a, G: GraphAdapter> {
    /// A mutable reference of the [`GraphAdapter`] used by the editor.
    pub graph: &'a mut G,

    /// The graph position of the pointer.
    pub pos: Pos,
}

/* -------------------------------------------------------------------------- */

/// A boxed dynamic callback for a node context menu.
pub(crate) type NodeContextMenuContent<'a, G> =
    Box<dyn FnMut(&mut Ui, NodeMenuContext<'_, G>) + 'a>;

/// Contextual information for graph editor's node context menu.
pub struct NodeMenuContext<'a, G: GraphAdapter> {
    /// A mutable reference of the [`GraphAdapter`] used by the editor.
    pub graph: &'a mut G,

    /// The identifier of the node.
    pub node_id: G::NodeId,
}

/* -------------------------------------------------------------------------- */

/// A boxed dynamic callback for a socket context menu.
pub(crate) type SocketContextMenuContent<'a, G> =
    Box<dyn FnMut(&mut Ui, SocketMenuContext<'_, G>) + 'a>;

/// Contextual information for graph editor's socket context menu.
pub struct SocketMenuContext<'a, G: GraphAdapter> {
    /// A mutable reference of the [`GraphAdapter`] used by the editor.
    pub graph: &'a mut G,

    /// The identifier of the socket.
    pub socket_id: G::SocketId,
}

/* -------------------------------------------------------------------------- */
