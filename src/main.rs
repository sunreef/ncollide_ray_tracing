mod camera;
mod normal_integrator;
mod sampler;

use std::path::Path;
use std::sync::Arc;

use nalgebra::{Point3, Unit, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    query::Ray,
    shape::{Cuboid, Plane, Segment, ShapeHandle, TriMesh},
    world::{CollisionGroups, CollisionWorld, GeometricQueryType},
};
use obj::{Obj, SimplePolygon};

use crate::camera::{Camera, CameraBuilder};

fn main() {
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

    let mut world = CollisionWorld::new(0.01f32);
    let cube_transform = Isometry::translation(0.0, 0.0, -1.0);
    let collision_group = CollisionGroups::new();
    world.add(
        cube_transform,
        ShapeHandle::new(tri_mesh),
        collision_group,
        GeometricQueryType::Contacts(0.01, 0.01),
        (),
    );
    let pedestal_transform = Isometry::translation(0.0, 0.0, -1.8);
    world.add(
        pedestal_transform,
        ShapeHandle::new(ground),
        collision_group,
        GeometricQueryType::Contacts(0.01, 0.01),
        (),
    );
    world.perform_broad_phase();
    world.perform_narrow_phase();

    let camera = CameraBuilder::new()
        .position(Isometry::face_towards(
            &Point3::new(4.0, -4.0, 1.5),
            &Point3::new(0.0, 0.0, -0.5),
            &Vector3::new(0.0, 0.0, 1.0),
        ))
        .screen_dimensions(Vector2::new(0.8, 0.8))
        .resolution(Vector2::new(800, 800))
        .build();
    camera.compute_samples(&world, &collision_group, 200);
}
