use nalgebra::{Point2, Point3, Vector3};
use ncollide3d::{
    query::{Ray, RayIntersection},
    shape::FeatureId,
    world::{CollisionGroups, CollisionWorld},
};
use rand::Rng;
use std::f32;
use std::f32::consts::PI;

use crate::object::ObjectData;
use crate::sampler::{CosineWeightedHemisphereSampler, HemisphereSampler};
use crate::scene::Scene;

pub struct AOIntegrator {
    range: f32,
}

impl AOIntegrator {
    pub fn new(range: f32) -> Self {
        AOIntegrator { range }
    }

    pub fn launch_ray<R: Rng>(&self, ray: &Ray<f32>, scene: &Scene, rng: &mut R) -> Point3<f32> {
        let mut min_toi = f32::MAX;
        let mut sample_value = Point3::new(0.0, 0.0, 0.0);
        let mut min_intersection =
            RayIntersection::new(0.0, Vector3::new(0.0, 0.0, 0.0), FeatureId::Unknown);
        let mut min_data = &ObjectData::default();
        let mut found_intersection = false;
        for intersection in scene
            .collision_world
            .interferences_with_ray(&ray, &CollisionGroups::new())
        {
            found_intersection = true;
            if intersection.1.toi < min_toi {
                let normal = intersection.1.normal;
                sample_value = Point3::new(
                    (125.0 + (normal[0] * 125.0)),
                    (125.0 + (normal[1] * 125.0)),
                    (125.0 + (normal[2] * 125.0)),
                );
                min_toi = intersection.1.toi;
                min_intersection = intersection.1;
                min_data = intersection.0.data();
            }
        }
        if !found_intersection {
            return Point3::new(0.0, 0.0, 0.0);
        }

        let sampler = CosineWeightedHemisphereSampler;
        let ray_samples = Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
        let new_ray_origin = ray.point_at(min_intersection.toi - 0.01);
        let (new_ray_direction, new_ray_probability) =
            sampler.sample(&ray_samples, &min_intersection.normal);
        //        println!("{:?}, {:?}", min_intersection.normal, new_ray_direction);
        let new_ray = Ray::new(new_ray_origin, new_ray_direction);

        let mut min_toi = f32::MAX;
        for intersection in scene
            .collision_world
            .interferences_with_ray(&new_ray, &CollisionGroups::new())
        {
            if intersection.1.toi < min_toi {
                min_toi = intersection.1.toi;
            }
        }
        if min_toi < self.range {
            Point3::new(0.0, 0.0, 0.0)
        } else {
            (1.0 / (2.0 * PI)) * min_data.albedo.as_ref().unwrap().clone() / new_ray_probability
        }
    }
}
