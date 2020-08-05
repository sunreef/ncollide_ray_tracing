use std::path::Path;

use nalgebra::Point3;
use ncollide3d::shape;
use obj::{Obj, SimplePolygon};
use serde::{Deserialize, Serialize};

use crate::object::shapes::ObjectToShape;

#[derive(Serialize, Deserialize)]
pub struct TriMesh {
    obj_path: String,
}

impl TriMesh {
    pub fn new(path: String) -> Self {
        TriMesh { obj_path: path }
    }
}

impl ObjectToShape for TriMesh {
    type ShapeType = shape::TriMesh<f32>;

    fn to_shape(self) -> Self::ShapeType {
        let mesh = Obj::<SimplePolygon>::load(Path::new(&self.obj_path)).unwrap();
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

        shape::TriMesh::new(vertices, indices, None)
    }
}
