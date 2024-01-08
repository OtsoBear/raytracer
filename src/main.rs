mod vec;
mod ray;

use std::io::stderr;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use std::io::Write;


fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

// Calculates the color of a ray in a ray tracing context.
fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    // The `unit_direction` is the normalized direction vector of the ray.
    let unit_direction = r.direction().normalized();
    // `t` is calculated as half of the sum of the y-component of the `unit_direction` vector and 1.0.
    let t = 0.5 * (unit_direction.y() + 1.0);
    // The color of the ray is calculated by performing a linear interpolation between white and light blue.
    // The interpolation is based on the value of `t`. If `t` is 0, the color will be white, and if `t` is 1, the color will be light blue.
    // For values of `t` between 0 and 1, the color will be a blend of white and light blue.
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    // `origin` is the starting point of the ray, typically the camera position, set to the origin of the coordinate system (0, 0, 0).
    let origin = Point3::new(0.0, 0.0, 0.0);

    // `horizontal` is a vector representing the horizontal dimension of the viewport.
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);

    // `vertical` is a vector representing the vertical dimension of the viewport.
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    // `lower_left_corner` is the position of the lower left corner of the viewport.
    // It is calculated by starting at the origin, moving half the viewport to the left, half the viewport down, and moving forward by the focal length.
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
  
   // Create a new Ray object.
   // The origin of the ray is the camera position.
   // The direction of the ray is calculated by adding the lower left corner of the viewport,
   // the product of `u` and the horizontal vector, and the product of `v` and the vertical vector,
   // and then subtracting the origin.
   // This results in a ray that starts at the origin and passes through a specific point on the viewport.
   let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }

    eprintln!("Done.");
}