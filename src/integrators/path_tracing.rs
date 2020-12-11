use nalgebra::{Point2, Point3, Vector3};
use ncollide3d::{
    pipeline::object::CollisionGroups,
    query::{Ray, RayIntersection},
    shape::FeatureId,
};
use rand::Rng;
use std::f32;

use crate::math::vector_traits::{ToGlobal, ToLocal};
use crate::object::WorldObjectData;
use crate::sampling::UniformShapeSampler;
use crate::scene::Scene;

pub struct PathTracingIntegrator {
    roulette_threshold: f32,
}

impl PathTracingIntegrator {
    pub fn new() -> Self {
        PathTracingIntegrator {
            roulette_threshold: 0.9f32,
        }
    }

    pub fn launch_ray<R: Rng>(&self, ray: &Ray<f32>, scene: &Scene, rng: &mut R) -> Vector3<f32> {
        let mut sample_value = Vector3::new(0.0, 0.0, 0.0);

        let mut min_toi = f32::MAX;
        let mut min_intersection =
            RayIntersection::new(0.0, Vector3::new(0.0, 0.0, 0.0), FeatureId::Unknown);
        let mut min_data = &WorldObjectData::default();

        match scene.collision_world.first_interference_with_ray(
            &ray,
            f32::MAX,
            &CollisionGroups::new(),
        ) {
            Some(intersection) => {
                min_toi = intersection.inter.toi;
                min_intersection = intersection.inter;
                min_data = intersection.co.data();
            }
            None => {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }

        let emission = &min_data.emission;
        let bsdf = &min_data.bsdf;
        let normal = &min_intersection.normal;

        // Emissive material contribution
        match emission {
            Some((intensity, color)) => {
                sample_value += *intensity * *color;
            }
            None => {}
        }

        // Light sampling

        // BSDF sampling
        match bsdf {
            Some(bsdf_function) => {
                let emitter_index = rng.gen_range(0, scene.emitters.len());
                let emitter_handle = scene.emitters[emitter_index];

                let emitter_object = scene
                    .collision_world
                    .collision_object(emitter_handle)
                    .unwrap();
                let emitter_shape = emitter_object.shape();
                let emitter_data = emitter_object.data();

                let shape_sampler = UniformShapeSampler;
                let emitter_samples = Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
                let (sampled_point, probability) =
                    shape_sampler.sample(&emitter_shape, &emitter_samples);
                let current_intersection_point = ray.point_at(min_toi) + 0.001f32 * normal;
                let mut emitter_dir = (sampled_point - current_intersection_point);
                let emitter_dist = emitter_dir.norm();
                emitter_dir /= emitter_dist;
                let emitter_ray = Ray::new(current_intersection_point.clone(), emitter_dir);
                match scene.collision_world.first_interference_with_ray(
                    &emitter_ray,
                    emitter_dist,
                    &CollisionGroups::new(),
                ) {
                    Some(intersection) => {}
                    None => {}
                }

                let roulette_sample = rng.gen_range(0.0, 1.0);
                if roulette_sample > self.roulette_threshold {
                    return sample_value;
                }
                let local_incident_vector = ray.dir.to_local(&normal);
                let bsdf_samples = Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
                let (local_new_dir, bsdf_value, bsdf_probability) =
                    bsdf_function.sample(&local_incident_vector, &bsdf_samples);
                let global_new_dir = local_new_dir.to_global(&normal).normalize();
                let new_ray = Ray::new(current_intersection_point, global_new_dir);
                let bounce_value = self.launch_ray(&new_ray, scene, rng);

                if bsdf_function.is_diffuse() {
                    let cos_theta = local_new_dir[2];
                    sample_value += bounce_value.component_mul(&bsdf_value) * cos_theta
                        / (bsdf_probability * self.roulette_threshold);
                } else {
                    sample_value += bounce_value.component_mul(&bsdf_value)
                        / (bsdf_probability * self.roulette_threshold);
                }
            }
            None => {}
        }
        sample_value
    }
}
