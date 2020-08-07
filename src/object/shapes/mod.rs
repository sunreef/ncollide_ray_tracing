mod trimesh;
use trimesh::*;

mod cuboid;
use cuboid::*;

mod metaball;
use metaball::*;

use nalgebra::{Point3, Vector3};
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
    Metaball(Vec<Point3<f32>>),
}

impl Shape {
    pub fn get_handle(self) -> ShapeHandle<f32> {
        match self {
            Shape::TriMesh(obj_path) => ShapeHandle::new(TriMesh::new(obj_path).to_shape()),
            Shape::Cuboid(dims) => ShapeHandle::new(Cuboid::new(dims).to_shape()),
            Shape::Metaball(points) => ShapeHandle::new(Metaball::new(points, 1.0f32, 1.0f32)),
        }
    }
}
