use nalgebra::Vector3;

pub trait BRDF: Send + Sync {
    fn sample(&self, dir: Vector3<f32>) -> f32;
}