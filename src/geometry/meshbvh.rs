use glam::Vec3A;

use crate::{
    geometry::{Intersection, Ray, bounds::BoundingBox, triangle::Triangle},
    materials::Material,
};

#[derive(Clone, Debug, PartialEq)]
pub enum MeshBVHValue {
    Bbox(BoundingBox),
    Tri(Triangle),
    None,
}

#[derive(Clone, Debug)]
pub struct MeshBVHNode {
    value: MeshBVHValue,
    child1: Option<Box<MeshBVHNode>>,
    child2: Option<Box<MeshBVHNode>>,
}

impl MeshBVHNode {
    fn new(mut triangles: Vec<Triangle>) -> Self {
        if triangles.len() == 0 {
            Self {
                value: MeshBVHValue::Bbox(BoundingBox::new(Vec3A::ZERO, Vec3A::ZERO)),
                child1: None,
                child2: None,
            }
        } else if triangles.len() == 1 {
            let triangle = triangles.pop().unwrap();
            Self {
                value: MeshBVHValue::Tri(triangle),
                child1: None,
                child2: None,
            }
        } else {
            let bboxes: Vec<BoundingBox> = triangles.iter().map(|tri| tri.bounding_box()).collect();
            let value = bboxes
                .iter()
                .copied()
                .reduce(|acc, bbox| acc.union(&bbox))
                .unwrap();

            triangles.sort_by(|tri1, tri2| {
                let dimensions = value.dimensions().to_vec3();
                let maximum = dimensions.max_element();
                let centre1 = tri1.centre().to_vec3();
                let centre2 = tri2.centre().to_vec3();
                if dimensions.x == maximum {
                    centre1.x.total_cmp(&centre2.x)
                } else if dimensions.y == maximum {
                    centre1.y.total_cmp(&centre2.y)
                } else {
                    centre1.z.total_cmp(&centre2.z)
                }
            });

            let (child1, child2) = triangles.split_at(triangles.len() / 2);

            Self {
                value: MeshBVHValue::Bbox(value),
                child1: Some(Box::new(Self::new(child1.into()))),
                child2: Some(Box::new(Self::new(child2.into()))),
            }
        }
    }

    fn intersect(
        &self,
        intersections: &mut Vec<Option<Intersection>>,
        ray: &Ray,
        material: &Material,
    ) {
        match &self.value {
            MeshBVHValue::Tri(triangle) => {
                intersections.push(triangle.intersect(ray, 1000.0, material))
            }
            MeshBVHValue::Bbox(bbox) => {
                if bbox.intersects(ray) {
                    match self.child1.as_ref() {
                        Some(node) => node.intersect(intersections, ray, material),
                        None => {}
                    }
                    match self.child2.as_ref() {
                        Some(node) => node.intersect(intersections, ray, material),
                        None => {}
                    }
                }
            }
            MeshBVHValue::None => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeshBVH {
    root: MeshBVHNode,
}

impl MeshBVH {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let root = MeshBVHNode::new(triangles);

        Self { root }
    }

    pub fn traverse(&self, ray: &Ray, material: &Material) -> Vec<Option<Intersection>> {
        let mut intersections = Vec::new();
        self.root.intersect(&mut intersections, ray, material);
        intersections
    }
}

// #[derive(Clone, Debug)]
// pub struct CompactMeshBVHNode {
//     value: MeshBVHValue,
//     child_offset: usize,
// }

// impl CompactMeshBVHNode {
//     fn new(nodes: &mut Vec<Self>, node: &MeshBVHNode, offset: usize) -> usize {
//         match node.value {
//             MeshBVHValue::Tri(tri) => {
//                 nodes[offset] = Self {
//                     value: MeshBVHValue::Tri(tri),
//                     child_offset: 0,
//                 };

//                 offset + 1
//             }
//             MeshBVHValue::Bbox(bbox) => {
//                 let child_offset = Self::new(nodes, node.child1.as_ref().unwrap(), offset + 1);
//                 nodes[offset] = Self {
//                     value: MeshBVHValue::Bbox(bbox),
//                     child_offset: child_offset,
//                 };
//                 Self::new(nodes, node.child2.as_ref().unwrap(), child_offset)
//             }
//             MeshBVHValue::None => 0,
//         }
//     }

//     fn intersect(
//         &self,
//         nodes: &Vec<CompactMeshBVHNode>,
//         intersections: &mut Vec<Option<Intersection>>,
//         ray: &Ray,
//         material_id: usize,
//         offset: usize,
//     ) {
//         match &self.value {
//             MeshBVHValue::Tri(triangle) => {
//                 intersections.push(triangle.intersect(ray, 1000.0, material_id))
//             }
//             MeshBVHValue::Bbox(bbox) => {
//                 if bbox.intersects(ray) {
//                     nodes[offset + 1].intersect(nodes, intersections, ray, material_id, offset + 1);
//                     nodes[self.child_offset].intersect(
//                         nodes,
//                         intersections,
//                         ray,
//                         material_id,
//                         self.child_offset,
//                     );
//                 }
//             }
//             MeshBVHValue::None => {}
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct CompactMeshBVH {
//     nodes: Vec<CompactMeshBVHNode>,
// }

// impl CompactMeshBVH {
//     pub fn new(triangles: Vec<Triangle>) -> Self {
//         let mut nodes = vec![
//             CompactMeshBVHNode {
//                 value: MeshBVHValue::None,
//                 child_offset: 0
//             };
//             2 * triangles.len()
//         ];
//         let bvh = MeshBVH::new(triangles);
//         CompactMeshBVHNode::new(&mut nodes, &bvh.root, 0);

//         Self { nodes }
//     }

//     pub fn traverse(&self, ray: &Ray, material_id: usize) -> Vec<Option<Intersection>> {
//         let mut intersections = Vec::new();
//         self.nodes[0].intersect(&self.nodes, &mut intersections, ray, material_id, 0);
//         intersections
//     }
// }
