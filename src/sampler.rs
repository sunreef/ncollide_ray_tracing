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

pub struct UniformSphereSampler;

impl UniformSphereSampler {
    pub fn sample(&self, samples: &Point2<f32>) -> (Vector3<f32>, f32) {
        let phi = 2.0 * PI * samples[0];
        let theta = (1.0 - 2.0 * samples[1]).acos();
        let sin_theta = theta.sin();
        (
            Vector3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, theta.cos()),
            0.25 * FRAC_1_PI,
        )
    }
}

pub struct UniformHemisphereSampler;

impl UniformHemisphereSampler {
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
