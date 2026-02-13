use std::f32::consts::PI;

use glam::{Vec3, Vec3A};
use num_complex::Complex;
use rand::{Rng, rngs::SmallRng};

use crate::colour::Colour;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metal {
    pub ior: Complex<f32>,
    pub roughness: f32,
}

impl Metal {
    pub fn sample_bsdf(&self, _outgoing: Vec3A, rng: &mut SmallRng) -> Vec3A {
        let (i, j): (f32, f32) = rng.random();
        let angle = 2.0 * PI * i;
        let radius = j.sqrt();
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let z = (1.0 - (x * x + y * y)).sqrt();
        let incoming: Vec3A = Vec3 { x, y, z }.into();
        incoming.normalize()
    }

    pub fn bsdf(&self, incoming: Vec3A, outgoing: Vec3A, colour: Colour) -> Colour {
        //torrance-sparrow model
        let micro_dir = (incoming - outgoing).normalize();
        let microfacets = self.microfacets(micro_dir);
        let masking = self.bidir_masking(incoming, outgoing);
        let fresnel = self.fresnel(outgoing);
        let cos_angle_i = incoming.angle_between(Vec3A::Z).cos();
        let cos_angle_o = outgoing.angle_between(-Vec3A::Z).cos();

        let result = colour * microfacets * masking * fresnel / (4.0 * cos_angle_i * cos_angle_o);

        //dbg!(microfacets, masking, fresnel);
        if cos_angle_o < 0.05 {
            dbg!(result);
        }

        result
    }

    pub fn pdf(&self, incoming: Vec3A, _outgoing: Vec3A) -> f32 {
        incoming.angle_between(Vec3A::Z).cos() / PI
    }

    fn fresnel(&self, outgoing: Vec3A) -> f32 {
        let angle = outgoing.angle_between(-Vec3A::Z);
        let cosangle_o = angle.cos();
        let cosangle_t = (1.0 - angle.sin().powi(2) / self.ior.powi(2)).sqrt();

        let parallel = (self.ior * cosangle_o - cosangle_t) / (self.ior * cosangle_o + cosangle_t);
        let perp = (cosangle_o - self.ior * cosangle_t) / (cosangle_o + self.ior * cosangle_t);

        let reflect_probability = 0.5 * (parallel.norm_sqr() + perp.norm_sqr());
        reflect_probability
    }

    // fn visible_microfacets(&self, micro_dir: Vec3A, view_dir: Vec3A) -> f32 {
    //     (self.microfacets(micro_dir) * self.masking(view_dir) * micro_dir.dot(view_dir).max(0.0))
    //         / view_dir.angle_between(Vec3A::Z).cos()
    // }

    fn microfacets(&self, micro_dir: Vec3A) -> f32 {
        //isotropic trowbridge-reitz distribution
        let angle = micro_dir.angle_between(Vec3A::Z);
        let rough_sq = self.roughness * self.roughness;
        let cos_angle_4 = angle.cos().powi(4);
        let tan_angle_2 = angle.tan().powi(2);

        //dbg!((angle, rough_sq, cos_angle_4, tan_angle_2));

        1.0 / (PI * rough_sq * cos_angle_4 * (1.0 + tan_angle_2 / rough_sq).powi(2))
    }

    // fn masking(&self, view_dir: Vec3A) -> f32 {
    //     1.0 / (1.0 + self.lambda(view_dir))
    // }

    fn bidir_masking(&self, incoming: Vec3A, outgoing: Vec3A) -> f32 {
        1.0 / (1.0 + self.lambda(incoming) + self.lambda(outgoing))
    }

    fn lambda(&self, view_dir: Vec3A) -> f32 {
        //smith's approximation for isotropic trowbridge-reitz distribution
        let angle = view_dir.angle_between(Vec3A::Z);
        let rough_sq = self.roughness * self.roughness;

        0.5 * ((1.0 + rough_sq * angle.tan().powi(2)).sqrt() - 1.0)
    }
}
