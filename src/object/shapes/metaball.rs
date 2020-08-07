use nalgebra::{Isometry3, Point3, Unit, Vector3};
use ncollide3d::{bounding_volume, query, shape};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Metaball {
    points: Vec<Point3<f32>>,
    radius: f32,
    level: f32,
}

impl Metaball {
    pub fn new(points: Vec<Point3<f32>>, radius: f32, level: f32) -> Self {
        Metaball {
            points,
            radius,
            level,
        }
    }
}

impl shape::Shape<f32> for Metaball {
    fn aabb(&self, m: &Isometry3<f32>) -> bounding_volume::AABB<f32> {
        let transformed_points = self
            .points
            .iter()
            .map(|p| m.transform_point(p))
            .collect::<Vec<_>>();
        let (min_point, max_point) = transformed_points.iter().fold(
            (transformed_points[0].clone(), transformed_points[0].clone()),
            |(min_point, max_point), p| {
                let new_min_point = Point3::new(
                    min_point[0].min(p[0]),
                    min_point[1].min(p[1]),
                    min_point[2].min(p[2]),
                );
                let new_max_point = Point3::new(
                    max_point[0].max(p[0]),
                    max_point[1].max(p[1]),
                    max_point[2].max(p[2]),
                );
                (new_min_point, new_max_point)
            },
        );
        let one_vector = Vector3::new(1.0, 1.0, 1.0);
        let aabb_min = min_point - self.radius * one_vector;
        let aabb_max = max_point + self.radius * one_vector;
        bounding_volume::AABB::new(aabb_min, aabb_max)
    }

    fn tangent_cone_contains_dir(
        &self,
        _: shape::FeatureId,
        _: &Isometry3<f32>,
        _: Option<&[f32]>,
        _: &Unit<Vector3<f32>>,
    ) -> bool {
        false
    }
}

impl query::RayCast<f32> for Metaball {
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry3<f32>,
        ray: &query::Ray<f32>,
        max_toi: f32,
        solid: bool,
    ) -> Option<query::RayIntersection<f32>> {
        None
    }
}
