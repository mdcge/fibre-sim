use std::ops::{Mul, Add, Neg, Div, Sub, AddAssign, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(a: f32, b: f32) -> Self {
        Vec2 { x: a, y: b }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn unit(&self) -> Self {
        let mag = self.mag();
        let unit_vec = match mag {
            0.0 => Vec2 { x: 0.0, y: 0.0 },
            _ => *self / mag,
        };
        unit_vec
    }
}


impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self {
        Vec2 { x: self.x+rhs.x, y: self.y+rhs.y }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self {
        Vec2 { x: self.x-rhs.x, y: self.y-rhs.y }
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec2 { x: self.x*rhs, y: self.y*rhs }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: rhs.x*self, y: rhs.y*self }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Vec2 { x: self.x/rhs, y: self.y/rhs }
    }
}
