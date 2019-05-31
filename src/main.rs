mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use std::path::Path;
use std::sync::Arc;

use nalgebra::{Point3, Unit, Vector2, Vector3};
use ncollide3d::math::Isometry;

use crate::camera::{Camera, CameraBuilder};
use crate::scene::Scene;

fn main() {
    let scene = Scene::new();
    let camera = CameraBuilder::new()
        .position(Isometry::face_towards(
            &Point3::new(4.0, -4.0, 1.5),
            &Point3::new(0.0, 0.0, -0.5),
            &Vector3::new(0.0, 0.0, 1.0),
        ))
        .screen_dimensions(Vector2::new(0.8, 0.8))
        .resolution(Vector2::new(800, 800))
        .build();
    camera.compute_samples(&scene, 100);
}
