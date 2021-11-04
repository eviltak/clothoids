use clothoids::{util, ParametricCurve, Vec2};

pub struct CubicBezier {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
}

impl CubicBezier {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> CubicBezier {
        let d = (c - a).len();
        let da = (b - a).normalized();
        let db = (b - c).normalized();

        let u = util::clamp((a - b).len(), 0.1 * d, d);
        let v = util::clamp((c - b).len(), 0.1 * d, d);

        let s = u + v;

        let u = u.min(0.3 * s);
        let v = v.min(0.3 * s);

        CubicBezier {
            p0: a,
            p1: a + u * da,
            p2: c + v * db,
            p3: c,
        }
    }

    fn second_derivative_at(&self, t: f32) -> Vec2 {
        6.0 * (1.0 - t) * (self.p2 - 2.0 * self.p1 + self.p0)
            + 6.0 * t * (self.p3 - 2.0 * self.p2 + self.p1)
    }
}

impl ParametricCurve for CubicBezier {
    fn at(&self, t: f32) -> Vec2 {
        (1.0 - t) * (1.0 - t) * (1.0 - t) * self.p0
            + 3.0 * t * (1.0 - t) * (1.0 - t) * self.p1
            + 3.0 * t * t * (1.0 - t) * self.p2
            + t * t * t * self.p3
    }

    fn tangent_at(&self, t: f32) -> Vec2 {
        3.0 * (1.0 - t) * (1.0 - t) * (self.p1 - self.p0)
            + 6.0 * (1.0 - t) * t * (self.p2 - self.p1)
            + 3.0 * t * t * (self.p3 - self.p2)
    }
}
