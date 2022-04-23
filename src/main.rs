mod vec3;
mod ray;

use vec3::{Vec3,Color};
use ray::Ray;

fn write_color(pixel_color: Color) {
    let c: f32 = 255.999;
    println!("{} {} {}\n", (pixel_color.x() * c) as i32, (pixel_color.y() * c) as i32, (pixel_color.z() * c) as i32)
}

/// Returns true if a given given ray `r` hits the inside of a sphere
/// of radius `radius`. False otherwise.
/// 
/// If a ray P(t) hits the sphere centered in `center`, there exists t for which 
/// (P(t) - C) * (P(t) - C) = radius^2.
///
/// Where C = (C_x, C_y, C_z) is the vector representing the `center` of the sphere.
/// 
/// To determine if `r` hits the sphere we need to plug in 
/// the definition of P(t) = A + t*b, do some algebra, and find to roots of:
///
/// t^2 (b*b) + 2*t*b * (A-C) + (A-C) * (A-C) - radius^2 = 0
///
/// # Arguments
/// - `center`: the center of the sphere
/// - `radius`: the radius of the sphere
/// - `r`: a ray
fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}


/// Linearly blends white and blue depending on the height of the 𝑦 coordinate 
/// after scaling the ray direction to unit length (so −1.0<𝑦<1.0). 
/// Because we're looking at the 𝑦 height after normalizing the vector, 
/// you'll notice a horizontal gradient to the color in addition to the vertical gradient.
/// 
/// # Arguments
///
/// - `r` - a struct defining origin and direction of  a ray.
fn ray_color(r: Ray) -> Color {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r) {
        // Red pixel
        return Vec3(1.0, 0.0, 0.0)       
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
 
    // Here Vec3(1.0, 1.0, 1.0) is the color white; and Vec3(0.5, 0.7, 1.0
    // is the color blue. Both are expressed as RGB values.
    // We scale 0.0 <= t <= 1 so that when t = 1.0 we get blue. When t = 0.0 we get white. 
    // In between, we get a linear blend.
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    // We setup a virtual viewport through which pass the scene rays.
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    // The distance between the projection plane and the projection point.
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);


    println!("P3\n{} {}\n{}", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u: f32 = i as f32 / (image_width - 1) as f32;
            let v: f32 = j as f32 / (image_height - 1) as f32;
            
            let direction: Vec3 = lower_left_corner + horizontal * u  + vertical * v - origin;
            
            let r = Ray(origin, direction);
            let pixel_color = ray_color(r);
            write_color(pixel_color)
       }
    }
    eprintln!("Done.");
}
