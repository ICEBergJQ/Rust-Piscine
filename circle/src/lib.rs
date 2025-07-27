use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    pub fn intersect(&self, c: Circle) -> bool {
        let distance = self.center.distance(c.center);
        distance <= (self.radius + c.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, p: Point) -> f64 {
        ((self.0 - p.0).powi(2) + (self.1 - p.1).powi(2)).sqrt()
    }
}
