use egui::Color32;

/* -------------------------------------------------------------------------- */

pub(super) enum Header {
    None,
    Title(TitleHeader),
}

pub(super) struct TitleHeader {
    pub text: String,
    pub text_color: Color32,
    pub background: Color32,
}

/* -------------------------------------------------------------------------- */
