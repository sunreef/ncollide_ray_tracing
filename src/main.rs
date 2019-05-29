mod camera;
mod sampler;

use std::path::Path;
use std::sync::Arc;

use nalgebra::{Point3, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    query::Ray,
    shape::{Segment, ShapeHandle, TriMesh},
    world::{CollisionGroups, CollisionWorld, GeometricQueryType},
};
use obj::{Obj, SimplePolygon};

use crate::camera::{Camera, CameraBuilder};

fn main() {
    let obj_path = Path::new("./assets/cube.obj");

    let mesh = Obj::<SimplePolygon>::load(obj_path).unwrap();

    let vertices = mesh
        .position
        .iter()
        .map(|p| Point3::from_slice(p))
        .collect::<Vec<_>>();
    //let normals = mesh
    //.normal
    //.iter()
    //.map(|p| Vector3::from_column_slice(p))

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

    let mut world = CollisionWorld::new(0.01f32);
    let cube_transform = Isometry::translation(0.0, 0.0, 0.0);
    let collision_group = CollisionGroups::new();
    world.add(
        cube_transform,
        ShapeHandle::new(tri_mesh),
        collision_group,
        GeometricQueryType::Contacts(0.01, 0.01),
        (),
    );
    world.perform_broad_phase();
    world.perform_narrow_phase();

    let camera = CameraBuilder::new()
        .position(Isometry::face_towards(
            &Point3::new(-5.0, 2.0, 2.0),
            &Point3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 1.0),
        ))
        .resolution(Vector2::new(400, 400))
        .build();
    camera.compute_samples(&world, &collision_group, 100);
}
