//! An [egui]-based visual graph editor.
//!
//! [`GraphEditor`] is a egui widget which create a visual graph editor.
//!
//! [egui]: https://docs.rs/egui/
//!

mod editor;
mod misc;
mod socket;
mod viewport;

pub use editor::{
    stages, ConnectionsUi, GraphEditor, GraphResponse, GraphUi, NodeLayout, NodeResponse, NodeUi,
};
pub use socket::{ConnectionInProgress, NodeSide, RenderedSocket, Socket, SocketShape};
pub use viewport::{Pos, Viewport};
