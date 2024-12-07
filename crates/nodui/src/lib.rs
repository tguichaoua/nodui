//! An [egui]-based visual graph editor.
//!
//! [`GraphEditor`] is a egui widget which create a visual graph editor.
//!
//! [egui]: https://docs.rs/egui/
//!
//! ```
//! # struct MyGraph;
//! # struct Foo;
//! # impl nodui::GraphAdapter for MyGraph {
//! #    type NodeId = ();
//! #    type SocketId = ();
//! #    fn accept<'graph, V>(&'graph mut self, mut visitor: V) where V: nodui::GraphVisitor<'graph, Self::NodeId, Self::SocketId> { }
//! #    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> nodui::ConnectionHint { unreachable!() }
//! #    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) { }
//! #    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> { std::iter::empty() }
//! # }
//! # impl nodui::NodeAdapter for Foo {
//! #    type NodeId = ();
//! #    type SocketId = ();
//! #    fn accept<'node, V>(&'node mut self, mut visitor: V) where V: nodui::NodeVisitor<'node, Self::SocketId> { }
//! #    fn id(&self) -> Self::NodeId { unreachable!() }
//! #    fn pos(&self) -> nodui::Pos { unreachable!() }
//! #    fn set_pos(&mut self, _: nodui::Pos) { }
//! # }
//! struct App {
//!     // `MyGraph` implements the `GraphAdapter` trait and hold the state for the visual editor.
//!     graph: MyGraph,
//! }
//!
//! impl eframe::App for App {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         egui::CentralPanel::default().show(ctx, |ui| {
//!             let graph = nodui::GraphEditor::new(&mut self.graph, "graph");
//!
//!             graph.show(ui);
//!         });
//!     }
//! }
//! ```
mod editor;
mod misc;
mod socket;
mod viewport;

pub use editor::{
    stages, ConnectionsUi, GraphEditor, GraphResponse, GraphUi, NodeLayout, NodeResponse, NodeUi,
};
pub use socket::{ConnectionInProgress, NodeSide, RenderedSocket, Socket, SocketShape};
pub use viewport::{Pos, Viewport};
