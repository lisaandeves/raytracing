use std::rc::Rc;

use glam::Vec3A;

use crate::{
    geometry::{Intersection, Ray, bounds::BoundingBox},
    materials::Material,
};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub origin: Vec3A,
    material: Material,
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32, origin: Vec3A, material: Material) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray, t_max: f32) -> Option<Intersection> {
        let a = ray.direction.length_squared();
        let b = -2.0 * ray.direction.dot(self.origin - ray.origin);
        let c = (self.origin - ray.origin).length_squared() - self.radius.powi(2);
        let discrim = b * b - 4.0 * a * c;

        if discrim < 0.0 {
            return None;
        }

        let t = (-b - discrim.sqrt()) / (2.0 * a);

        if t > t_max || t < 0.0 {
            return None;
        }

        let point = ray.at(t);
        let normal = (point - self.origin).normalize();

        Some(Intersection {
            point,
            normal,
            material: Rc::new(self.material),
            t,
        })
    }

    pub fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            self.origin - self.radius * Vec3A::ONE,
            self.origin + self.radius * Vec3A::ONE,
        )
    }
}
