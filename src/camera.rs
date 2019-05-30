use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::{Point2, Point3, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    query::Ray,
    world::{CollisionGroups, CollisionWorld},
};
use rand::{prelude::*, rngs::SmallRng, thread_rng, Rng};
use rayon::{iter::ParallelIterator, prelude::*};

use std::f32;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::integrators::AOIntegrator;
use crate::sampler::UniformSampler2;
use crate::scene::Scene;
use crate::object::ObjectData;

pub struct CameraBuilder {
    position: Isometry<f32>,
    focal_length: f32,
    screen_dimensions: Vector2<f32>,
    resolution: Vector2<usize>,
}

pub struct Camera {
    position: Isometry<f32>,
    focal_length: f32,
    screen_dimensions: Vector2<f32>,
    resolution: Vector2<usize>,

    pixel_dimensions: Vector2<f32>,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            position: Isometry::identity(),
            focal_length: 1.0,
            screen_dimensions: Vector2::new(1.0, 1.0),
            resolution: Vector2::new(100, 100),
        }
    }

    pub fn position(mut self, new_position: Isometry<f32>) -> Self {
        self.position = new_position;
        self
    }

    pub fn focal_length(mut self, new_focal_length: f32) -> Self {
        self.focal_length = new_focal_length;
        self
    }

    pub fn screen_dimensions(mut self, new_dimensions: Vector2<f32>) -> Self {
        self.screen_dimensions = new_dimensions;
        self
    }

    pub fn resolution(mut self, new_resolution: Vector2<usize>) -> Self {
        self.resolution = new_resolution;
        self
    }

    pub fn build(self) -> Camera {
        let pixel_dimensions = Vector2::new(
            self.screen_dimensions[0] / self.resolution[0] as f32,
            self.screen_dimensions[1] / self.resolution[1] as f32,
        );
        Camera {
            position: self.position,
            focal_length: self.focal_length,
            screen_dimensions: self.screen_dimensions,
            resolution: self.resolution,

            pixel_dimensions: pixel_dimensions,
        }
    }
}

impl Camera {
    pub fn compute_samples(
        &self,
        scene: &Scene,
        n_samples: u32,
    ) -> Vec<Vec<Point3<f32>>> {
        let mut image = RgbImage::new(self.resolution[0] as u32, self.resolution[1] as u32);
        let integrator = AOIntegrator::new(5.0);
        let pixel_sampler = UniformSampler2::new(self.pixel_dimensions);
        let start_time = Instant::now();
        let samples = (0..self.resolution[0])
            .into_par_iter()
            .map(|x| {
                let mut rng = thread_rng();
                let mut row: Vec<Point3<f32>> = Vec::new();
                row.resize(self.resolution[1], Point3::new(0.0, 0.0, 0.0));
                for y in 0..self.resolution[1] {
                    for s in 0..n_samples {
                        let x_coord = (x as f32 - self.resolution[0] as f32 / 2.0);
                        let y_coord = -(y as f32 - self.resolution[1] as f32 / 2.0);
                        let pixel_samples =
                            Point2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0));
                        let pixel_position = pixel_sampler.sample(&pixel_samples);
                        let ray_target = Point3::new(
                            x_coord * self.pixel_dimensions[0] + pixel_position[0],
                            y_coord * self.pixel_dimensions[1] + pixel_position[1],
                            self.focal_length,
                        );
                        let ray_direction = ray_target.coords.normalize();
                        let initial_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), ray_direction)
                            .transform_by(&self.position);
                        let sample_value = integrator.launch_ray(&initial_ray, scene, &mut rng);
                        row[y] = (row[y] * s as f32 + sample_value.coords) / (s + 1) as f32;
                    }
                }
                row
            })
            .collect::<Vec<_>>();
        for x in 0..self.resolution[0] {
            for y in 0..self.resolution[1] {
                let value = samples[x][y];
                image.get_pixel_mut(x as u32, y as u32).data =
                    [(255.0 * value[0]) as u8, (255.0 * value[1]) as u8, (255.0 * value[2]) as u8];
            }
        }
        image.save("./output.png").unwrap();
        let end_time = Instant::now() - start_time;

        println!(
            "Total time: {} seconds",
            end_time.as_millis() as f32 / 1000.0
        );

        Vec::new()
    }
}
