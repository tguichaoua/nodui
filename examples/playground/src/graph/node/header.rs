use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NodeHeaderStyle {
    pub mode: HeaderMode,
    pub title: nodui::ui::TextUi,
    pub background: nodui::ui::Color,
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
            title: nodui::ui::TextUi::new("New Node").with_color(nodui::ui::Color::BLACK),
            background: egui::Color32::KHAKI.to_tuple().into(),
        }
    }
}

impl From<NodeHeaderStyle> for nodui::ui::NodeHeader {
    #[inline]
    fn from(value: NodeHeaderStyle) -> Self {
        let NodeHeaderStyle {
            mode,
            title,
            background,
        } = value;

        match mode {
            HeaderMode::None => nodui::ui::NodeHeader::None,
            HeaderMode::Title => {
                nodui::ui::NodeHeader::Title(nodui::ui::TitleHeader { title, background })
            }
        }
    }
}
