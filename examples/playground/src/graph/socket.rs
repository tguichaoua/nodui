use serde::{Deserialize, Serialize};

use super::SocketId;

#[derive(Serialize, Deserialize)]
pub struct Socket {
    id: SocketId,
    pub style: SocketStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocketStyle {
    pub side: nodui::NodeSide,
    pub name: String,
    pub name_color: egui::Color32,
    pub color: egui::Color32,
    pub shape: nodui::SocketShape,
}

impl Default for SocketStyle {
    fn default() -> Self {
        Self {
            side: nodui::NodeSide::Left,
            name: String::from("socket"),
            name_color: egui::Color32::WHITE,
            color: egui::Color32::WHITE,
            shape: nodui::SocketShape::default(),
        }
    }
}

impl Socket {
    pub(super) fn new(style: SocketStyle) -> Self {
        Self {
            id: SocketId::new(),
            style,
        }
    }

    pub fn id(&self) -> SocketId {
        self.id
    }
}
