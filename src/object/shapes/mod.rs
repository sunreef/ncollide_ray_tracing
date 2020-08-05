mod trimesh;
use trimesh::*;

mod cuboid;
use cuboid::*;

use nalgebra::Vector3;
use ncollide3d::shape::ShapeHandle;
use serde::{Deserialize, Serialize};

pub trait ObjectToShape {
    type ShapeType;
    fn to_shape(self) -> Self::ShapeType;
}

#[derive(Serialize, Deserialize)]
pub enum Shape {
    TriMesh(String),
    Cuboid(Vector3<f32>),
}

impl Shape {
    pub fn get_handle(self) -> ShapeHandle<f32> {
        match self {
            Shape::TriMesh(obj_path) => ShapeHandle::new(TriMesh::new(obj_path).to_shape()),
            Shape::Cuboid(dims) => ShapeHandle::new(Cuboid::new(dims).to_shape()),
        }
    }
}
