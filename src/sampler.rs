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
        let phi = input[0] * 2.0 * PI;
        let theta = input[1].acos();

        let b3 = normal;
        let cross_dir = if b3[0] < 0.5 {
            Vector3::new(1.0,0.0,0.0)
        }
        else {
            Vector3::new(0.0,1.0,0.0)
        };
        let b1 = b3.cross(&cross_dir).normalize();
        let b2 = b3.cross(&b1);
//        println!("{}, {}, {}, {}", theta_n, phi_n, theta, phi);

        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let x = sin_theta * cos_phi;
        let y = sin_theta * sin_phi;
        let z = cos_theta;
        x * b1 + y * b2 + z * b3
    }
}
