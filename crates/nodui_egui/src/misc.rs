// struct LineSegment {
//     points: [Pos2; 2],
//     stroke: Stroke,
// }

// impl LineSegment {
//     fn into_shape(self) -> egui::Shape {
//         let Self { points, stroke } = self;
//         egui::Shape::LineSegment { points, stroke }
//     }
// }

// fn gradient_segment(
//     pos: RangeInclusive<Pos2>,
//     color: RangeInclusive<Color32>,
//     width: f32,
//     steps: usize,
// ) -> impl Iterator<Item = LineSegment> {
//     let color = Rgba::from(*color.start())..=Rgba::from(*color.end());

//     let dir = *pos.end() - *pos.start();

//     (0..=steps)
//         .map(move |i| {
//             let t = i as f32 / steps as f32;
//             let p = *pos.start() + dir * t;
//             let color = Color32::from(egui::lerp(color.clone(), t));

//             (p, color)
//         })
//         .tuple_windows()
//         .map(move |((p0, _), (p1, color))| LineSegment {
//             points: [p0, p1],
//             stroke: Stroke { width, color },
//         })
// }
