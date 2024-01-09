use super::vec::{Vec3, Point3};
use super::ray::Ray;

/// Represents a record of a hit between a ray and a hittable object.
pub struct HitRecord {
    /// The point of intersection between the ray and the object.
    pub p: Point3,
    /// The surface normal at the point of intersection.
    pub normal: Vec3,
    /// The parameter value along the ray where the hit occurred.
    pub t: f64,
}

/// Trait representing an object that can be hit by a ray.
pub trait Hit {
    /// Determines if the ray intersects with the object within a given range of t-values.

    /// An optional hit record containing information about the intersection, or `None` if there is no intersection.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<hitrecord>;
}