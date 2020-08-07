use nalgebra::{Point2, Vector3};

use crate::shaders::{BxDF, BRDF, BSDF};

#[derive(Clone, Debug)]
pub struct MirrorBRDF {}

impl MirrorBRDF {
    pub fn new() -> Self {
        MirrorBRDF {}
    }
}

impl BxDF for MirrorBRDF {
    fn eval(&self, _: &Vector3<f32>, _: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn sample(
        &self,
        dir: &Vector3<f32>,
        samples: &Point2<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32) {
        let new_dir = Vector3::new(dir[0], dir[1], -dir[2]);
        let brdf_value = Vector3::new(1.0, 1.0, 1.0);
        let probability = 1.0;
        (new_dir, brdf_value, probability)
    }
}

impl BRDF for MirrorBRDF {}

// ------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MirrorBSDF {
    brdf: MirrorBRDF,
}

impl MirrorBSDF {
    pub fn new() -> Self {
        MirrorBSDF {
            brdf: MirrorBRDF::new(),
        }
    }
}

impl BSDF for MirrorBSDF {
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

    fn is_diffuse(&self) -> bool {
        false
    }
}
