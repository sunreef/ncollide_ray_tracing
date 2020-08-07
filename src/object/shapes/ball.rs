use nalgebra::{Point2, Point3};
use ncollide3d::shape;
use serde::{Deserialize, Serialize};

use crate::object::shapes::{ObjectToShape, SampleShape};
use crate::sampler::UniformSphereSampler;

#[derive(Serialize, Deserialize)]
pub struct Ball {
    radius: f32,
}

impl Ball {
    pub fn new(radius: f32) -> Self {
        Ball { radius }
    }
}

impl ObjectToShape for Ball {
    type ShapeType = shape::Ball<f32>;

    fn to_shape(self) -> Self::ShapeType {
        shape::Ball::new(self.radius)
    }
}

impl SampleShape for Ball {
    fn sample(&self, samples: &Point2<f32>) -> Point3<f32> {
        let sphere_sampler = UniformSphereSampler;
        let (vector, probability) = sphere_sampler.sample(samples);
        self.radius * Point3::from(vector)
    }
}
