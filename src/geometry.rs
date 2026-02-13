pub mod bounds;
pub mod mesh;
pub mod meshbvh;
pub mod object;
pub mod sphere;
pub mod triangle;

use glam::Vec3A;
use std::{fmt::Debug, rc::Rc};

use crate::materials::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Debug)]
pub struct Intersection {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub material: Rc<Material>,
    pub t: f32,
}

impl Intersection {
    pub fn min(self, other: Self) -> Self {
        if self.t < other.t { self } else { other }
    }
}
