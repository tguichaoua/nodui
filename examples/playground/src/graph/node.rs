use nodui::ui::NodeUI;
use serde::{Deserialize, Serialize};

use super::{socket::SocketStyle, NodeId, Socket};

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    pub pos: nodui::Pos,
    pub(super) sockets: Vec<Socket>,
    pub style: NodeStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeStyle {
    pub body: nodui::ui::NodeBody,
    pub header: nodui::ui::TitleHeader,
    pub outline: nodui::ui::Stroke,
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle {
            body: nodui::ui::NodeBody::default(),
            header: nodui::ui::TitleHeader::new(
                ("New Node", nodui::ui::Color::BLACK),
                egui::Color32::KHAKI.to_tuple(),
            ),
            outline: nodui::ui::Stroke {
                color: nodui::ui::Color::WHITE,
                width: 1.0,
            },
        }
    }
}

impl Node {
    pub(super) fn new(
        pos: nodui::Pos,
        style: NodeStyle,
        sockets: impl IntoIterator<Item = SocketStyle>,
    ) -> Self {
        let id = NodeId::new();

        Self {
            id,
            pos,
            sockets: sockets.into_iter().map(Socket::new).collect(),
            style,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn sockets(&self) -> &[Socket] {
        &self.sockets
    }

    pub fn sockets_mut(&mut self) -> &mut [Socket] {
        &mut self.sockets
    }

    pub fn add_socket(&mut self) -> &mut Socket {
        self.sockets.push(Socket::new(SocketStyle::default()));
        #[allow(clippy::unwrap_used)]
        self.sockets.last_mut().unwrap()
    }
}

impl NodeStyle {
    pub fn into_node_ui(self) -> NodeUI {
        let Self {
            body,
            header,
            outline,
        } = self;

        NodeUI {
            header: header.into(),
            body,
            outline,
        }
    }
}
