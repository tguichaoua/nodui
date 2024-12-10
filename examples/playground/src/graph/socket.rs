use serde::{Deserialize, Serialize};

use super::{Maybe, SocketId};

#[derive(Serialize, Deserialize)]
pub struct Socket {
    id: SocketId,
    pub style: SocketStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocketStyle {
    pub side: nodui::NodeSide,
    pub name: String,
    pub name_color: Maybe<egui::Color32>,
    pub color: Maybe<egui::Color32>,
    pub shape: nodui::SocketShape,
}

impl Default for SocketStyle {
    fn default() -> Self {
        Self {
            side: nodui::NodeSide::Left,
            name: String::from("socket"),
            name_color: Maybe::disabled_with(egui::Color32::WHITE),
            color: Maybe::disabled_with(egui::Color32::WHITE),
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
