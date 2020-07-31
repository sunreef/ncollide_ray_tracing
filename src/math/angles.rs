use nalgebra::Vector3;

use crate::math::Vector3ToBasis;

pub fn angles_to_vector(phi: f32, theta: f32, normal: &Vector3<f32>) -> Vector3<f32> {
    let (b1, b2, b3) = normal.orthonormal_basis();
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();
    let x = sin_theta * cos_phi;
    let y = sin_theta * sin_phi;
    let z = cos_theta;
    x * b1 + y * b2 + z * b3
}
