use nalgebra::{Point2, Point3, Vector3};
use ncollide3d::{
    pipeline::object::CollisionGroups,
    query::{Ray, RayIntersection},
    shape::FeatureId,
};
use rand::Rng;
use std::f32;

use crate::math::vector_traits::{ToGlobal, ToLocal};
use crate::object::ObjectData;
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

    pub fn launch_ray<R: Rng>(&self, ray: &Ray<f32>, scene: &Scene, rng: &mut R) -> Point3<f32> {
        let mut sample_value = Point3::new(0.0, 0.0, 0.0);
        let mut contribution = Vector3::new(1.0f32, 1.0f32, 1.0f32);
        let mut keep_going = true;

        let mut count_bounces = 0;

        let mut current_ray = ray.clone();
        while keep_going {
            let mut min_toi = f32::MAX;
            let mut found_intersection = false;
            let mut min_intersection =
                RayIntersection::new(0.0, Vector3::new(0.0, 0.0, 0.0), FeatureId::Unknown);
            let mut min_data = &ObjectData::default();

            for intersection in scene.collision_world.interferences_with_ray(
                &current_ray,
                f32::MAX,
                &CollisionGroups::new(),
            ) {
                found_intersection = true;
                if intersection.2.toi < min_toi {
                    min_toi = intersection.2.toi;
                    min_intersection = intersection.2;
                    min_data = intersection.1.data();
                }
            }
            if !found_intersection {
                sample_value += Vector3::new(0.0, 0.0, 0.0);
                break;
            }

            let emission = &min_data.emission;
            let bsdf = &min_data.bsdf;
            let normal = &min_intersection.normal;

            match emission {
                Some((intensity, color)) => {
                    if count_bounces > 0 {
                        sample_value += *intensity * contribution.component_mul(&color.coords);
                    }
                }
                None => {}
            }

            match bsdf {
                Some(bsdf_function) => {
                    let local_incident_vector = current_ray.dir.to_local(&normal);
                    let bsdf_samples =
                        Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
                    let (local_new_dir, bsdf_value, bsdf_probability) =
                        bsdf_function.sample(&local_incident_vector, &bsdf_samples);
                    let global_new_dir = local_new_dir.to_global(&normal).normalize();
                    //println!("{:?}", global_new_dir.dot(normal));
                    current_ray = Ray::new(
                        current_ray.point_at(min_toi) + 0.01f32 * normal,
                        global_new_dir,
                    );

                    let cos_theta = local_new_dir[2];
                    contribution =
                        contribution.component_mul(&bsdf_value) * cos_theta / bsdf_probability;
                    count_bounces += 1;

                    let roulette_sample = rng.gen_range(0.0, 1.0);
                    if roulette_sample > self.roulette_threshold {
                        keep_going = false;
                    } else {
                        keep_going = true;
                        contribution /= self.roulette_threshold;
                    }
                }
                None => {
                    keep_going = false;
                }
            }
        }
        sample_value
    }
}
