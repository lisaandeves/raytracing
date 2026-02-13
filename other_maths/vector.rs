// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// use rand::{Rng, rngs::SmallRng};

// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub struct Vec3 {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
// }

// impl Vec3 {
//     pub fn zero() -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         }
//     }

//     pub fn x_axis() -> Self {
//         Self {
//             x: 1.0,
//             y: 0.0,
//             z: 0.0,
//         }
//     }

//     pub fn y_axis() -> Self {
//         Self {
//             x: 0.0,
//             y: 1.0,
//             z: 0.0,
//         }
//     }

//     pub fn z_axis() -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 1.0,
//         }
//     }

//     pub fn random(rng: &mut SmallRng) -> Self {
//         let (x, y, z) = rng.random();
//         Self { x, y, z }
//     }

//     pub fn dot(&self, other: &Self) -> f64 {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }

//     pub fn abs_sq(&self) -> f64 {
//         self.dot(self)
//     }

//     pub fn abs(&self) -> f64 {
//         self.abs_sq().sqrt()
//     }

//     pub fn unit(&self) -> Self {
//         *self / self.abs()
//     }

//     pub fn cross(&self, other: &Self) -> Self {
//         Self {
//             x: self.y * other.z - self.z * other.y,
//             y: self.z * other.x - self.x * other.z,
//             z: self.x * other.y - self.y - other.x,
//         }
//     }

//     pub fn angle(&self, other: &Self) -> f64 {
//         self.unit().dot(&other.unit()).acos()
//     }

//     pub fn project(&self, other: &Self) -> Self {
//         self.dot(&other.unit()) * other.unit()
//     }

//     pub fn reflect(&self, normal: &Self) -> Self {
//         *self - 2.0 * self.project(normal)
//     }

//     pub fn orient(&self, normal: &Self) -> Self {
//         let dot = Self::dot(self, normal);
//         if dot >= 0.0 {
//             *self
//         } else {
//             self.reflect(normal)
//         }
//     }

//     pub fn as_array(&self) -> [f64; 3] {
//         [self.x, self.y, self.z]
//     }
// }

// impl Add for Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

// impl AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//         self.z += rhs.z;
//     }
// }

// impl Sub for Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }

// impl SubAssign for Vec3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//         self.z -= rhs.z;
//     }
// }

// impl Mul<f64> for Vec3 {
//     type Output = Vec3;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Self {
//             x: rhs * self.x,
//             y: rhs * self.y,
//             z: rhs * self.z,
//         }
//     }
// }

// impl MulAssign<f64> for Vec3 {
//     fn mul_assign(&mut self, rhs: f64) {
//         self.x *= rhs;
//         self.y *= rhs;
//         self.z *= rhs;
//     }
// }

// impl Mul<Vec3> for f64 {
//     type Output = Vec3;

//     fn mul(self, rhs: Vec3) -> Self::Output {
//         rhs * self
//     }
// }

// impl Div<f64> for Vec3 {
//     type Output = Vec3;

//     fn div(self, rhs: f64) -> Self::Output {
//         (1.0 / rhs) * self
//     }
// }

// impl DivAssign<f64> for Vec3 {
//     fn div_assign(&mut self, rhs: f64) {
//         self.x /= rhs;
//         self.y /= rhs;
//         self.z /= rhs;
//     }
// }

// impl Neg for Vec3 {
//     type Output = Vec3;

//     fn neg(self) -> Self::Output {
//         Vec3 {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//         }
//     }
// }
