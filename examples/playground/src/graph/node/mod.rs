mod header;

use serde::{Deserialize, Serialize};

use super::{socket::SocketStyle, Maybe, NodeId, Socket};

pub(crate) use header::{HeaderMode, NodeHeaderStyle};

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    pub pos: nodui::Pos,
    pub(super) sockets: Vec<Socket>,
    pub style: NodeStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NodeStyle {
    pub body: NodeBody,
    pub header: NodeHeaderStyle,
    pub outline: Maybe<egui::Stroke>,
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle {
            body: NodeBody::default(),
            header: NodeHeaderStyle::default(),
            outline: Maybe::disabled_with(egui::Stroke::new(1.0, egui::Color32::WHITE)),
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

// impl From<NodeStyle> for NodeUI {
//     #[inline]
//     fn from(value: NodeStyle) -> Self {
//         let NodeStyle {
//             body,
//             header,
//             outline,
//         } = value;

//         NodeUI {
//             header: header.into(),
//             body,
//             outline,
//         }
//     }
// }

/* -------------------------------------------------------------------------- */

/// Defines how the node body should be rendered.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NodeBody {
    /// The layout for the sockets.
    pub layout: nodui::NodeLayout,

    /// The background color.
    pub background_color: Maybe<egui::Color32>,

    /// The padding of the body.
    pub padding: egui::Margin,

    /// The space between the two columns when `layout` is [`NodeLayout::Double`].
    pub column_gap: f32,
}

impl Default for NodeBody {
    #[inline]
    fn default() -> Self {
        Self {
            layout: nodui::NodeLayout::Double,
            background_color: Maybe::disabled_with(egui::Color32::from_black_alpha(170)),
            padding: egui::Margin::same(5.0),
            column_gap: 5.0,
        }
    }
}

/* -------------------------------------------------------------------------- */
