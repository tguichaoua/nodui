mod node;

use std::ops::{Deref, DerefMut};

use node::NodeAdapter;
use nodui::{ConnectionHint, NodeSeq, SizeHint};
use serde::{Deserialize, Serialize};

use crate::graph::{self, Graph};

#[derive(Default, Serialize, Deserialize)]
pub struct GraphAdapter {
    pub graph: Graph,
    pub selected_node: Option<graph::NodeId>,
}

impl Deref for GraphAdapter {
    type Target = Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl DerefMut for GraphAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl GraphAdapter {
    pub fn selected_node(&mut self) -> Option<&mut graph::Node> {
        self.selected_node
            .and_then(|node_id| self.graph.get_node_mut(node_id))
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
