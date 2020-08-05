use nalgebra::{Point2, Vector3};
use serde::{Deserialize, Serialize};

pub trait BxDF {
    fn eval(&self, dir1: &Vector3<f32>, dir2: &Vector3<f32>) -> Vector3<f32>;

    fn sample(
        &self,
        dir: &Vector3<f32>,
        samples: &Point2<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32);
}

pub trait BRDF: BxDF {}

pub trait BTDF: BxDF {}

pub trait BSDF: Send + Sync {
    fn eval(&self, dir1: &Vector3<f32>, dir2: &Vector3<f32>) -> Vector3<f32>;

    fn sample(
        &self,
        dir: &Vector3<f32>,
        samples: &Point2<f32>,
    ) -> (Vector3<f32>, Vector3<f32>, f32);
}

pub mod lambert;

#[derive(Serialize, Deserialize)]
pub enum Shader {
    Lambert(Vector3<f32>),
}

impl Shader {
    pub fn to_bsdf(self) -> Box<dyn BSDF> {
        match self {
            Shader::Lambert(albedo) => Box::new(lambert::LambertBSDF::new(albedo)),
        }
    }
}
