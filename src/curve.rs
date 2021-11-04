use crate::Vec2;

pub trait ParametricCurve {
    fn at(&self, t: f32) -> Vec2;
    fn tangent_at(&self, t: f32) -> Vec2;
}