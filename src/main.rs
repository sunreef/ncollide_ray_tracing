mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use image::RgbImage;
use nalgebra::{Point3, Vector3};
use ncollide3d::math::Isometry;

use crate::object::{shapes::Shape, ObjectData};
use crate::scene::Scene;
use crate::shaders::Shader;

fn add_mesh_to_scene(scene: &mut Scene, obj_path: String) {
    let mesh_data = ObjectData {
        shape: Some(Shape::TriMesh(obj_path)),
        position: Some(Isometry::translation(0.0, 0.0, -1.0)),
        bsdf: Some(Shader::Lambert(Vector3::new(0.8, 0.8, 0.8))),
        ..Default::default()
    };
    let json_str = serde_json::to_string_pretty(&mesh_data).expect("serialization failed");
    println!("{}", json_str);

    scene.add_object(serde_json::from_str(&json_str).expect("Deserialization failed"));
}

fn add_lights(scene: &mut Scene) {
    let left_light_data = ObjectData {
        shape: Some(Shape::Cuboid(Vector3::new(100.0, 0.1, 100.0))),
        position: Some(Isometry::translation(0.0, 20.0, 0.0)),
        emission: Some((1.0f32, Point3::new(0.4, 0.4, 0.8))),
        ..Default::default()
    };
    scene.add_object(left_light_data);

    let sun_light_data = ObjectData {
        shape: Some(Shape::Cuboid(Vector3::new(1.0, 1.0, 1.0))),
        position: Some(Isometry::translation(4.0, 8.0, 0.0)),
        emission: Some((20.0f32, Point3::new(0.9, 0.7, 0.6))),
        ..Default::default()
    };
    scene.add_object(sun_light_data);
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
