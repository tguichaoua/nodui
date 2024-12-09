//! The rendering of the viewport.

use egui::{
    epaint::RectShape, pos2, vec2, Color32, NumExt, Rect, Rounding, Shape, Stroke, UiBuilder,
};

use crate::viewport::Viewport;

use super::{stages, EditorState, GraphEditor};

/* -------------------------------------------------------------------------- */

impl GraphEditor<stages::Viewport> {
    /// Shows the viewport of the editor.
    #[inline]
    pub fn show_viewport<S>(self, ui: &mut egui::Ui) -> GraphEditor<stages::Nodes<S>>
    where
        S: Send + Sync + Clone + 'static,
    {
        let Self {
            id,
            stage:
                stages::Viewport {
                    width,
                    height,
                    view_aspect,
                    min_size,
                },
        } = self;

        // TODO
        let background_color = Color32::BLACK;
        let grid_stroke = Stroke::new(0.5, Color32::DARK_GRAY);
        let look_at = None;

        /* ---- */

        let pos = ui.available_rect_before_wrap().min;

        let size = {
            let width = width
                .unwrap_or_else(|| {
                    if let (Some(height), Some(aspect)) = (height, view_aspect) {
                        height * aspect
                    } else {
                        ui.available_size_before_wrap().x
                    }
                })
                .at_least(min_size.x);

            let height = height
                .unwrap_or_else(|| {
                    if let Some(aspect) = view_aspect {
                        width / aspect
                    } else {
                        ui.available_size_before_wrap().y
                    }
                })
                .at_least(min_size.y);

            vec2(width, height)
        };

        let rect = Rect::from_min_size(pos, size);

        ui.advance_cursor_after_rect(rect);

        let mut ui = ui.new_child(UiBuilder::new().id_salt(id).max_rect(rect));
        ui.set_clip_rect(rect);

        /* ---- */

        let mut state = EditorState::<S>::load(ui.ctx(), id);

        /* ---- */

        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if response.dragged() {
            response.ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            state.viewport_position -= response.drag_delta();
        }

        let viewport = {
            if let Some(look_at) = look_at {
                let pos = state.grid.graph_to_canvas(look_at);
                state.viewport_position = pos;
            }

            Viewport {
                position: rect.center().to_vec2() - state.viewport_position.to_vec2(),
                grid: state.grid.clone(),
            }
        };

        /* ---- */

        // Paint the background
        ui.painter()
            .add(RectShape::filled(rect, Rounding::ZERO, background_color));

        /* ---- */

        // Paint the grid
        if !grid_stroke.is_empty() {
            let dx = state.viewport_position.to_vec2().x % state.grid.size;
            let dy = state.viewport_position.to_vec2().y % state.grid.size;

            let center = rect.center() - vec2(dx, dy);

            #[allow(clippy::cast_possible_truncation)]
            let n = (size.x / state.grid.size) as i32 / 2;
            #[allow(clippy::cast_possible_truncation)]
            let m = (size.y / state.grid.size) as i32 / 2;

            for x in (-n)..(n + 2) {
                #[allow(clippy::cast_precision_loss)]
                let x = x as f32;
                let x = x.mul_add(state.grid.size, center.x);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(x, rect.min.y), pos2(x, rect.max.y)],
                    stroke: grid_stroke.into(),
                });
            }

            for y in (-m)..(m + 2) {
                #[allow(clippy::cast_precision_loss)]
                let y = y as f32;
                let y = y.mul_add(state.grid.size, center.y);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                    stroke: grid_stroke.into(),
                });
            }

            // Outline around the viewport
            ui.painter().add(RectShape::stroke(
                rect,
                Rounding::ZERO,
                (1.0, grid_stroke.color),
            ));
        }

        /* ---- */

        GraphEditor {
            id,
            stage: stages::Nodes {
                ui,
                state,
                viewport,
                response,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
