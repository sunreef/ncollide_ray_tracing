use ncollide3d::{
    math::Isometry,
    pipeline::object::{CollisionGroups, GeometricQueryType},
    shape::{Shape, ShapeHandle},
};

use crate::object::ObjectData;
use crate::scene::Scene;

pub struct ShapeBuilder<'a, S> {
    scene: &'a mut Scene,
    shape: S,
    transform: Isometry<f32>,
    data: ObjectData,
}

impl<'a, S> ShapeBuilder<'a, S>
where
    S: Shape<f32>,
{
    pub fn new(scene: &'a mut Scene, shape: S) -> Self {
        ShapeBuilder {
            scene,
            shape,
            transform: Isometry::identity(),
            data: ObjectData::default(),
        }
    }

    pub fn with_transform(mut self, transform: Isometry<f32>) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_data(mut self, data: ObjectData) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) {
        self.scene.collision_world.add(
            self.transform,
            ShapeHandle::new(self.shape),
            CollisionGroups::new(),
            GeometricQueryType::Contacts(0.0001, 0.0001),
            self.data,
        );
    }
}
