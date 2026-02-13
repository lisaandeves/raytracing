use glam::{Quat, Vec3A};
use itertools::Itertools;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;
use softbuffer::Surface;
use std::{rc::Rc, time::Instant};
use winit::{dpi::PhysicalSize, event_loop::OwnedDisplayHandle, window::Window};

use crate::{
    bvh::BVH,
    colour::Colour,
    geometry::{Intersection, Ray},
    materials::MaterialType,
    scene::Scene,
};

pub struct Renderer {
    size: PhysicalSize<u32>,
    pub scene: Scene,
    bvh: BVH,
    samples_per_pixel: u32,
    bounces_per_ray: u32,
}

impl Renderer {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        let scene = Scene::spheres();

        let now = Instant::now();

        let bvh = BVH::new(scene.shapes.clone());

        let elapsed = now.elapsed();
        println!("{:?} for bvh build", elapsed);

        let samples_per_pixel = 500;
        let bounces_per_ray = 5;
        Self {
            size,
            scene,
            bvh,
            samples_per_pixel,
            bounces_per_ray,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }

    pub fn render(&self, surface: &mut Surface<OwnedDisplayHandle, Rc<Window>>) {
        let now = Instant::now();

        let width = self.size.width as i32;
        let height = self.size.height as i32;

        let mut image = vec![Colour::black(); (width * height) as usize];

        for sample_num in 0..self.samples_per_pixel {
            let now = Instant::now();

            let new_image: Vec<Colour> = (0..height)
                .cartesian_product(0..width)
                .collect_vec()
                .into_par_iter()
                .map(|(y, x)| {
                    let mut rng = SmallRng::from_rng(&mut rand::rng());
                    self.render_pixel(x, y, &mut rng).gamma_correct()
                })
                .collect();

            image = image
                .iter()
                .cloned()
                .zip(new_image)
                .map(|(cur, new)| (sample_num as f32 * cur + new) / ((sample_num + 1) as f32))
                .collect();

            let to_buffer: Vec<u32> = image
                .iter()
                .map(|col| col.to_u8().map(|c| c as u32))
                .map(|[r, g, b]| b | (g << 8) | (r << 16))
                .collect();

            let mut buffer = surface.buffer_mut().unwrap();
            buffer.copy_from_slice(to_buffer.as_slice());
            buffer.present().unwrap();

            let elapsed = now.elapsed();
            if sample_num == 0 {
                println!("{:?} for one sample render", elapsed);
            }
        }

        let elapsed = now.elapsed();
        println!("{:?} for full render", elapsed);
    }

    fn render_pixel(&self, x: i32, y: i32, rng: &mut SmallRng) -> Colour {
        let (del_x, del_y): (f32, f32) = rng.random();
        let x = x as f32 + del_x - 0.5;
        let y = y as f32 + del_y - 0.5;
        let x_camera = 2.0 * x / self.size.width as f32 - 1.0;
        let y_camera = 2.0 * y / self.size.height as f32 - 1.0;

        let ray = self.scene.camera.make_ray(x_camera, y_camera);

        self.integrator(ray, self.bounces_per_ray, rng)
    }

    fn integrator(&self, ray: Ray, bounces: u32, rng: &mut SmallRng) -> Colour {
        // let now = Instant::now();

        let closest_intersection = self
            .bvh
            .traverse(&ray)
            .into_iter()
            .flatten()
            .reduce(Intersection::min);

        // let elapsed = now.elapsed();
        // println!("{:?} for intersections", elapsed);

        match closest_intersection {
            None => self.scene.sky_colour,
            Some(intersection) => {
                if bounces == 0 {
                    return Colour::black();
                }

                let material = intersection.material;
                if material.kind == MaterialType::Emissive {
                    return material.colour;
                }

                let intersection_frame = Quat::from_axis_angle(
                    intersection.normal.cross(Vec3A::Z).normalize().into(),
                    intersection.normal.angle_between(Vec3A::Z),
                );

                let outgoing = intersection_frame * ray.direction;
                let incoming = material.sample_bsdf(outgoing, rng);
                let bsdf = material.bsdf(incoming, outgoing);
                let pdf = material.pdf(incoming, outgoing);
                let cos_angle = incoming.angle_between(Vec3A::Z).cos();
                let epsilon = if cos_angle >= 0.0 { 0.001 } else { -0.001 };
                let incoming_ray = Ray {
                    origin: intersection.point + epsilon * intersection.normal,
                    direction: intersection_frame.conjugate() * incoming,
                };

                let rendering_equation =
                    bsdf * self.integrator(incoming_ray, bounces - 1, rng) * cos_angle / pdf;

                rendering_equation
            }
        }
    }
}
