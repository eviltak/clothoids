use clothoids::Vec2;
use util::CubicBezier;
use util::{Window, Curve};

mod util;

fn main() {
    let mut window = Window::new("Clothoid Transitions", [800, 800], [100.0, 100.0]);

    let mut bezier1 = CubicBezier::figure(
        "Bezier 1",
        [Vec2::ZERO, Vec2::ONE * 25.0, Vec2::RIGHT * 50.0],
    );

    while let Some(_) = window.draw(|drawer| {
        drawer.draw(&bezier1)?;

        Ok(())
    }) {}
}
