use serde::{Deserialize, Serialize};

use crate::graph::Maybe;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NodeHeaderStyle {
    pub mode: HeaderMode,
    pub title: String,
    pub title_color: Maybe<egui::Color32>,
    pub background: Maybe<egui::Color32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(crate) enum HeaderMode {
    None,
    Title,
}

impl Default for NodeHeaderStyle {
    fn default() -> Self {
        Self {
            mode: HeaderMode::Title,
            title: String::from("New Node"),
            title_color: Maybe::disabled_with(egui::Color32::BLACK),
            background: Maybe::disabled_with(egui::Color32::KHAKI),
        }
    }
}

// impl From<NodeHeaderStyle> for nodui::NodeHeader {
//     #[inline]
//     fn from(value: NodeHeaderStyle) -> Self {
//         let NodeHeaderStyle {
//             mode,
//             title,
//             background,
//         } = value;

//         match mode {
//             HeaderMode::None => nodui::NodeHeader::None,
//             HeaderMode::Title => {
//                 nodui::NodeHeader::Title(nodui::TitleHeader { title, background })
//             }
//         }
//     }
// }
