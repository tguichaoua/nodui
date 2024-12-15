//! Utilities for layout.

use egui::{vec2, Vec2};

/// Stacks the sizes horizontally.
pub(crate) fn stack_horizontally(sizes: impl IntoIterator<Item = Vec2>) -> Vec2 {
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;

    sizes.into_iter().for_each(|size| {
        x += size.x;
        y = y.max(size.y);
    });

    vec2(x, y)
}

/// Stacks the sizes vertically.
pub(crate) fn stack_vertically(sizes: impl IntoIterator<Item = Vec2>) -> Vec2 {
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;

    sizes.into_iter().for_each(|size| {
        x = x.max(size.x);
        y += size.y;
    });

    vec2(x, y)
}

/// Stacks the sizes vertically and add the gap vertically in between each item.
pub(crate) fn stack_vertically_with_gap(sizes: impl IntoIterator<Item = Vec2>, gap: f32) -> Vec2 {
    let mut sizes = sizes.into_iter();

    let mut x = 0.0_f32;
    let mut y = 0.0_f32;

    let Some(first) = sizes.next() else {
        return Vec2::ZERO;
    };

    x = x.max(first.x);
    y += first.y;

    sizes.for_each(|size| {
        x = x.max(size.x);
        y += size.y + gap;
    });

    vec2(x, y)
}
