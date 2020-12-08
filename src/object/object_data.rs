use nalgebra::{Isometry3, Point3, Vector3};
use serde::{Deserialize, Serialize};

use crate::object::shapes::Shape;
use crate::shaders::{Shader, BSDF};

#[derive(Default, Serialize, Deserialize)]
pub struct ObjectData {
    pub shape: Option<Shape>,
    pub position: Option<Isometry3<f32>>,
    pub emission: Option<(f32, Vector3<f32>)>,
    pub bsdf: Option<Shader>,
}

impl ObjectData {
    pub fn to_world_data(self) -> WorldObjectData {
        WorldObjectData {
            emission: self.emission,
            bsdf: match self.bsdf {
                Some(shader) => Some(shader.to_bsdf()),
                None => None,
            },
        }
    }
}

#[derive(Default)]
pub struct WorldObjectData {
    pub emission: Option<(f32, Vector3<f32>)>,
    pub bsdf: Option<Box<dyn BSDF>>,
}
