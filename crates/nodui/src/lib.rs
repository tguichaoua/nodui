//! An [egui]-based visual graph editor.
//!
//! [`GraphEditor`] is a egui widget which create a visual graph editor.
//!
//! The visual editor and the graph data interact via the [adapter traits](nodui_core::adapter).
//!
//! [egui]: https://docs.rs/egui/
//!
//! ```
//! # struct MyGraph;
//! # struct Foo;
//! # impl nodui::GraphAdapter for MyGraph {
//! #    type NodeId = ();
//! #    type SocketId = ();
//! #    fn nodes(&self) -> impl Iterator<Item: nodui::NodeAdapter<NodeId = Self::NodeId, SocketId = Self::SocketId>> {
//! #        core::iter::empty::<Foo>()
//! #    }
//! #    fn set_node_pos(&mut self, node_id: Self::NodeId, pos: nodui::Pos) { }
//! #    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> nodui::ConnectionHint { unreachable!() }
//! #    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) { }
//! #    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> { std::iter::empty() }
//! # }
//! # impl nodui::NodeAdapter for Foo {
//! #    type NodeId = ();
//! #    type SocketId = ();
//! #    fn sockets(&self) -> impl Iterator<Item: nodui::SocketAdapter<SocketId = Self::SocketId>> {
//! #        core::iter::empty::<Foo>()
//! #    }
//! #    fn id(&self) -> Self::NodeId { unreachable!() }
//! #    fn pos(&self) -> nodui::Pos { unreachable!() }
//! # }
//! # impl nodui::SocketAdapter for Foo {
//! #   type SocketId = ();
//! #   fn id(&self) -> Self::SocketId { unreachable!() }
//! #   fn ui(&self) -> nodui::ui::SocketUI { unreachable!() }
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

pub use nodui_core::*;
pub use nodui_egui::*;
