mod node;

use std::ops::{Deref, DerefMut};

use node::NodeAdapter;
use nodui::{ConnectionHint, NodeSeq, SizeHint};
use serde::{Deserialize, Serialize};

use crate::graph;

#[derive(Default, Serialize, Deserialize)]
pub struct GraphAdapter {
    pub graph: graph::Graph,
    pub selected_node: Option<graph::NodeId>,
    pub clipboard: Option<(graph::NodeStyle, Vec<graph::SocketStyle>)>,
}

impl Deref for GraphAdapter {
    type Target = graph::Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl DerefMut for GraphAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl nodui::GraphAdapter for GraphAdapter {
    type NodeId = graph::NodeId;
    type SocketId = graph::SocketId;

    fn accept<'graph, V>(&'graph mut self, mut visitor: V)
    where
        V: nodui::GraphVisitor<'graph, Self::NodeId, Self::SocketId>,
    {
        let graph::ViewMut { nodes, connections } = self.graph.view_mut();
        let mut node_seq = visitor.nodes(SizeHint::of(nodes));
        for node in nodes {
            node_seq.visit_node(NodeAdapter { node, connections });
        }
    }

    fn connection_hint(&self, _a: Self::SocketId, _b: Self::SocketId) -> ConnectionHint {
        ConnectionHint::Accept // TODO
    }

    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) {
        self.connections_mut().connect(a, b);
    }

    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> {
        self.graph.connections().iter()
    }
}

impl GraphAdapter {
    pub fn selected_node(&mut self) -> Option<&mut graph::Node> {
        self.selected_node
            .and_then(|node_id| self.graph.get_node_mut(node_id))
    }

    pub fn delete_selected_node(&mut self) {
        if let Some(node_id) = self.selected_node.take() {
            self.graph.remove_node(node_id);
        }
    }

    pub fn new_node(&mut self, pos: nodui::Pos) {
        let node = self.add_node(pos, graph::NodeStyle::default(), []);
        self.selected_node = Some(node.id());
    }

    pub fn copy_node(&mut self, node_id: graph::NodeId) {
        if let Some(node) = self.graph.get_node(node_id) {
            self.clipboard = Some((
                node.style.clone(),
                node.sockets().iter().map(|s| s.style.clone()).collect(),
            ));
        }
    }

    pub fn paste_node(&mut self, pos: nodui::Pos) {
        if let Some((settings, sockets)) = self.clipboard.as_ref() {
            let node = self
                .graph
                .add_node(pos, settings.clone(), sockets.iter().cloned());
            self.selected_node = Some(node.id());
        }
    }

    pub fn paste_node_settings_to(&mut self, target_node: graph::NodeId) {
        if let Some((settings, _)) = self.clipboard.as_ref() {
            if let Some(target) = self.graph.get_node_mut(target_node) {
                target.style = settings.clone();
            }
        }
    }

    pub fn paste_sockets_to(&mut self, target_node: graph::NodeId) {
        if let Some((_, sockets)) = self.clipboard.as_ref() {
            self.graph
                .replace_sockets(target_node, sockets.iter().cloned());
        }
    }
}
