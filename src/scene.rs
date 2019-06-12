use nalgebra::{Point3, Vector3};
use ncollide3d::{
    math::Isometry,
    shape::{Cuboid, ShapeHandle, TriMesh},
    world::{CollisionGroups, CollisionWorld, GeometricQueryType},
};
use obj::{Obj, SimplePolygon};

use std::path::Path;

use crate::object::ObjectData;
use crate::shaders::lambert::LambertBSDF;

pub struct Scene {
    pub collision_world: CollisionWorld<f32, ObjectData>,
}

impl Scene {
    pub fn new() -> Self {
        let obj_path = Path::new("./assets/deer.obj");
        let mesh = Obj::<SimplePolygon>::load(obj_path).unwrap();
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
        let tri_mesh = TriMesh::new(vertices, indices, None);

        let ground = Cuboid::new(Vector3::new(0.5, 1.0, 0.5));

        let mut world = CollisionWorld::new(0.0001f32);
        let mesh_transform = Isometry::translation(0.0, 0.0, -1.0);
        let mesh_data = ObjectData {
            bsdf: Some(Box::new(LambertBSDF::new(Vector3::new(0.8, 0.0, 0.0)))),
            ..Default::default()
        };
        world.add(
            mesh_transform,
            ShapeHandle::new(tri_mesh),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            mesh_data,
        );
        let pedestal_transform = Isometry::translation(0.0, 0.0, -1.8);
        let pedestal_data = ObjectData {
            bsdf: Some(Box::new(LambertBSDF::new(Vector3::new(0.0, 0.0, 0.8)))),
            ..Default::default()
        };
        world.add(
            pedestal_transform,
            ShapeHandle::new(ground),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            pedestal_data,
        );
        world.perform_broad_phase();
        world.perform_narrow_phase();

        Scene {
            collision_world: world,
        }
    }
}
