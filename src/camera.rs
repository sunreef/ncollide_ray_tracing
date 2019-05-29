use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::{Point2, Point3, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    query::Ray,
    world::{CollisionGroups, CollisionWorld},
};
use rand::{thread_rng, Rng};

use std::f32;

use crate::sampler::{Sampler2, UniformSampler2};

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
        world: &CollisionWorld<f32, ()>,
        collision_group: &CollisionGroups,
        n_samples: u32,
    ) -> Vec<Vec<Point3<f32>>> {
        let mut rng = thread_rng();
        let mut samples = Vec::new();
        let mut image = RgbImage::new(self.resolution[0] as u32, self.resolution[1] as u32);
        let pixel_sampler = UniformSampler2::new(self.pixel_dimensions);
        for x in 0..self.resolution[0] {
            samples.push(Vec::new());
            let x_coord = (x as f32 - self.resolution[0] as f32 / 2.0);
            for y in 0..self.resolution[1] {
                samples[x].push(Vec::new());
                for _ in 0..n_samples {
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
                    let mut min_toi = f32::MAX;
                    let mut sample_value = [0.0f32,0.0f32,0.0f32];
                    for intersection in world.interferences_with_ray(&initial_ray, collision_group)
                    {
                        if intersection.1.toi < min_toi {
                            let normal = intersection.1.normal;
                            sample_value = [
                                (125.0 + (normal[0] * 125.0)),
                                (125.0 + (normal[1] * 125.0)),
                                (125.0 + (normal[2] * 125.0)),
                            ];
                            min_toi = intersection.1.toi;
                        }
                    }
                    samples[x][y].push(sample_value);
                }
            }
        }
        for x in 0..self.resolution[0] {
            for y in 0..self.resolution[1] {
                let mut average = [0.0, 0.0, 0.0];
                for s in 0..n_samples {
                    average[0] += samples[x][y][s as usize][0];
                    average[1] += samples[x][y][s as usize][1];
                    average[2] += samples[x][y][s as usize][2];
                }
                image.get_pixel_mut(x as u32, y as u32).data = [
                    (average[0] / n_samples as f32) as u8,
                    (average[1] / n_samples as f32) as u8,
                    (average[2] / n_samples as f32) as u8,
                ];
            }
        }

        image.save("./output.png").unwrap();

        Vec::new()
    }
}
