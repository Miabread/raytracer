use crate::vec3::{Arrow, Point};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Arrow,
}

impl Ray {
    pub fn new(origin: Point, direction: Arrow) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
