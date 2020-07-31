use nalgebra::{Point2, Vector2, Vector3};

use std::f32::consts::{FRAC_1_PI, PI};

use crate::math::angles_to_vector;

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
    pub fn sample(&self, input: &Point2<f32>, normal: &Vector3<f32>) -> (Vector3<f32>, f32) {
        let phi = input[0] * 2.0 * PI;
        let theta = input[1].acos();
        (angles_to_vector(phi, theta, normal), 0.5 * FRAC_1_PI)
    }
}

pub struct CosineWeightedHemisphereSampler;

impl CosineWeightedHemisphereSampler {
    pub fn sample(&self, input: &Point2<f32>, normal: &Vector3<f32>) -> (Vector3<f32>, f32) {
        let phi = input[0] * 2.0 * PI;
        let theta = 0.5 * ((1.0 - 2.0 * input[1]).acos());
        let cos_theta = theta.cos();
        (angles_to_vector(phi, theta, normal), cos_theta * FRAC_1_PI)
    }
}
