use nalgebra::{Point2, Vector2, Vector3};

use std::f32::consts::PI;

pub struct UniformSampler2 {
    dims: Vector2<f32>,
}

impl UniformSampler2 {
    pub fn new(dims: Vector2<f32>) -> UniformSampler2 {
        UniformSampler2 { dims }
    }

    pub fn sample(&self, input: &Point2<f32>) -> Point2<f32> {
        Point2::new(input[0] * self.dims[0], input[1] * self.dims[1])
    }
}

pub struct HemisphereSampler;

impl HemisphereSampler {
    pub fn sample(&self, input: &Point2<f32>, normal: Vector3<f32>) -> Vector3<f32> {
        let theta_n = normal[2].asin();
        let phi_n = normal[1].atan2(normal[0]);

        let phi = input[0] * 2.0 * PI;
        let theta = (1.0 - input[1]).acos();

        let cos_theta = (theta + theta_n).cos();
        let sin_theta = (theta + theta_n).sin();
        let cos_phi = (phi + phi_n).cos();
        let sin_phi = (phi + phi_n).sin();
        Vector3::new(cos_theta * cos_phi, cos_theta * sin_phi, sin_theta)
    }
}
