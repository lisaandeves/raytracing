use glam::Vec3A;

use crate::geometry::{Intersection, Ray, bounds::BoundingBox, object::Object};

#[derive(Clone, Debug)]
pub enum BVHValue {
    Bbox(BoundingBox),
    Object(Object),
}

#[derive(Clone, Debug)]
pub struct BVHNode {
    value: BVHValue,
    child1: Option<Box<BVHNode>>,
    child2: Option<Box<BVHNode>>,
}

impl BVHNode {
    fn new(mut objects: Vec<Object>) -> Self {
        if objects.len() == 0 {
            Self {
                value: BVHValue::Bbox(BoundingBox::new(Vec3A::ZERO, Vec3A::ZERO)),
                child1: None,
                child2: None,
            }
        } else if objects.len() == 1 {
            let object = objects.pop().unwrap();
            let bbox = object.bounding_box();
            Self {
                value: BVHValue::Bbox(bbox),
                child1: Some(Box::new(BVHNode {
                    value: BVHValue::Object(object),
                    child1: None,
                    child2: None,
                })),
                child2: None,
            }
        } else {
            let bboxes: Vec<BoundingBox> =
                objects.iter().map(|object| object.bounding_box()).collect();
            let value = bboxes
                .iter()
                .copied()
                .reduce(|acc, bbox| acc.union(&bbox))
                .unwrap();

            objects.sort_by(|obj1, obj2| {
                let dimensions = value.dimensions().to_vec3();
                let maximum = dimensions.max_element();
                let centre1 = obj1.centre().to_vec3();
                let centre2 = obj2.centre().to_vec3();
                if dimensions.x == maximum {
                    centre1.x.total_cmp(&centre2.x)
                } else if dimensions.y == maximum {
                    centre1.y.total_cmp(&centre2.y)
                } else {
                    centre1.z.total_cmp(&centre2.z)
                }
            });

            let (child1, child2) = objects.split_at(objects.len() / 2);

            Self {
                value: BVHValue::Bbox(value),
                child1: Some(Box::new(Self::new(child1.into()))),
                child2: Some(Box::new(Self::new(child2.into()))),
            }
        }
    }

    fn intersect(&self, intersections: &mut Vec<Option<Intersection>>, ray: &Ray) {
        match &self.value {
            BVHValue::Object(object) => intersections.push(object.intersect(ray, 1000.0)),
            BVHValue::Bbox(bbox) => {
                if bbox.intersects(ray) {
                    match self.child1.as_ref() {
                        Some(node) => node.intersect(intersections, ray),
                        None => {}
                    }
                    match self.child2.as_ref() {
                        Some(node) => node.intersect(intersections, ray),
                        None => {}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BVH {
    root: BVHNode,
}

impl BVH {
    pub fn new(objects: Vec<Object>) -> Self {
        let root = BVHNode::new(objects);

        Self { root }
    }

    pub fn traverse(&self, ray: &Ray) -> Vec<Option<Intersection>> {
        let mut intersections = Vec::new();
        self.root.intersect(&mut intersections, ray);

        intersections
    }
}
