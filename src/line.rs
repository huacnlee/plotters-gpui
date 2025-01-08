use gpui::{point, Hsla, Path, Pixels, Point, WindowContext};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct Line {
    pub points: Vec<Point<Pixels>>,
    pub width: Pixels,
    pub color: Hsla,
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

    pub fn width(mut self, width: f64) -> Self {
        self.width = width.into();
        self
    }

    pub fn add_point(&mut self, point: Point<Pixels>) {
        self.points.push(point);
    }

    pub fn render_pixels(&mut self, cx: &mut WindowContext) {
        if self.points.is_empty() {
            warn!("Line must have at least 1 points to render");
            return;
        }

        dbg!(&self.points);

        let stroke = tiny_skia::Stroke {
            width: self.width.0,
            ..Default::default()
        };
        let mut builder = tiny_skia::PathBuilder::new();
        for p in self.points.iter() {
            builder.line_to(p.x.0, p.y.0);
        }
        let Some(path) = builder.finish() else {
            return;
        };
        let Some(s_path) = path.stroke(&stroke, cx.scale_factor()) else {
            return;
        };

        let Some(first_p) = s_path.points().first() else {
            return;
        };

        let mut path = Path::new(s_point(*first_p));
        for i in 1..s_path.len() - 1 {
            let p = s_path.points()[i];
            let verb = s_path.verbs()[i];

            match verb {
                tiny_skia_path::PathVerb::Line => {
                    path.line_to(s_point(p));
                }
                tiny_skia_path::PathVerb::Move => {
                    path.move_to(s_point(p));
                }
                tiny_skia_path::PathVerb::Close => path.close(),
                _ => todo!(),
            }
        }
        cx.paint_path(path, self.color);
    }
}

fn s_point(p: tiny_skia::Point) -> Point<Pixels> {
    point(p.x.into(), p.y.into())
}
