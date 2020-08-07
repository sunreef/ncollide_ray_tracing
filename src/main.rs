mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use image::RgbImage;
use nalgebra::{Point3, Vector2, Vector3};
use ncollide3d::math::Isometry;

use crate::camera::CameraBuilder;
use crate::object::{shapes::Shape, ObjectData};
use crate::scene::SceneData;
use crate::shaders::Shader;

fn add_objects_to_scene(scene: &mut SceneData, obj_path: String) {
    let mirror_ball = ObjectData {
        shape: Some(Shape::Ball(0.5f32)),
        position: Some(Isometry::translation(0.0, 0.0, 0.0)),
        //bsdf: Some(Shader::Lambert(Vector3::new(0.8, 0.8, 0.8))),
        bsdf: Some(Shader::Mirror),
        ..Default::default()
    };
    scene.add_object(mirror_ball);

    let left_green_wall = ObjectData {
        shape: Some(Shape::Cuboid(Vector3::new(10.0, 0.1, 10.0))),
        position: Some(Isometry::translation(-5.0, 3.0, -5.0)),
        bsdf: Some(Shader::Lambert(Vector3::new(0.0, 1.0, 0.0))),
        //bsdf: Some(Shader::Mirror),
        ..Default::default()
    };
    scene.add_object(left_green_wall);

    let right_red_wall = ObjectData {
        shape: Some(Shape::Cuboid(Vector3::new(10.0, 0.1, 10.0))),
        position: Some(Isometry::translation(-5.0, -3.0, -5.0)),
        bsdf: Some(Shader::Lambert(Vector3::new(1.0, 0.0, 0.0))),
        //bsdf: Some(Shader::Mirror),
        ..Default::default()
    };
    scene.add_object(right_red_wall);

    //let mesh_data = ObjectData {
    //shape: Some(Shape::TriMesh(obj_path)),
    //position: Some(Isometry::translation(0.0, 0.0, -1.0)),
    //bsdf: Some(Shader::Lambert(Vector3::new(0.8, 0.8, 0.8))),
    ////bsdf: Some(Shader::Mirror),
    //..Default::default()
    //};
    //scene.add_object(mesh_data);
}

fn add_lights(scene: &mut SceneData) {
    //let left_light_data = ObjectData {
    //shape: Some(Shape::Cuboid(Vector3::new(100.0, 0.1, 100.0))),
    //position: Some(Isometry::translation(0.0, 20.0, 0.0)),
    //emission: Some((1.3f32, Point3::new(0.4, 0.4, 0.8))),
    //..Default::default()
    //};
    //scene.add_object(left_light_data);

    let sun_light_data = ObjectData {
        shape: Some(Shape::Cuboid(Vector3::new(0.3, 0.3, 0.05))),
        position: Some(Isometry::translation(0.0, 0.0, 2.0)),
        emission: Some((100.0f32, Point3::new(1.0, 1.0, 1.0))),
        ..Default::default()
    };
    scene.add_object(sun_light_data);
}

fn main() {
    let mut scene_data = SceneData {
        camera: Some(
            CameraBuilder::new()
                .position(Isometry::face_towards(
                    &Point3::new(5.0, 0.0, 0.0),
                    &Point3::new(0.0, 0.0, 0.0),
                    &Vector3::new(0.0, 0.0, 1.0),
                ))
                .screen_dimensions(Vector2::new(1.0, 1.0))
                .resolution(Vector2::new(1000, 1000))
                .build(),
        ),
        objects: Vec::new(),
    };
    let obj_path = "./assets/deer.obj".to_owned();
    add_objects_to_scene(&mut scene_data, obj_path);
    add_lights(&mut scene_data);

    println!("{}", serde_json::to_string_pretty(&scene_data).expect(""));

    let mut scene = scene_data.to_scene();

    scene.perform_collision_phase();
    let samples = scene.capture(1000u32);

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
