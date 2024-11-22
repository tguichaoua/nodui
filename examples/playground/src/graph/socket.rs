use serde::{Deserialize, Serialize};

use super::SocketId;

#[derive(Serialize, Deserialize)]
pub struct Socket {
    id: SocketId,
    pub side: nodui::ui::NodeSide,
    pub name: nodui::ui::TextUi,
    pub color: nodui::ui::Color,
    pub shape: nodui::ui::SocketShape,
}

impl Socket {
    pub fn new() -> Self {
        Self {
            id: SocketId::new(),
            side: nodui::ui::NodeSide::Left,
            name: nodui::ui::TextUi::default(),
            color: nodui::ui::Color::WHITE,
            shape: nodui::ui::SocketShape::default(),
        }
    }

    pub fn id(&self) -> SocketId {
        self.id
    }
}
