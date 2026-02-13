use glam::{Affine3A, Vec3, Vec3A};
use num_complex::Complex;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use tobj::load_obj;

use crate::{
    camera::Camera,
    colour::Colour,
    geometry::{
        mesh::TriangleMesh,
        object::{Object, ObjectType},
        sphere::Sphere,
    },
    materials::{Material, MaterialType, diffuse::Diffuse, metal::Metal, specular::Specular},
};

pub enum InputMovement {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
    RotateLeft,
    RotateRight,
    RotateUp,
    RotateDown,
    RotateX(f32),
    RotateY(f32),
}

pub struct Scene {
    pub shapes: Vec<Object>,
    pub materials: Vec<Material>,
    pub camera: Camera,
    pub sky_colour: Colour,
}

impl Scene {
    pub fn spheres() -> Self {
        let camera = Camera::default();
        let sky_colour = Colour {
            r: 0.1,
            g: 0.2,
            b: 0.4,
        };
        let mut shapes = Vec::new();
        let mut materials = Vec::new();

        let floor_material = Material {
            kind: MaterialType::Diffuse(Diffuse {}),
            colour: [0.5, 0.5, 0.5].into(),
        };
        let floor = Sphere::new(
            1000.0,
            Vec3 {
                x: 0.0,
                y: 1000.0,
                z: -5.0,
            }
            .into(),
            floor_material,
        );
        materials.push(floor_material);
        shapes.push(Object::new(ObjectType::Sphere(floor)));

        let light_material = Material {
            kind: MaterialType::Emissive,
            colour: 20.0 * Colour::white(),
        };
        let light = Sphere::new(
            5.0,
            15.0 * Vec3A::X - 20.0 * Vec3A::Y - 5.0 * Vec3A::Z,
            light_material,
        );

        materials.push(light_material);
        shapes.push(Object::new(ObjectType::Sphere(light)));

        let mut rng = SmallRng::from_rng(&mut rand::rng());
        for i in 0..22 {
            for j in 0..22 {
                let choose_material: f32 = rng.random();
                let (a, b): (f32, f32) = rng.random();
                let origin = Vec3 {
                    x: i as f32 - 11.0 + 0.9 * a,
                    y: -0.2,
                    z: -(j as f32) - 0.9 * b,
                }
                .into();
                let colour = Colour::random(&mut rng);

                let material = if choose_material <= 0.75 {
                    Material {
                        kind: MaterialType::Diffuse(Diffuse {}),
                        colour,
                    }
                } else if choose_material <= 0.87 {
                    Material {
                        kind: MaterialType::Reflective,
                        colour,
                    }
                } else {
                    Material {
                        kind: MaterialType::Specular(Specular { ior: 1.5 }),
                        colour,
                    }
                };

                let sphere = Sphere::new(0.2, origin, material);

                materials.push(material);
                shapes.push(Object::new(ObjectType::Sphere(sphere)));
            }
        }

        Self {
            camera,
            shapes,
            materials,
            sky_colour,
        }
    }

    pub fn bunny() -> Self {
        let camera = Camera::default();
        let sky_colour = Colour {
            r: 0.5,
            g: 0.8,
            b: 0.9,
        };
        let mut shapes = Vec::new();
        let mut materials = Vec::new();

        let floor_material = Material {
            colour: [0.5, 0.5, 0.5].into(),
            kind: MaterialType::Reflective,
        };
        let floor = Sphere::new(
            1000.0,
            Vec3 {
                x: 0.0,
                y: 1000.0,
                z: -5.0,
            }
            .into(),
            floor_material,
        );
        materials.push(floor_material);
        shapes.push(Object::new(ObjectType::Sphere(floor)));

        let bunny_material = Material {
            kind: MaterialType::Reflective,
            colour: Colour {
                r: 1.0,
                g: 0.4,
                b: 0.7,
            },
        };

        let bunny_material2 = Material {
            kind: MaterialType::Diffuse(Diffuse {}),
            colour: Colour {
                r: 1.0,
                g: 0.4,
                b: 0.7,
            },
        };

        let (bunny_obj, _) = load_obj(
            "assets/stanford-bunny.obj",
            &tobj::OFFLINE_RENDERING_LOAD_OPTIONS,
        )
        .unwrap();

        let bunny = TriangleMesh::new(
            &bunny_obj[0].mesh,
            bunny_material,
            -3.0 * Vec3A::Z + 0.8 * Vec3A::Y,
            10.0,
        );

        let bunny2 = TriangleMesh::new(
            &bunny_obj[0].mesh,
            bunny_material2,
            -3.0 * Vec3A::Z + 0.8 * Vec3A::Y - 2.0 * Vec3A::X,
            10.0,
        );

        let bunny3 = TriangleMesh::new(
            &bunny_obj[0].mesh,
            bunny_material2,
            -3.0 * Vec3A::Z + 0.8 * Vec3A::Y + 2.0 * Vec3A::X,
            10.0,
        );

        materials.push(bunny_material);
        shapes.push(Object::new(ObjectType::Mesh(bunny)));
        shapes.push(Object::new(ObjectType::Mesh(bunny2)));
        shapes.push(Object::new(ObjectType::Mesh(bunny3)));

        let light_material = Material {
            kind: MaterialType::Emissive,
            colour: Colour::white(),
        };

        let light = Sphere::new(0.2, -2.0 * Vec3A::Y - 2.5 * Vec3A::Z, light_material);

        materials.push(light_material);
        shapes.push(Object::new(ObjectType::Sphere(light)));

        Self {
            camera,
            shapes,
            materials,
            sky_colour,
        }
    }

    // pub fn cornell_box() -> Self {
    //     let sky_colour = Colour {
    //         r: 0.5,
    //         g: 0.8,
    //         b: 0.9,
    //     };
    //     let camera = Camera {
    //         origin: 3.5 * Vec3A::Z - 3.0 * Vec3A::Y,
    //         rotation: Quat::IDENTITY,
    //         aspect_ratio: 16.0 / 9.0,
    //         fov: 60.0,
    //         focal_length: 1.0,
    //     };
    //     let mut shapes = Vec::new();
    //     let mut materials = Vec::new();

    //     let (objects, _) = load_obj("assets/cornell-box.obj", &tobj::GPU_LOAD_OPTIONS).unwrap();

    //     let meshes: Vec<Object> = objects
    //         .iter()
    //         .enumerate()
    //         .map(|(i, obj)| {
    //             Object::new(ObjectType::Mesh(TriangleMesh::new(
    //                 &obj.mesh,
    //                 i,
    //                 Vec3A::ZERO,
    //                 1.0,
    //             )))
    //         })
    //         .collect();

    //     let light_material = Material {
    //         kind: MaterialType::Emissive,
    //         colour: Colour {
    //             r: 1.0,
    //             g: 1.0,
    //             b: 1.0,
    //         },
    //     };

    //     materials.push(light_material);

    //     let back_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 1.0,
    //             g: 1.0,
    //             b: 1.0,
    //         },
    //     };

    //     materials.push(back_material);

    //     let ceiling_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 1.0,
    //             g: 1.0,
    //             b: 1.0,
    //         },
    //     };

    //     materials.push(ceiling_material);

    //     let floor_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 1.0,
    //             g: 1.0,
    //             b: 1.0,
    //         },
    //     };

    //     materials.push(floor_material);

    //     let left_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 1.0,
    //             g: 0.0,
    //             b: 0.0,
    //         },
    //     };

    //     materials.push(left_material);

    //     let right_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 0.0,
    //             g: 1.0,
    //             b: 0.0,
    //         },
    //     };

    //     materials.push(right_material);

    //     let short_box_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 0.7,
    //             g: 0.7,
    //             b: 0.7,
    //         },
    //     };

    //     materials.push(short_box_material);

    //     let tall_box_material = Material {
    //         kind: MaterialType::Diffuse(Diffuse {}),
    //         colour: Colour {
    //             r: 0.5,
    //             g: 0.5,
    //             b: 0.5,
    //         },
    //     };

    //     materials.push(tall_box_material);

    //     shapes.extend(meshes);

    //     Self {
    //         camera,
    //         shapes,
    //         materials,
    //         sky_colour,
    //     }
    // }

    pub fn move_camera(&mut self, direction: InputMovement) {
        let matrix = match direction {
            InputMovement::Left => Affine3A::from_translation(Vec3 {
                x: -0.05,
                y: 0.0,
                z: 0.0,
            }),
            InputMovement::Right => Affine3A::from_translation(Vec3 {
                x: 0.05,
                y: 0.0,
                z: 0.0,
            }),
            InputMovement::Up => Affine3A::from_translation(Vec3 {
                x: 0.0,
                y: -0.05,
                z: 0.0,
            }),
            InputMovement::Down => Affine3A::from_translation(Vec3 {
                x: 0.0,
                y: 0.05,
                z: 0.0,
            }),
            InputMovement::Forward => Affine3A::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -0.05,
            }),
            InputMovement::Backward => Affine3A::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.05,
            }),
            InputMovement::RotateLeft => Affine3A::from_rotation_y(0.05),
            InputMovement::RotateRight => Affine3A::from_rotation_y(-0.05),
            InputMovement::RotateUp => Affine3A::from_rotation_x(-0.05),
            InputMovement::RotateDown => Affine3A::from_rotation_x(0.05),
            InputMovement::RotateX(theta) => Affine3A::from_rotation_y(-theta),
            InputMovement::RotateY(theta) => Affine3A::from_rotation_x(theta),
        };

        self.camera.transform(matrix);
    }
}
