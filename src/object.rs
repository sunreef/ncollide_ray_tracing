use crate::shaders::BSDF;
use nalgebra::Point3;

#[derive(Default)]
pub struct ObjectData {
    pub emission: Option<(f32, Point3<f32>)>,
    pub bsdf: Option<Box<BSDF>>,
}
