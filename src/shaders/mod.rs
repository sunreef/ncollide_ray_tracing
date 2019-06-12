use nalgebra::Vector3;

pub trait BxDF {
    fn sample(&self, dir1: Vector3<f32>, dir2: Vector3<f32>) -> Vector3<f32>;
}

pub trait BRDF: BxDF {}

pub trait BTDF: BxDF {}

pub trait BSDF: Send + Sync {
    fn sample(&self, dir1: Vector3<f32>, dir2: Vector3<f32>) -> Vector3<f32>;
}

pub mod lambert;
