use glam::{Vec3, Vec3A};

use crate::geometry::Ray;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinOrMax {
    Min,
    Max,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoundingBox {
    min: Vec3A,
    max: Vec3A,
}

impl BoundingBox {
    pub fn new(p1: Vec3A, p2: Vec3A) -> Self {
        BoundingBox {
            min: Vec3A::min(p1, p2),
            max: Vec3A::max(p1, p2),
        }
    }

    pub fn corner(&self, x: MinOrMax, y: MinOrMax, z: MinOrMax) -> Vec3A {
        let p_x = match x {
            MinOrMax::Min => self.min.x,
            MinOrMax::Max => self.max.x,
        };
        let p_y = match y {
            MinOrMax::Min => self.min.y,
            MinOrMax::Max => self.max.y,
        };
        let p_z = match z {
            MinOrMax::Min => self.min.z,
            MinOrMax::Max => self.max.z,
        };

        Vec3 {
            x: p_x,
            y: p_y,
            z: p_z,
        }
        .into()
    }

    pub fn extend(&self, point: Vec3A) -> Self {
        BoundingBox {
            min: Vec3A::min(self.min, point),
            max: Vec3A::max(self.max, point),
        }
    }

    pub fn union(&self, other: &BoundingBox) -> Self {
        BoundingBox {
            min: Vec3A::min(self.min, other.min),
            max: Vec3A::max(self.max, other.max),
        }
    }

    pub fn intersection(&self, other: &BoundingBox) -> Option<Self> {
        let min = Vec3A::min(self.min, other.min);
        let max = Vec3A::max(self.max, other.max);
        if min == Vec3A::min(min, max) {
            Some(BoundingBox { min, max })
        } else {
            None
        }
    }

    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        self.intersection(other).is_some()
    }

    pub fn contains(&self, point: &Vec3A) -> bool {
        self.min == Vec3A::min(*point, self.min) && self.max == Vec3A::max(*point, self.max)
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let min = ray.direction.signum().to_array().map(|sign| {
            if sign == 1.0 {
                MinOrMax::Min
            } else {
                MinOrMax::Max
            }
        });
        let max = ray.direction.signum().to_array().map(|sign| {
            if sign == 1.0 {
                MinOrMax::Max
            } else {
                MinOrMax::Min
            }
        });
        let min = self.corner(min[0], min[1], min[2]);
        let max = self.corner(max[0], max[1], max[2]);

        let t_min = (min - ray.origin) / ray.direction;
        let t_max = (max - ray.origin) / ray.direction;

        t_min.max_element() <= t_max.min_element() && t_max.max_element() > 0.0
    }

    pub fn dimensions(&self) -> Vec3A {
        self.max - self.min
    }

    pub fn centre(&self) -> Vec3A {
        0.5 * (self.min + self.max)
    }
}
