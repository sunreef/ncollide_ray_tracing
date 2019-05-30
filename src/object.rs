use nalgebra::Point3;
use crate::shaders::BRDF;

#[derive(Default)]
pub struct ObjectData {
    pub albedo: Option<Point3<f32>>,
    pub emission: Option<(f32, Point3<f32>)>,
    pub brdf: Option<Box<BRDF>>,
}