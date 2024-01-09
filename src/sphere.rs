use super::vec::{Point3, Vec3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl Hit for Sphere {
    // Calculates if a ray intersects with a sphere and returns the hit record.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<hitrecord> {
        // Calculates the vector from the ray origin to the center of the sphere.
        let oc = r.origin() - self.center;

        // Calculates the squared length of the direction vector of the ray.
        let a = r.direction().length().powi(2);

        // Calculates the value of half_b, which is the dot product of the vector oc and the direction of the ray.
        let half_b = oc.dot(r.direction());

        // Represents the discriminant of the quadratic equation used to determine if a ray intersects with a sphere.
        let c = oc.length().powi(2) - self.radius.powi(2);

        /// Calculates discriminant; returns None if ray doesn't intersect sphere.
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        /// Find the nearest root that lies in the acceptable range

        // Calculate the square root of the discriminant
        let sqrtd = discriminant.sqrt();
        // Calculate the smaller root of the quadratic equation
        let mut root = (-half_b - sqrtd) / a;
        // If the smaller root is outside the range [t_min, t_max]
        if root < t_min || t_max < root {
            // Calculate the larger root
            root = (-half_b + sqrtd) / a;
            // If the larger root is also outside the range [t_min, t_max]
            if root < t_min || t_max < root {
                // The ray does not intersect the sphere within the specified range
                return None;
            }
        }
        // Calculate the point of intersection 'p' by plugging the root into the ray equation
        let p = r.at(root);
        // Create a HitRecord to store the details of the intersection
        let rec = HitRecord {
            // 't' is the parameter at which the ray intersects the sphere
            t: root,
            // 'p' is the point of intersection
            p: p,
            // 'normal' is the normal vector at the point of intersection, calculated by subtracting
            // the sphere's center from 'p' and dividing by the sphere's radius
            normal: (p - self.center) / self.radius,
        };

        Some(rec)
    }
}