use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector from its coordinates.
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Returns the length (magnitude) of the vector.
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }

    /// Returns the _square_ of the length (magnitude) of the vector.
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the dot product of this vector with another vector.
    #[inline]
    pub fn dot(&self, b: &Vec2) -> f32 {
        self.x * b.x + self.y * b.y
    }

    /// Returns the normalized (unit) vector for the given vector.
    #[inline]
    pub fn normalized(self) -> Vec2 {
        let len = self.len();
        if len == 0.0 {
            Vec2::ZERO
        } else {
            self / len
        }
    }

    /// Returns a vector whose components are the minimum of the corresponding components
    /// of the two vectors.
    #[inline]
    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns a vector whose components are the maximum of the corresponding components
    /// of the two vectors.
    #[inline]
    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }

    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };

    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };

    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };

    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, s: f32) {
        *self = Vec2 {
            x: self.x * s,
            y: self.y * s,
        };
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, s: f32) -> Vec2 {
        self * (1.0 / s)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, s: f32) {
        *self *= 1.0 / s;
    }
}
