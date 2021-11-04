use std::error::Error;

use piston_window::{Event, EventLoop, PistonWindow, WindowSettings};
use plotters::coord::types::RangedCoordf32;
use plotters::element::PointElement;
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};

use clothoids::Vec2;
use figure::FigureDrawer;
pub use figure::{Curve, Figure};

mod figure;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type PlotChartContext<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>>;

pub struct Window {
    window: PistonWindow,
    plot_scale: (f32, f32),
    chart_margin: f32,
    label_size: f32,
}

impl Window {
    pub fn new(name: &'static str, size: [u32; 2], plot_size: [f32; 2]) -> Self {
        let plot_scale = (plot_size[0] / size[0] as f32, plot_size[1] / size[1] as f32);

        let mut window: PistonWindow = WindowSettings::new(name, size).samples(4).build().unwrap();

        window.set_max_fps(30);

        Self {
            window,
            plot_scale,
            chart_margin: 30.0,
            label_size: 30.0,
        }
    }

    pub fn draw(
        &mut self,
        draw_figures: impl FnOnce(&mut FigureDrawer<PistonBackend>) -> Result<()>,
    ) -> Option<Event> {
        let maybe_event = draw_piston_window(&mut self.window, |b| {
            let window_width = b.get_size().0 as f32 - 2.0 * self.chart_margin - self.label_size;
            let window_height = b.get_size().1 as f32 - 2.0 * self.chart_margin - self.label_size;

            let root = b.into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .margin(self.chart_margin)
                .x_label_area_size(self.label_size)
                .y_label_area_size(self.label_size)
                .build_cartesian_2d(
                    0f32..window_width * self.plot_scale.0,
                    0f32..window_height * self.plot_scale.1,
                )?;

            chart
                .configure_mesh()
                .x_labels(5)
                .y_labels(5)
                .label_style(("sans-serif", 20))
                .draw()?;

            draw_figures(&mut FigureDrawer::new(&mut chart))?;

            chart
                .configure_series_labels()
                .label_font(("sans-serif", 20))
                .background_style(&WHITE.mix(0.5))
                .border_style(&BLACK)
                .draw()?;

            Ok(())
        });

        maybe_event.map(|event| event)
    }
}
