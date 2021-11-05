use std::error::Error;

use piston_window::{
    EventLoop, EventSettings, MouseCursorEvent, PistonWindow, Window, WindowSettings,
};
use plotters::coord::types::RangedCoordf32;
use plotters::coord::ReverseCoordTranslate;
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};

use clothoids::Vec2;
pub use figure::{Curve, Figure};
use figure::{FigureDrawer, FigureUpdater};

mod figure;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type PlotChartContext<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>>;

pub struct PlotWindow {
    window: PistonWindow,
    plot_scale: (f32, f32),
    chart_margin: f32,
    label_size: f32,
    mouse_pos: [f64; 2],
    mouse_pos_in_chart: Option<Vec2>,
}

impl PlotWindow {
    pub fn new(name: &'static str, size: [u32; 2], plot_size: [f32; 2]) -> Self {
        let mut window: PistonWindow = WindowSettings::new(name, size).samples(4).build().unwrap();

        let size = window.draw_size();
        let plot_scale = (
            plot_size[0] / size.width as f32,
            plot_size[1] / size.height as f32,
        );

        window.set_event_settings(EventSettings::new().max_fps(30).ups(0).lazy(true));

        Self {
            window,
            plot_scale,
            chart_margin: 30.0,
            label_size: 30.0,
            mouse_pos: [0.0, 0.0],
            mouse_pos_in_chart: None,
        }
    }

    pub fn draw(
        &mut self,
        draw_figures: impl FnOnce(&mut FigureDrawer<PistonBackend>) -> Result<()>,
    ) -> Option<FigureUpdater> {
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

            self.mouse_pos_in_chart = chart
                .as_coord_spec()
                .reverse_translate((self.mouse_pos[0] as i32, self.mouse_pos[1] as i32))
                .map(|(x, y)| Vec2::new(x, y));

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

        maybe_event.map(|event| {
            event.mouse_cursor(|pos| {
                let draw_size = self.window.draw_size();
                let window_size = self.window.size();
                self.mouse_pos = [
                    pos[0] * draw_size.width / window_size.width,
                    pos[1] * draw_size.height / window_size.height,
                ];
            });

            FigureUpdater::new(event, self.mouse_pos_in_chart)
        })
    }
}
