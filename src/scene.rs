use nalgebra::{Point3, Unit, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    pipeline::object::{CollisionGroups, GeometricQueryType},
    shape::{Cuboid, ShapeHandle, TriMesh},
    world::CollisionWorld,
};
use obj::{Obj, SimplePolygon};

use std::path::Path;

use crate::camera::{Camera, CameraBuilder};
use crate::object::ObjectData;
use crate::shaders::lambert::LambertBSDF;

pub struct Scene {
    pub camera: Camera,
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

        let mut world = CollisionWorld::new(0.0001f32);
        let mesh_transform = Isometry::translation(0.0, 0.0, -1.0);
        let mesh_data = ObjectData {
            bsdf: Some(Box::new(LambertBSDF::new(Vector3::new(0.8, 0.8, 0.8)))),
            ..Default::default()
        };
        world.add(
            mesh_transform,
            ShapeHandle::new(tri_mesh),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            mesh_data,
        );

        let top_light = Cuboid::new(Vector3::new(3.0, 3.0, 0.1));
        let top_transform = Isometry::translation(0.0, 0.0, 6.0);
        let top_data = ObjectData {
            emission: Some((2.0f32, Point3::new(1.0, 1.0, 1.0))),
            ..Default::default()
        };
        world.add(
            top_transform,
            ShapeHandle::new(top_light),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            top_data,
        );

        let left_light = Cuboid::new(Vector3::new(0.1, 3.0, 3.0));
        let left_transform = Isometry::translation(6.0, 0.0, 0.0);
        let left_data = ObjectData {
            emission: Some((2.0f32, Point3::new(1.0, 0.0, 1.0))),
            ..Default::default()
        };
        world.add(
            left_transform,
            ShapeHandle::new(left_light),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            left_data,
        );
        world.perform_broad_phase();
        world.perform_narrow_phase();

        Scene {
            camera: CameraBuilder::new()
                .position(Isometry::face_towards(
                    &Point3::new(4.0, -4.0, 1.5),
                    &Point3::new(0.0, 0.0, -0.5),
                    &Vector3::new(0.0, 0.0, 1.0),
                ))
                .screen_dimensions(Vector2::new(0.8, 0.8))
                .resolution(Vector2::new(800, 800))
                .build(),
            collision_world: world,
        }
    }

    pub fn capture(&self, n_samples: u32) -> Vec<Vec<Point3<f32>>> {
        self.camera.compute_samples(&self, n_samples)
    }
}
