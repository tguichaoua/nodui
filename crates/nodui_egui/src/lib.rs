//! The [egui]-based visual graph editor for [nodui].
//!
//! [nodui]: https://crates.io/crates/nodui

mod editor;
mod misc;
mod socket;
mod viewport;

pub use editor::{
    stages, ConnectionsUi, GraphEditor, GraphResponse, GraphUi, NodeResponse, NodeUi,
};
pub use socket::{ConnectionInProgress, RenderedSocket, Socket};
pub use viewport::Viewport;
