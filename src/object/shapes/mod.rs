use nalgebra::{Point2, Point3, Vector3};
use ncollide3d::shape::ShapeHandle;
use serde::{Deserialize, Serialize};

pub trait ObjectToShape {
    type ShapeType;
    fn to_shape(self) -> Self::ShapeType;
}

pub trait SampleShape {
    fn sample(&self, samples: &Point2<f32>) -> Vector3<f32>;
}

mod ball;
mod cuboid;
mod metaball;
mod trimesh;

#[derive(Serialize, Deserialize)]
pub enum Shape {
    TriMesh(String),
    Cuboid(Vector3<f32>),
    Ball(f32),
    Metaball(Vec<Point3<f32>>),
}

impl Shape {
    pub fn get_handle(self) -> ShapeHandle<f32> {
        match self {
            Shape::TriMesh(obj_path) => {
                ShapeHandle::new(trimesh::TriMesh::new(obj_path).to_shape())
            }
            Shape::Cuboid(dims) => ShapeHandle::new(cuboid::Cuboid::new(dims).to_shape()),
            Shape::Ball(radius) => ShapeHandle::new(ball::Ball::new(radius).to_shape()),
            Shape::Metaball(points) => {
                ShapeHandle::new(metaball::Metaball::new(points, 1.0f32, 1.0f32))
            }
        }
    }
}
