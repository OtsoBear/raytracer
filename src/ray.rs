use super::vec::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3
}

/// Represents a ray in 3D space.
///
/// A ray is defined by its origin and direction.
/// It can be used to calculate points along the ray at different distances.
impl Ray {
    /// Creates a new ray with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the ray.
    /// * `direction` - The direction vector of the ray.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance.
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    /// Returns the origin point of the ray.
    ///
    /// # Returns
    ///
    /// The origin point of the ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction vector of the ray.
    ///
    /// # Returns
    ///
    /// The direction vector of the ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Calculates a point along the ray at a given distance.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance along the ray.
    ///
    /// # Returns
    ///
    /// The point along the ray at the given distance.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}