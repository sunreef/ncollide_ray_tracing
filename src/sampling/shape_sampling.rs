use nalgebra::{Point2, Point3, Vector2, Vector3};
use ncollide3d::{
    math::Isometry,
    shape::{Ball, Cuboid, Shape, ShapeHandle},
};

use std::f32::consts::{FRAC_1_PI, PI};

use crate::math::angles_to_vector;
use crate::sampling::UniformSphereSampler;

pub struct UniformShapeSampler;

impl UniformShapeSampler {
    pub fn sample(
        &self,
        shape: &ShapeHandle<f32>,
        position: &Isometry<f32>,
        samples: &Point2<f32>,
    ) -> (Point3<f32>, f32) {
        if shape.is_shape::<Ball<f32>>() {
            let sampler = UniformBallSampler::new(shape.as_shape::<Ball<f32>>().unwrap());
            let (pos, prob) = sampler.sample(samples);
            return (position * pos, prob);
        } else if shape.is_shape::<Cuboid<f32>>() {
            let sampler = UniformCuboidSampler::new(shape.as_shape::<Cuboid<f32>>().unwrap());
            let (pos, prob) = sampler.sample(samples);
            return (position * pos, prob);
        } else {
            return (Point3::new(0.0, 0.0, 0.0), 1.0f32);
        }
    }
}

struct OriginSampler;
impl OriginSampler {
    pub fn sample(&self, samples: &Point2<f32>) -> (Point3<f32>, f32) {
        (Point3::new(0.0, 0.0, 0.0), 1.0f32)
    }
}

struct UniformBallSampler<'a> {
    ball: &'a Ball<f32>,
}

impl<'a> UniformBallSampler<'a> {
    pub fn new(ball: &'a Ball<f32>) -> Self {
        UniformBallSampler { ball }
    }

    pub fn sample(&self, samples: &Point2<f32>) -> (Point3<f32>, f32) {
        let sphere_sampler = UniformSphereSampler;
        let (vector, probability) = sphere_sampler.sample(samples);
        (Point3::from(self.ball.radius() * vector), probability)
    }
}

struct UniformCuboidSampler<'a> {
    cuboid: &'a Cuboid<f32>,
}

impl<'a> UniformCuboidSampler<'a> {
    pub fn new(cuboid: &'a Cuboid<f32>) -> Self {
        UniformCuboidSampler { cuboid }
    }

    pub fn sample(&self, samples: &Point2<f32>) -> (Point3<f32>, f32) {
        let half_sizes = self.cuboid.half_extents();
        let face_area_x = 4.0 * half_sizes[1] * half_sizes[2];
        let face_area_y = 4.0 * half_sizes[0] * half_sizes[2];
        let face_area_z = 4.0 * half_sizes[0] * half_sizes[1];
        let total_area = face_area_x + face_area_y + face_area_z;

        let ratios = vec![
            face_area_x / total_area,
            (face_area_x + face_area_y) / total_area,
            1.0,
        ];

        let mut r_index = 0usize;
        for i in 0..3 {
            if ratios[i] > samples[0] {
                r_index = i;
                break;
            }
        }

        let mut modified_sample = samples[0];
        if r_index > 0 {
            modified_sample -= ratios[r_index - 1];
            modified_sample /= ratios[r_index] - ratios[r_index - 1];
        }
        if r_index == 0 {
            modified_sample /= ratios[r_index];
        }

        let mut face_sign = 1.0f32;
        if modified_sample < 0.5 {
            modified_sample *= 2.0;
        } else {
            modified_sample = 2.0 * modified_sample - 1.0;
            face_sign = -1.0f32;
        }

        let surface_point = match r_index {
            0 => Point3::new(
                face_sign * half_sizes[0],
                (2.0 * modified_sample - 1.0) * half_sizes[1],
                (2.0 * samples[1] - 1.0) * half_sizes[2],
            ),
            1 => Point3::new(
                (2.0 * modified_sample - 1.0) * half_sizes[0],
                face_sign * half_sizes[1],
                (2.0 * samples[1] - 1.0) * half_sizes[2],
            ),
            2 => Point3::new(
                (2.0 * modified_sample - 1.0) * half_sizes[0],
                (2.0 * samples[1] - 1.0) * half_sizes[1],
                face_sign * half_sizes[2],
            ),
            _ => Point3::new(0.0, 0.0, 0.0),
        };

        (surface_point, 1.0f32 / (2.0 * total_area))
    }
}
