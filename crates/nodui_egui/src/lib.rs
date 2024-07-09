//! The [egui]-based visual graph editor for [nodui].
//!
//! [nodui]: https://crates.io/crates/nodui

pub mod connection;
pub mod context_menu;
mod conversion;
mod editor;
mod node;
mod socket;
mod viewport;

pub use connection::CustomConnectionRenderer;
pub use editor::{GraphEditor, GraphOutput};
pub use socket::RenderedSocket;
