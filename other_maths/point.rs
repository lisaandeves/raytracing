// use std::{
//     cmp::Ordering,
//     ops::{Add, Sub},
// };

// use crate::maths::vector::Vec3;

// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub struct Point3 {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
// }

// impl Point3 {
//     pub fn zero() -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         }
//     }

//     pub fn distance(&self, other: &Self) -> f64 {
//         (*self - *other).abs()
//     }

//     pub fn min_components(&self, other: &Self) -> Self {
//         Point3 {
//             x: f64::min(self.x, other.x),
//             y: f64::min(self.y, other.y),
//             z: f64::min(self.z, other.z),
//         }
//     }

//     pub fn max_components(&self, other: &Self) -> Self {
//         Point3 {
//             x: f64::max(self.x, other.x),
//             y: f64::max(self.y, other.y),
//             z: f64::max(self.z, other.z),
//         }
//     }
// }

// impl Add<Vec3> for Point3 {
//     type Output = Point3;

//     fn add(self, rhs: Vec3) -> Self::Output {
//         Point3 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

// impl Sub<Vec3> for Point3 {
//     type Output = Point3;

//     fn sub(self, rhs: Vec3) -> Self::Output {
//         self + (Vec3::zero() - rhs)
//     }
// }

// impl Sub for Point3 {
//     type Output = Vec3;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Vec3 {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }

// impl PartialOrd for Point3 {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if self.x < other.x && self.y < other.y && self.z < other.z {
//             Some(Ordering::Less)
//         } else if self.x > other.x && self.y > other.y && self.z > other.z {
//             Some(Ordering::Greater)
//         } else if self == other {
//             Some(Ordering::Equal)
//         } else {
//             None
//         }
//     }
// }
