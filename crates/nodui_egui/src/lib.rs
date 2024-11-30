//! The [egui]-based visual graph editor for [nodui].
//!
//! [nodui]: https://crates.io/crates/nodui

mod editor2;
mod misc;
mod socket;
mod viewport;

pub use socket::RenderedSocket;
pub use viewport::Viewport;

pub use editor2::{
    stages, ConnectionInProgress, ConnectionsUi, GraphEditor as GraphEditor2, GraphResponse,
    GraphUi, NodeResponse, NodeUi, RenderedSocket as RenderedSocket2, Socket,
};
