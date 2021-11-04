
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    max.min(min.max(x))
}