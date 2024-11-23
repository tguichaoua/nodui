use serde::{Deserialize, Serialize};

use super::SocketId;

#[derive(Serialize, Deserialize)]
pub struct Socket {
    id: SocketId,
    pub style: SocketStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SocketStyle {
    pub side: nodui::ui::NodeSide,
    pub name: nodui::ui::TextUi,
    pub color: nodui::ui::Color,
    pub shape: nodui::ui::SocketShape,
}

impl Default for SocketStyle {
    fn default() -> Self {
        Self {
            side: nodui::ui::NodeSide::Left,
            name: nodui::ui::TextUi::new("socket"),
            color: nodui::ui::Color::WHITE,
            shape: nodui::ui::SocketShape::default(),
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
