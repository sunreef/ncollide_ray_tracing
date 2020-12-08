use nalgebra::{Point3, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    pipeline::object::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType},
    world::CollisionWorld,
};
use serde::{Deserialize, Serialize};

use crate::camera::{Camera, CameraBuilder};
use crate::object::{ObjectData, WorldObjectData};

pub struct Scene {
    pub camera: Camera,
    pub collision_world: CollisionWorld<f32, WorldObjectData>,

    pub emitters: Vec<CollisionObjectSlabHandle>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: CameraBuilder::new()
                .position(Isometry::face_towards(
                    &Point3::new(4.0, -4.0, 1.5),
                    &Point3::new(-0.5, 0.0, 0.0),
                    &Vector3::new(0.0, 0.0, 1.0),
                ))
                .screen_dimensions(Vector2::new(0.8, 0.6))
                .resolution(Vector2::new(800, 600))
                .build(),
            collision_world: CollisionWorld::<f32, WorldObjectData>::new(0.0001f32),
            emitters: Vec::new(),
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn add_object(&mut self, mut data: ObjectData) {
        let shape = std::mem::take(&mut data.shape);
        let position = std::mem::take(&mut data.position);
        let world_data = data.to_world_data();

        match (position, shape) {
            (Some(pos), Some(s)) => {
                let mut is_emitter = world_data.emission != None;

                let (object_handle, _) = self.collision_world.add(
                    pos,
                    s.get_handle(),
                    CollisionGroups::new(),
                    GeometricQueryType::Contacts(0.0001, 0.0001),
                    world_data,
                );
                if is_emitter {
                    self.emitters.push(object_handle);
                }
            }
            _ => (),
        }
    }

    pub fn perform_collision_phase(&mut self) {
        self.collision_world.perform_broad_phase();
        self.collision_world.perform_narrow_phase();
    }

    pub fn capture(&self, n_samples: u32) -> Vec<Vec<Point3<f32>>> {
        self.camera.compute_samples(&self, n_samples)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct SceneData {
    pub camera: Option<Camera>,
    pub objects: Vec<ObjectData>,
}

impl SceneData {
    pub fn add_object(&mut self, object: ObjectData) {
        self.objects.push(object);
    }

    pub fn to_scene(self) -> Scene {
        let mut scene = Scene::new();
        match self.camera {
            Some(camera) => scene.set_camera(camera),
            None => (),
        }
        for object in self.objects {
            scene.add_object(object);
        }
        scene
    }
}
