use std::path::Path;

use nalgebra::Point3;
use ncollide3d::shape;
use obj::{Obj, SimplePolygon};
use serde::{Deserialize, Serialize};

use crate::object::shapes::ObjectToShape;

#[derive(Serialize, Deserialize)]
pub struct TriMesh {
    vertices: Vec<Point3<f32>>,
    indices: Vec<Point3<usize>>,
}

impl TriMesh {
    pub fn new(path: String) -> Self {
        let mesh = Obj::<SimplePolygon>::load(Path::new(&path)).unwrap();
        let vertices = mesh
            .position
            .iter()
            .map(|p| Point3::from_slice(p))
            .collect::<Vec<_>>();
        let mut indices = Vec::new();
        for object in mesh.objects {
            for group in object.groups {
                for poly in group.polys {
                    let point = Point3::new(poly[0].0, poly[1].0, poly[2].0);
                    indices.push(point);
                }
            }
        }
        TriMesh { vertices, indices }
    }
}

impl ObjectToShape for TriMesh {
    type ShapeType = shape::TriMesh<f32>;

    fn to_shape(self) -> Self::ShapeType {
        shape::TriMesh::new(self.vertices, self.indices, None)
    }
}
