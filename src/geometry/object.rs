use glam::Vec3A;

use crate::geometry::{Intersection, Ray, bounds::BoundingBox, mesh::TriangleMesh, sphere::Sphere};

#[derive(Clone, Debug)]
pub enum ObjectType {
    Mesh(TriangleMesh),
    Sphere(Sphere),
}

#[derive(Clone, Debug)]
pub struct Object {
    kind: ObjectType,
}

impl Object {
    pub fn new(kind: ObjectType) -> Self {
        Self { kind }
    }

    pub fn intersect(&self, ray: &Ray, t_max: f32) -> Option<Intersection> {
        match &self.kind {
            ObjectType::Mesh(mesh) => mesh.intersect(ray, t_max),
            ObjectType::Sphere(sphere) => sphere.intersect(ray, t_max),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        match &self.kind {
            ObjectType::Mesh(mesh) => mesh.bounding_box(),
            ObjectType::Sphere(sphere) => sphere.bounding_box(),
        }
    }

    pub fn centre(&self) -> Vec3A {
        match &self.kind {
            ObjectType::Mesh(mesh) => mesh.centre,
            ObjectType::Sphere(sphere) => sphere.origin,
        }
    }
}
