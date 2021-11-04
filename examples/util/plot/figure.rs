use std::error::Error;

use piston_window::{Event, EventLoop, PistonWindow, WindowSettings};
use plotters::coord::types::RangedCoordf32;
use plotters::element::PointElement;
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};

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
}

impl<C: Curve<N>, const N: usize> Figure<C, N> {
    fn new(label: String, points: [Vec2; N]) -> Self {
        Self {
            label,
            points,
            curve: C::from_points(points),
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

        let style_clone = style.clone();

        chart
            .draw_series(PointSeries::of_element(
                self.points.iter().map(|v| (v.x, v.y)),
                10,
                style,
                &TriangleMarker::make_point,
            ))?;

        Ok(())
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

    pub fn draw<const N: usize>(&mut self, figure: &Figure<impl Curve<N>, N>) -> Result<()> {
        figure.draw(self.chart, Palette99::pick(self.figure_count))?;

        self.figure_count += 1;
        Ok(())
    }
}
