use gpui::{div, prelude::*, App, AppContext, Application, Context, Entity, Window, WindowOptions};
use parking_lot::RwLock;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use plotters_gpui::backend::GpuiBackend;
use plotters_gpui::element::{PlottersChart, PlottersDrawAreaModel, PlottersDrawAreaViewer};
use std::rc::Rc;

struct MainViewer {
    figure: Entity<PlottersDrawAreaViewer>,
}

impl MainViewer {
    fn new(model: Rc<RwLock<PlottersDrawAreaModel>>, cx: &mut App) -> Self {
        let figure = PlottersDrawAreaViewer::with_shared_model(model);

        Self {
            figure: cx.new(move |_| figure),
        }
    }
}

impl Render for MainViewer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_col()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child(self.figure.clone())
    }
}

struct MyChart;
impl PlottersChart for MyChart {
    fn plot(
        &mut self,
        root: &DrawingArea<GpuiBackend, Shift>,
    ) -> Result<(), plotters_gpui::DrawingErrorKind> {
        let x_axis = (-3.0..3.0).step(0.1);
        let z_axis = (-3.0..3.0).step(0.1);

        let mut chart = ChartBuilder::on(root)
            .caption("3D Plot Test", ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())
            .unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()
            .unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-30..30).map(|f| f as f64 / 10.0),
                    (-30..30).map(|f| f as f64 / 10.0),
                    |x, z| (x * x + z * z).cos(),
                )
                .style(BLUE.mix(0.2).filled()),
            )
            .unwrap()
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .draw_series(LineSeries::new(
                (-100..100)
                    .map(|y| y as f64 / 40.0)
                    .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                &BLACK,
            ))
            .unwrap()
            .label("Line")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

        chart
            .configure_series_labels()
            .border_style(BLACK)
            .draw()
            .unwrap();

        Ok(())
    }
}

fn main_viewer(cx: &mut App) -> MainViewer {
    let figure = PlottersDrawAreaModel::new(Box::new(MyChart));
    MainViewer::new(Rc::new(RwLock::new(figure)), cx)
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            move |_, cx| {
                let view = main_viewer(cx);
                cx.new(move |_| view)
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
