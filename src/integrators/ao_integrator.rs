use nalgebra::{Point2, Point3, Vector3};
use ncollide3d::{
    pipeline::object::CollisionGroups,
    query::{Ray, RayIntersection},
    shape::FeatureId,
};
use rand::Rng;
use std::f32;
use std::f32::consts::PI;

use crate::sampler::UniformHemisphereSampler;
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
        let mut min_intersection =
            RayIntersection::new(0.0, Vector3::new(0.0, 0.0, 0.0), FeatureId::Unknown);
        let mut found_intersection = false;
        for intersection in
            scene
                .collision_world
                .interferences_with_ray(&ray, f32::MAX, &CollisionGroups::new())
        {
            found_intersection = true;
            if intersection.2.toi < min_toi {
                min_toi = intersection.2.toi;
                min_intersection = intersection.2;
            }
        }
        if !found_intersection {
            return Point3::new(0.0, 0.0, 0.0);
        }

        //let sampler = CosineWeightedHemisphereSampler;
        let sampler = UniformHemisphereSampler;
        let ray_samples = Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
        let new_ray_origin = ray.point_at(min_intersection.toi - 0.001);
        let (new_ray_direction, new_ray_probability) =
            sampler.sample(&ray_samples, &min_intersection.normal);
        let new_ray = Ray::new(new_ray_origin, new_ray_direction);

        for _ in scene.collision_world.interferences_with_ray(
            &new_ray,
            self.range,
            &CollisionGroups::new(),
        ) {
            return Point3::new(0.0, 0.0, 0.0);
        }
        (1.0 / (2.0 * PI)) * Point3::new(1.0, 1.0, 1.0) / new_ray_probability
    }
}
