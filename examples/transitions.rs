use clothoids::Vec2;
use util::{CubicBezier, Curve, PlotWindow};

mod util;

fn main() {
    let mut window = PlotWindow::new("Clothoid Transitions", [800, 800], [100.0, 100.0]);

    let mut bezier1 = CubicBezier::figure(
        "Bezier 1",
        [Vec2::ZERO, Vec2::ONE * 10.0, Vec2::RIGHT * 20.0].map(|v| Vec2::ONE + v),
    );

    let mut bezier2 = CubicBezier::figure(
        "Bezier 2",
        [Vec2::ZERO, Vec2::ONE * 10.0, Vec2::RIGHT * 20.0].map(|v| Vec2::ONE * 70.0 - v),
    );

    while let Some(updater) = window.draw(|drawer| {
        drawer.draw(&bezier1)?;
        drawer.draw(&bezier2)?;

        Ok(())
    }) {
        updater.update(&mut bezier1);
        updater.update(&mut bezier2);
    }
}
