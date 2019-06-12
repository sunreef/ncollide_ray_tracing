use nalgebra::Vector3;
use std::f32::consts::FRAC_1_PI;

use crate::shaders::{BxDF, BRDF, BSDF};

#[derive(Clone, Debug)]
pub struct LambertBRDF {
    albedo: Vector3<f32>,
}

impl LambertBRDF {
    pub fn new(albedo: Vector3<f32>) -> Self {
        LambertBRDF { albedo }
    }
}

impl BxDF for LambertBRDF {
    fn sample(&self, dir1: Vector3<f32>, dir2: Vector3<f32>) -> Vector3<f32> {
        self.albedo * FRAC_1_PI
    }
}

impl BRDF for LambertBRDF {}

// ------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct LambertBSDF {
    brdf: LambertBRDF,
}

impl LambertBSDF {
    pub fn new(albedo: Vector3<f32>) -> Self {
        LambertBSDF {
            brdf: LambertBRDF::new(albedo),
        }
    }
}

impl BSDF for LambertBSDF {
    fn sample(&self, dir1: Vector3<f32>, dir2: Vector3<f32>) -> Vector3<f32> {
        self.brdf.sample(dir1, dir2)
    }
}
