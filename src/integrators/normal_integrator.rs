use nalgebra::Point3;
use ncollide3d::{
    query::Ray,
    world::{CollisionGroups, CollisionWorld},
};
use rand::Rng;
use std::f32;

pub struct NormalIntegrator;

impl NormalIntegrator {
    pub fn launch_ray<R: Rng>(
        &self,
        ray: &Ray<f32>,
        world: &CollisionWorld<f32, ()>,
        rng: &mut R,
    ) -> Point3<f32> {
        let mut min_toi = f32::MAX;
        let mut sample_value = Point3::new(0.0, 0.0, 0.0);
        for intersection in world.interferences_with_ray(&ray, &CollisionGroups::new()) {
            if intersection.1.toi < min_toi {
                let normal = intersection.1.normal;
                sample_value = Point3::new(
                    (125.0 + (normal[0] * 125.0)),
                    (125.0 + (normal[1] * 125.0)),
                    (125.0 + (normal[2] * 125.0)),
                );
                min_toi = intersection.1.toi;
            }
        }
        sample_value
    }
}
