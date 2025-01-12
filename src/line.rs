use gpui::{point, px, Hsla, Path, Pixels, Point, WindowContext};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct Line {
    pub points: Vec<Point<Pixels>>,
    pub width: Pixels,
    pub color: Hsla,
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Line {
    pub fn new() -> Self {
        Self {
            points: vec![],
            width: 1.0.into(),
            color: gpui::black(),
        }
    }

    pub fn between_points(start: Point<Pixels>, end: Point<Pixels>) -> Self {
        let mut line = Self::new();
        line.add_point(start);
        line.add_point(end);
        line
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    pub fn add_point(&mut self, point: Point<Pixels>) {
        self.points.push(point);
    }

    pub fn render_pixels(&mut self, cx: &mut WindowContext) {
        if self.points.is_empty() {
            warn!("Line must have at least 1 point to render");
            return;
        }

        let stroke = tiny_skia::Stroke {
            width: self.width.0,
            line_join: tiny_skia::LineJoin::Miter,
            line_cap: tiny_skia::LineCap::Square,
            ..Default::default()
        };

        // Handle single point case
        if self.points.len() == 1 {
            let point = self.points[0];
            let mut builder = tiny_skia::PathBuilder::new();
            builder.move_to(point.x.0, point.y.0);
            builder.line_to(point.x.0, point.y.0);

            if let Some(path) = stroke_path(builder, &stroke, cx) {
                cx.paint_path(path, self.color);
            }
            return;
        }

        // Draw each line segment separately to handle overlapping properly
        for window in self.points.windows(2) {
            let mut builder = tiny_skia::PathBuilder::new();
            builder.move_to(window[0].x.0, window[0].y.0);
            builder.line_to(window[1].x.0, window[1].y.0);

            if let Some(path) = stroke_path(builder, &stroke, cx) {
                cx.paint_path(path, self.color);
            }
        }
    }
}

/// Build tiny-skia PathBuilder into a Path with stroke
fn stroke_path(
    builder: tiny_skia::PathBuilder,
    stroke: &tiny_skia::Stroke,
    cx: &WindowContext,
) -> Option<Path<Pixels>> {
    let skia_path = builder.finish()?;
    let skia_path = skia_path.stroke(stroke, cx.scale_factor())?;
    let first_p = skia_path.points().first()?;

    let mut path = Path::new(point(px(first_p.x), px(first_p.y)));
    for i in 1..skia_path.len() - 1 {
        let verb = skia_path.verbs()[i];
        let Some(p) = skia_path.points().get(i) else {
            continue;
        };

        match verb {
            tiny_skia_path::PathVerb::Move => path.move_to(point(px(p.x), px(p.y))),
            tiny_skia_path::PathVerb::Line => path.line_to(point(px(p.x), px(p.y))),
            _ => {}
        }
    }

    Some(path)
}
