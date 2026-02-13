// use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

// use crate::maths::{point::Point3, vector::Vec3};

// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub struct Matrix4x4(pub [[f64; 4]; 4]);

// impl Matrix4x4 {
//     pub fn identity() -> Self {
//         Matrix4x4([
//             [1.0, 0.0, 0.0, 0.0],
//             [0.0, 1.0, 0.0, 0.0],
//             [0.0, 0.0, 1.0, 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn zero() -> Self {
//         Matrix4x4([[0.0; 4]; 4])
//     }

//     pub fn translation(delta: Vec3) -> Self {
//         Matrix4x4([
//             [1.0, 0.0, 0.0, delta.x],
//             [0.0, 1.0, 0.0, delta.y],
//             [0.0, 0.0, 1.0, delta.z],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn rotation_x(theta: f64) -> Self {
//         Matrix4x4([
//             [1.0, 0.0, 0.0, 0.0],
//             [0.0, theta.cos(), -theta.sin(), 0.0],
//             [0.0, theta.sin(), theta.cos(), 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn rotation_y(theta: f64) -> Self {
//         Matrix4x4([
//             [theta.cos(), 0.0, theta.sin(), 0.0],
//             [0.0, 1.0, 0.0, 0.0],
//             [-theta.sin(), 0.0, theta.cos(), 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn rotation_z(theta: f64) -> Self {
//         Matrix4x4([
//             [theta.cos(), -theta.sin(), 0.0, 0.0],
//             [theta.sin(), theta.cos(), 0.0, 0.0],
//             [0.0, 0.0, 1.0, 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn scale(x: f64, y: f64, z: f64) -> Self {
//         Matrix4x4([
//             [x, 0.0, 0.0, 0.0],
//             [0.0, y, 0.0, 0.0],
//             [0.0, 0.0, z, 0.0],
//             [0.0, 0.0, 0.0, 1.0],
//         ])
//     }

//     pub fn test() -> Self {
//         Matrix4x4([
//             [-5.0, 2.0, 6.0, -8.0],
//             [1.0, -5.0, 1.0, 8.0],
//             [7.0, 7.0, -6.0, -7.0],
//             [1.0, -3.0, 7.0, 4.0],
//         ])
//     }

//     pub fn powi(&self, n: u32) -> Self {
//         let mut result = Self::identity();
//         let mut i = n;
//         while i > 0 {
//             result = result * *self;
//             i -= 1;
//         }
//         result
//     }

//     pub fn trace(&self) -> f64 {
//         self.0.iter().enumerate().map(|(i, row)| row[i]).sum()
//     }

//     pub fn determinant(&self) -> f64 {
//         fn det_2x2(m: [[f64; 2]; 2]) -> f64 {
//             m[0][0] * m[1][1] - m[1][0] * m[0][1]
//         }

//         fn det_3x3(m: [[f64; 3]; 3]) -> f64 {
//             let a00 = [[m[1][1], m[1][2]], [m[2][1], m[2][2]]];
//             let a01 = [[m[1][0], m[1][2]], [m[2][0], m[2][2]]];
//             let a02 = [[m[1][0], m[1][1]], [m[2][0], m[2][1]]];
//             m[0][0] * det_2x2(a00) - m[0][1] * det_2x2(a01) + m[0][2] * det_2x2(a02)
//         }

//         let a00 = [
//             [self[1][1], self[1][2], self[1][3]],
//             [self[2][1], self[2][2], self[2][3]],
//             [self[3][1], self[3][2], self[3][3]],
//         ];
//         let a01 = [
//             [self[1][0], self[1][2], self[1][3]],
//             [self[2][0], self[2][2], self[2][3]],
//             [self[3][0], self[3][2], self[3][3]],
//         ];
//         let a02 = [
//             [self[1][0], self[1][1], self[1][3]],
//             [self[2][0], self[2][1], self[2][3]],
//             [self[3][0], self[3][1], self[3][3]],
//         ];
//         let a03 = [
//             [self[1][0], self[1][1], self[1][2]],
//             [self[2][0], self[2][1], self[2][2]],
//             [self[3][0], self[3][1], self[3][2]],
//         ];

//         self[0][0] * det_3x3(a00) - self[0][1] * det_3x3(a01) + self[0][2] * det_3x3(a02)
//             - self[0][3] * det_3x3(a03)
//     }

//     ///Cayley-Hamilton method. Very slow!
//     pub fn inverse(&self) -> Self {
//         let a = self.clone();
//         let a2 = a * a;
//         let a3 = a2 * a;

//         let tr_a = a.trace();
//         let tr_a2 = a2.trace();
//         let tr_a3 = a3.trace();

//         let det_a = a.determinant();

//         let term_id = (1.0 / 6.0) * (tr_a * tr_a * tr_a - 3.0 * tr_a * tr_a2 + 2.0 * tr_a3);
//         let term_a = -(1.0 / 2.0) * (tr_a * tr_a - tr_a2);
//         let term_a2 = tr_a;
//         let term_a3 = -1.0;

//         (1.0 / det_a) * (term_id * Self::identity() + term_a * a + term_a2 * a2 + term_a3 * a3)
//     }

//     pub fn transpose(&self) -> Self {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 matrix[row][col] = self[col][row]
//             }
//         }

//         matrix
//     }
// }

// impl Add for Matrix4x4 {
//     type Output = Matrix4x4;

//     fn add(self, rhs: Self) -> Self::Output {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 matrix[row][col] = self[row][col] + rhs[row][col]
//             }
//         }

//         matrix
//     }
// }

// impl Sub for Matrix4x4 {
//     type Output = Matrix4x4;

//     fn sub(self, rhs: Self) -> Self::Output {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 matrix[row][col] = self[row][col] - rhs[row][col]
//             }
//         }
//         matrix
//     }
// }

// impl Neg for Matrix4x4 {
//     type Output = Matrix4x4;

//     fn neg(self) -> Self::Output {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 matrix[row][col] = -self[row][col]
//             }
//         }
//         matrix
//     }
// }

// impl Mul for Matrix4x4 {
//     type Output = Matrix4x4;

//     fn mul(self, rhs: Self) -> Self::Output {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 let mut sum = 0.0;
//                 for k in 0..4 {
//                     sum += self[row][k] * rhs[k][col]
//                 }
//                 matrix[row][col] = sum
//             }
//         }
//         matrix
//     }
// }

// impl Mul<f64> for Matrix4x4 {
//     type Output = Matrix4x4;

//     fn mul(self, rhs: f64) -> Self::Output {
//         let mut matrix = Self::zero();
//         for row in 0..4 {
//             for col in 0..4 {
//                 matrix[row][col] = self[row][col] * rhs
//             }
//         }
//         matrix
//     }
// }

// impl Mul<Vec3> for Matrix4x4 {
//     type Output = Vec3;

//     fn mul(self, rhs: Vec3) -> Self::Output {
//         Vec3 {
//             x: self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
//             y: self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
//             z: self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
//         }
//     }
// }

// impl Mul<Point3> for Matrix4x4 {
//     type Output = Point3;

//     fn mul(self, rhs: Point3) -> Self::Output {
//         let x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3];
//         let y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3];
//         let z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3];
//         let w = self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3];

//         Point3 {
//             x: x / w,
//             y: y / w,
//             z: z / w,
//         }
//     }
// }

// impl Mul<Matrix4x4> for f64 {
//     type Output = Matrix4x4;

//     fn mul(self, rhs: Matrix4x4) -> Self::Output {
//         rhs * self
//     }
// }

// impl Index<usize> for Matrix4x4 {
//     type Output = [f64; 4];

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.0[index]
//     }
// }

// impl IndexMut<usize> for Matrix4x4 {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.0[index]
//     }
// }
