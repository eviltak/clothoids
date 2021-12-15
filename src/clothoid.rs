use crate::{ParametricCurve, Vec2};

const STEPS: usize = 1000;

pub struct Clothoid {
    dir: f32,
    end: Vec2,
    length: f32,
    a: f32,
    theta0: f32,
    lookup: [Vec2; STEPS],
}

impl Clothoid {
    pub fn new(theta0: f32, length: f32, end: Vec2, param: f32, clockwise: bool) -> Self {
        let mut clothoid = Self {
            dir: if clockwise { -1.0 } else { 1.0 },
            end,
            length,
            a: param,
            theta0,
            lookup: [Vec2::ZERO; STEPS],
        };

        clothoid.build_lookup();
        clothoid
    }

    fn build_lookup(&mut self) {

    }

    fn theta(&self, s: f32) -> f32 {
        self.theta0 + 0.5 * self.dir * s * s / (self.a * self.a)
    }
}

impl ParametricCurve for Clothoid {
    fn at(&self, s: f32) -> Vec2 {
        todo!()
    }

    fn tangent_at(&self, s: f32) -> Vec2 {
        let theta = self.theta(s);
        Vec2::new(theta.cos(), theta.sin())  
    }
}
