
# Rustful Raytracer

## A raytracer made with rust

This is a simple ray tracer implemented in Rust.

The ray tracer calculates the color of each pixel in an image by tracing rays from the camera through the viewport and into the scene.
It supports rendering spheres and performs basic shading based on the intersection of the rays with the spheres.

The `main.rs` file contains the entry point of the program and defines the `main` function.
It also includes the `hit_sphere` and `ray_color` functions, which are used to calculate the intersection of rays with spheres and the color of the rays, respectively.

The `mod` keyword is used to define two modules: `vec` and `ray`.
These modules contain the implementations of vector and ray data structures used in the ray tracer.

The `use` keyword is used to import necessary types and functions from the standard library.

The program generates a PPM image with a resolution of 256x144 pixels.
The image is output to the console in P3 format, which is a text-based format for storing RGB pixel values.

To run the program, execute the `main` function.
The resulting image will be printed to the console.

### Example usage:

```rust
cargo run
```

This will compile and run the program, generating the ray traced image.
