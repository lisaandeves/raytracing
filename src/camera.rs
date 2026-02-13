use std::ops::Mul;

use glam::{Affine3A, Quat, Vec3, Vec3A};

use crate::geometry::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3A,
    pub rotation: Quat,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub focal_length: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3A,
        rotation: Quat,
        aspect_ratio: f32,
        fov: f32,
        focal_length: f32,
    ) -> Self {
        Self {
            origin,
            rotation,
            aspect_ratio,
            fov,
            focal_length,
        }
    }

    pub fn make_ray(&self, x: f32, y: f32) -> Ray {
        let height = self.fov.to_radians().mul(0.5).tan();
        let width = height * self.aspect_ratio;
        let x_dir = width * x * Vec3A::X;
        let y_dir = height * y * Vec3A::Y;
        let z_dir = -Vec3A::Z;
        let direction = (self.rotation * (x_dir + y_dir + z_dir)).normalize();
        Ray {
            origin: self.origin,
            direction,
        }
    }

    pub fn transform(&mut self, _matrix: Affine3A) {}
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Vec3 {
            x: 0.0,
            y: -1.0,
            z: 1.0,
        }
        .into();
        let rotation = Quat::from_rotation_x(0.2);
        let aspect_ratio = 16.0 / 9.0;
        let fov = 60.0;
        let focal_length = 1.0;

        Self::new(origin, rotation, aspect_ratio, fov, focal_length)
    }
}
