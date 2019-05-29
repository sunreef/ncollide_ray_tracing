use nalgebra::{Point2, Vector2};

pub trait Sampler2 {
    fn sample(&self, input: &Point2<f32>) -> Point2<f32>;
}

pub struct UniformSampler2 {
    dims: Vector2<f32>,
}

impl Sampler2 for UniformSampler2 {
    fn sample(&self, input: &Point2<f32>) -> Point2<f32> {
        Point2::new(input[0] * self.dims[0], input[1] * self.dims[1])
    }
}

impl UniformSampler2 {
    pub fn new(dims: Vector2<f32>) -> UniformSampler2 {
        UniformSampler2 { dims }
    }
}
