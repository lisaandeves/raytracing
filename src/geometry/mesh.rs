use glam::{Vec3, Vec3A};
use itertools::Itertools;
use tobj::Mesh;

use crate::{
    geometry::{Intersection, Ray, bounds::BoundingBox, meshbvh::MeshBVH, triangle::Triangle},
    materials::Material,
};

#[derive(Clone, Debug)]
pub struct TriangleMesh {
    triangle_bvh: MeshBVH,
    bbox: BoundingBox,
    pub centre: Vec3A,
    material: Material,
}

impl TriangleMesh {
    pub fn new(obj: &Mesh, material: Material, translate: Vec3A, scale: f32) -> Self {
        let triangles: Vec<Triangle> = obj
            .indices
            .iter()
            .tuples::<(_, _, _)>()
            .map(|indices| {
                let p0 = Vec3 {
                    x: scale * obj.positions[3 * *indices.0 as usize] + translate.x,
                    y: -scale * obj.positions[3 * *indices.0 as usize + 1] + translate.y,
                    z: scale * obj.positions[3 * *indices.0 as usize + 2] + translate.z,
                }
                .into();
                let p1 = Vec3 {
                    x: scale * obj.positions[3 * *indices.1 as usize] + translate.x,
                    y: -scale * obj.positions[3 * *indices.1 as usize + 1] + translate.y,
                    z: scale * obj.positions[3 * *indices.1 as usize + 2] + translate.z,
                }
                .into();
                let p2 = Vec3 {
                    x: scale * obj.positions[3 * *indices.2 as usize] + translate.x,
                    y: -scale * obj.positions[3 * *indices.2 as usize + 1] + translate.y,
                    z: scale * obj.positions[3 * *indices.2 as usize + 2] + translate.z,
                }
                .into();
                Triangle::new(p0, p1, p2)
            })
            .collect();

        let centre: Vec3A = triangles.iter().map(|tri| tri.centre()).sum();
        let centre = centre / triangles.len() as f32;

        let bbox = triangles
            .iter()
            .map(|object| object.bounding_box())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();

        let triangle_bvh = MeshBVH::new(triangles);

        Self {
            triangle_bvh,
            centre,
            bbox,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray, _t_max: f32) -> Option<Intersection> {
        if self.bbox.intersects(ray) {
            self.triangle_bvh
                .traverse(ray, &self.material)
                .iter()
                .cloned()
                .flatten()
                .reduce(Intersection::min)
        } else {
            None
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }
}
