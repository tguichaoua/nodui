use serde::{Deserialize, Serialize};

use super::{NodeId, Socket, SocketId};

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    pub pos: nodui::Pos,
    sockets: Vec<Socket>,
    pub body: nodui::ui::NodeBody,
    pub header: nodui::ui::TitleHeader,
    pub outline: nodui::ui::Stroke,
}

impl Node {
    pub fn new(pos: nodui::Pos) -> Self {
        let id = NodeId::new();

        Self {
            id,
            pos,
            sockets: Vec::new(),
            body: nodui::ui::NodeBody::default(),
            header: nodui::ui::TitleHeader::new(
                ("New Node", nodui::ui::Color::BLACK),
                egui::Color32::KHAKI.to_tuple(),
            ),
            outline: nodui::ui::Stroke::default(),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn sockets_mut(&mut self) -> &mut [Socket] {
        &mut self.sockets
    }

    pub fn add_socket(&mut self) -> &mut Socket {
        self.sockets.push(Socket::new());
        #[allow(clippy::unwrap_used)]
        self.sockets.last_mut().unwrap()
    }

    pub fn get_node_mut(&mut self, id: SocketId) -> Option<&mut Socket> {
        self.sockets.iter_mut().find(|socket| socket.id() == id)
    }
}
