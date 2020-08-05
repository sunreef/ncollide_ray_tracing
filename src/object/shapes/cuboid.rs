use nalgebra::Vector3;
use ncollide3d::shape;
use serde::{Deserialize, Serialize};

use crate::object::shapes::ObjectToShape;

#[derive(Serialize, Deserialize)]
pub struct Cuboid {
    dims: Vector3<f32>,
}

impl Cuboid {
    pub fn new(dims: Vector3<f32>) -> Self {
        Cuboid { dims }
    }
}

impl ObjectToShape for Cuboid {
    type ShapeType = shape::Cuboid<f32>;

    fn to_shape(self) -> Self::ShapeType {
        shape::Cuboid::new(self.dims)
    }
}
