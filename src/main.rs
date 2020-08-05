mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use image::RgbImage;
use nalgebra::{Point3, Vector3};
use ncollide3d::{math::Isometry, shape::Cuboid, transformation::ToTriMesh};
use obj::{Obj, SimplePolygon};
use std::path::Path;

use crate::object::shapes::TriMesh;
use crate::object::{ObjectData, ObjectToShape};
use crate::scene::Scene;
use crate::shaders::lambert::LambertBSDF;

fn add_mesh_to_scene(scene: &mut Scene, obj_path: String) {
    let trimesh = TriMesh::new(obj_path);
    let mesh_transform = Isometry::translation(0.0, 0.0, -1.0);
    let mesh_data = ObjectData {
        bsdf: Some(Box::new(LambertBSDF::new(Vector3::new(0.8, 0.8, 0.8)))),
        ..Default::default()
    };
    scene
        .add_shape(trimesh.to_shape())
        .with_transform(mesh_transform)
        .with_data(mesh_data)
        .build();
}

fn add_lights(scene: &mut Scene) {
    let left_light = Cuboid::new(Vector3::new(100.0, 0.1, 100.0));
    let left_transform = Isometry::translation(0.0, 20.0, 0.0);
    let left_data = ObjectData {
        emission: Some((1.0f32, Point3::new(0.4, 0.4, 0.8))),
        ..Default::default()
    };
    scene
        .add_shape(left_light)
        .with_transform(left_transform)
        .with_data(left_data)
        .build();

    let sun_light = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
    let sun_transform = Isometry::translation(4.0, 8.0, 0.0);
    let sun_data = ObjectData {
        emission: Some((20.0f32, Point3::new(0.9, 0.7, 0.6))),
        ..Default::default()
    };
    scene
        .add_shape(sun_light)
        .with_transform(sun_transform)
        .with_data(sun_data)
        .build();
}

fn main() {
    let mut scene = Scene::new();
    let obj_path = "./assets/deer.obj".to_owned();
    add_mesh_to_scene(&mut scene, obj_path);
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
    image.save("./results/output_cosine.png").unwrap();
}
