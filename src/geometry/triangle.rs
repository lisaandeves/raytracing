use std::rc::Rc;

use glam::Vec3A;

use crate::{
    geometry::{Intersection, Ray, bounds::BoundingBox},
    materials::Material,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Triangle {
    p0: Vec3A,
    p1: Vec3A,
    p2: Vec3A,
}

impl Triangle {
    pub fn new(p0: Vec3A, p1: Vec3A, p2: Vec3A) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn intersect(&self, ray: &Ray, t_max: f32, material: &Material) -> Option<Intersection> {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        let dir_cross_e2 = ray.direction.cross(edge2);
        let det = dir_cross_e2.dot(edge1);

        if det < f32::EPSILON && det > -f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let origin = ray.origin - self.p0;
        let u = inv_det * dir_cross_e2.dot(origin);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let origin_cross_e1 = origin.cross(edge1);
        let v = inv_det * origin_cross_e1.dot(ray.direction);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * origin_cross_e1.dot(edge2);

        if t < 0.0 || t > t_max {
            return None;
        }

        //assumes triangle vertices are stored in counter-clockwise order
        let normal = -edge1.cross(edge2).normalize();

        Some(Intersection {
            point: ray.at(t),
            normal,
            material: Rc::new(*material),
            t,
        })
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.p0, self.p1).extend(self.p2)
    }

    pub fn centre(&self) -> Vec3A {
        0.33333 * (self.p0 + self.p1 + self.p2)
    }
}
