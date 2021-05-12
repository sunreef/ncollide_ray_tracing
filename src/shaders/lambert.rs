use nalgebra::{Point2, Vector3};
use std::f32::consts::FRAC_1_PI;

use crate::sampling::CosineWeightedHemisphereSampler;
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
    fn eval(&self, v1: &Vector3<f32>, v2: &Vector3<f32>) -> Vector3<f32> {
        self.albedo * FRAC_1_PI
    }

    fn sample(
        &self,
        dir: &Vector3<f32>,
        samples: &Point2<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32) {
        let sampler = CosineWeightedHemisphereSampler;
        let (new_vector, probability) = sampler.sample(samples, &Vector3::new(0.0, 0.0, 1.0));
        let brdf_value = self.eval(dir, &new_vector);
        (new_vector, brdf_value, probability)
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
    fn eval(&self, dir1: &Vector3<f32>, dir2: &Vector3<f32>) -> Vector3<f32> {
        self.brdf.eval(dir1, dir2)
    }

    fn sample(
        &self,
        dir: &Vector3<f32>,
        samples: &Point2<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32) {
        self.brdf.sample(dir, samples)
    }
}
