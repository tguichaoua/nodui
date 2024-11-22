#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

pub struct App {
    state: State,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct State {
    selected_anchor: Anchor,

    visual_math: visual_math::App,
}

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumString,
    strum::IntoStaticStr,
)]
enum Anchor {
    #[default]
    VisualMath,
}

impl core::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl App {
    #[must_use]
    #[inline]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        App { state }
    }
}

impl eframe::App for App {
    #[inline]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    #[inline]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            let anchor = anchor.parse().ok();
            if let Some(anchor) = anchor {
                self.state.selected_anchor = anchor;
            }
        }

        egui::TopBottomPanel::top("demo_app_top_bar")
            .frame(egui::Frame::none().inner_margin(4.0))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.visuals_mut().button_frame = false;
                    self.bar_contents(ui, frame);
                });
            });

        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor {
                app.update(ctx, frame);
            }
        }
    }
}

impl App {
    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        #[allow(trivial_casts)]
        let vec = vec![(
            "Visual Math",
            Anchor::VisualMath,
            &mut self.state.visual_math as &mut dyn eframe::App,
        )];

        vec.into_iter()
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut selected_anchor = self.state.selected_anchor;
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor;
                if frame.is_web() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
                }
            }
        }
        self.state.selected_anchor = selected_anchor;
    }
}
