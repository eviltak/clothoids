use piston_window::{Button, Event, MouseButton, PressEvent, ReleaseEvent};
use plotters::prelude::*;

use clothoids::{ParametricCurve, Vec2};

use crate::util::CubicBezier;

use super::{PlotChartContext, Result};

const CURVE_LENGTH: usize = 500;

pub trait Curve<const N: usize> {
    fn from_points(points: [Vec2; N]) -> Self;

    fn to_line_series<DB: DrawingBackend>(
        &self,
        length: usize,
        style: impl Into<ShapeStyle>,
    ) -> LineSeries<DB, (f32, f32)>;

    fn figure(label: impl Into<String>, points: [Vec2; N]) -> Figure<Self, N>
    where
        Self: Sized,
    {
        Figure::new(label.into(), points)
    }
}

impl Curve<3> for CubicBezier {
    fn from_points(points: [Vec2; 3]) -> Self {
        CubicBezier::new(points[0], points[1], points[2])
    }

    fn to_line_series<DB: DrawingBackend>(
        &self,
        length: usize,
        style: impl Into<ShapeStyle>,
    ) -> LineSeries<DB, (f32, f32)> {
        LineSeries::new(
            (0..length)
                .into_iter()
                .map(|i| i as f32 / length as f32)
                .map(|t| self.at(t))
                .map(|v| (v.x, v.y)),
            style,
        )
    }
}

pub struct Figure<C: Curve<N>, const N: usize> {
    label: String,
    points: [Vec2; N],
    curve: C,
    held: Option<usize>,
}

impl<C: Curve<N>, const N: usize> Figure<C, N> {
    fn new(label: String, points: [Vec2; N]) -> Self {
        Self {
            label,
            points,
            curve: C::from_points(points),
            held: None,
        }
    }

    fn draw<DB: DrawingBackend>(
        &self,
        chart: &mut PlotChartContext<DB>,
        style: impl Into<ShapeStyle>,
    ) -> Result<()>
    where
        DB::ErrorType: 'static,
    {
        let style = style.into();
        let style_clone = style.clone();

        chart
            .draw_series(self.curve.to_line_series(CURVE_LENGTH, style.clone()))?
            .label(self.label.clone())
            .legend(move |(x, y)| {
                Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], style_clone.clone())
            });

        chart.draw_series(
            self.points
                .iter()
                .map(|v| TriangleMarker::new((v.x, v.y), 8, style.filled())),
        )?;

        Ok(())
    }

    fn recurve(&mut self) {
        self.curve = C::from_points(self.points);
    }
}

pub struct FigureDrawer<'a, 'cc, DB: DrawingBackend> {
    figure_count: usize,
    chart: &'cc mut PlotChartContext<'a, DB>,
}

impl<'a, 'cc, DB: DrawingBackend> FigureDrawer<'a, 'cc, DB>
where
    DB::ErrorType: 'static,
{
    pub(super) fn new(chart: &'cc mut PlotChartContext<'a, DB>) -> Self {
        Self {
            figure_count: 0,
            chart,
        }
    }

    pub fn draw<C: Curve<N>, const N: usize>(&mut self, figure: &Figure<C, N>) -> Result<()> {
        figure.draw(self.chart, Palette99::pick(self.figure_count))?;

        self.figure_count += 1;
        Ok(())
    }
}

pub struct FigureUpdater {
    event: Event,
    mouse_pos: Option<Vec2>,
}

impl FigureUpdater {
    pub fn new(event: Event, mouse_pos: Option<Vec2>) -> Self {
        Self { event, mouse_pos }
    }

    pub fn update<C: Curve<N>, const N: usize>(&self, figure: &mut Figure<C, N>) {
        if let Some(mouse_pos) = self.mouse_pos {
            self.update_with_mouse(figure, mouse_pos);
        }
    }

    fn update_with_mouse<C: Curve<N>, const N: usize>(
        &self,
        figure: &mut Figure<C, N>,
        mouse_pos: Vec2,
    ) {
        const THRESHOLD: f32 = 10.0;
        if let Some(Button::Mouse(MouseButton::Left)) = self.event.release_args() {
            figure.held = None;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = self.event.press_args() {
            if figure.held.is_none() {
                figure.held = figure
                    .points
                    .iter()
                    .map(|&p| (p - mouse_pos).sqr_len())
                    .enumerate()
                    .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                    .filter(|&(_, d)| d < THRESHOLD * THRESHOLD)
                    .map(|(i, _)| i);
            }
        }

        if let Some(i) = figure.held {
            figure.points[i] = mouse_pos;
            figure.recurve();
        }
    }
}
