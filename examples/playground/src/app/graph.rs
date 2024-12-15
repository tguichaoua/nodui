use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::graph;

#[derive(Default, Serialize, Deserialize)]
pub struct GraphApp {
    pub graph: graph::Graph,
    pub selected_node: Option<(graph::NodeId, egui::Id)>,
    pub clipboard: Option<(graph::NodeStyle, Vec<graph::SocketStyle>)>,
}

impl Deref for GraphApp {
    type Target = graph::Graph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl DerefMut for GraphApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl GraphApp {
    pub fn selected_node(&mut self) -> Option<&mut graph::Node> {
        self.selected_node
            .and_then(|(node_id, _)| self.graph.get_node_mut(node_id))
    }

    pub fn delete_selected_node(&mut self) {
        if let Some((node_id, _)) = self.selected_node.take() {
            self.graph.remove_node(node_id);
        }
    }

    pub fn new_node(&mut self, pos: nodui::Pos) {
        let node = self.add_node(pos, graph::NodeStyle::default(), []);
        self.selected_node = Some((node.id(), egui::Id::NULL));
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
            self.selected_node = Some((node.id(), egui::Id::NULL));
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
