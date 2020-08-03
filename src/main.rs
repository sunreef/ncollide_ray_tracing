mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use image::RgbImage;
use nalgebra::{Point3, Vector3};
use ncollide3d::{
    math::Isometry,
    shape::{Cuboid, TriMesh},
};
use obj::{Obj, SimplePolygon};
use std::path::Path;

use crate::object::ObjectData;
use crate::scene::Scene;
use crate::shaders::lambert::LambertBSDF;

fn add_mesh_to_scene(scene: &mut Scene, obj_path: &Path) {
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
    let mesh_transform = Isometry::translation(0.0, 0.0, -1.0);
    let mesh_data = ObjectData {
        bsdf: Some(Box::new(LambertBSDF::new(Vector3::new(0.8, 0.8, 0.8)))),
        ..Default::default()
    };
    scene
        .add_shape(tri_mesh)
        .with_transform(mesh_transform)
        .with_data(mesh_data)
        .build();
}

fn add_lights(scene: &mut Scene) {
    let top_light = Cuboid::new(Vector3::new(3.0, 3.0, 0.1));
    let top_transform = Isometry::translation(0.0, 0.0, 6.0);
    let top_data = ObjectData {
        emission: Some((2.0f32, Point3::new(1.0, 1.0, 1.0))),
        ..Default::default()
    };
    scene
        .add_shape(top_light)
        .with_transform(top_transform)
        .with_data(top_data)
        .build();

    let left_light = Cuboid::new(Vector3::new(0.1, 3.0, 3.0));
    let left_transform = Isometry::translation(6.0, 0.0, 0.0);
    let left_data = ObjectData {
        emission: Some((2.0f32, Point3::new(0.4, 0.2, 0.6))),
        ..Default::default()
    };
    scene
        .add_shape(left_light)
        .with_transform(left_transform)
        .with_data(left_data)
        .build();
}

fn main() {
    let mut scene = Scene::new();
    let obj_path = Path::new("./assets/deer.obj");
    add_mesh_to_scene(&mut scene, &obj_path);
    add_lights(&mut scene);

    scene.perform_collision_phase();
    let samples = scene.capture(100u32);

    let mut image = RgbImage::new(samples.len() as u32, samples[0].len() as u32);
    let clamp = |x: f32| 1.0f32.min(0.0f32.max(x));
    for x in 0..samples.len() {
        for y in 0..samples[0].len() {
            let value = samples[x][y];

            image.get_pixel_mut(x as u32, y as u32).data = [
                (255.0 * clamp(value[0])) as u8,
                (255.0 * clamp(value[1])) as u8,
                (255.0 * clamp(value[2])) as u8,
            ];
        }
    }
    image.save("./output_cosine.png").unwrap();
}
