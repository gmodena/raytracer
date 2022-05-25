mod vec3;
mod ray;
mod hit;
mod sphere;

use hit::{HitRecord, Hittable};
use vec3::{Vec3,Color};
use ray::Ray;
use sphere::Sphere;

fn write_color(pixel_color: Color) {
    let c: f32 = 255.999;
    println!("{} {} {}\n", (pixel_color.x() * c) as i32, (pixel_color.y() * c) as i32, (pixel_color.z() * c) as i32)
}

/// Returns the position `t` along a ray `r`, if `r` hits the inside of a sphere.
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
fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        // t
        return -1.0
    } 
    // t
    (- half_b - discriminant.sqrt()) / a
}


/// Renders surface normals on a sphere.
/// 
/// Linearly blends white and blue depending on the height of the 𝑦 coordinate 
/// after scaling the ray direction to unit length (so −1.0<𝑦<1.0). 
/// Because we're looking at the 𝑦 height after normalizing the vector, 
/// you'll notice a horizontal gradient to the color in addition to the vertical gradient.
/// 
/// # Arguments
///
/// - `r`: a struct defining origin and direction of  a ray.
/// - `world`: a sphere implementing the `Hittable` interface. 
fn ray_color<T: Hittable>(r: Ray, world: &T) -> Color {
    let has_hit: Option<HitRecord> = world.hit(r, 0.0, f32::INFINITY);
    
    has_hit.map(|record| (record.normal + Vec3(1.0, 1.0, 1.0)) * 0.5 )
        .unwrap_or({
            let unit_direction: Vec3 = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            // Here Vec3(1.0, 1.0, 1.0) is the color white; and Vec3(0.5, 0.7, 1.0
            // is the color blue. Both are expressed as RGB values.
            // We scale 0.0 <= t <= 1 so that when t = 1.0 we get blue. When t = 0.0 we get white. 
            // In between, we get a linear blend.
            Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
        })
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

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere { 
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5 
        }),
        Box::new(Sphere { 
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0 
        })
    ];

    println!("P3\n{} {}\n{}", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u: f32 = i as f32 / (image_width - 1) as f32;
            let v: f32 = j as f32 / (image_height - 1) as f32;
            
            let direction: Vec3 = lower_left_corner + horizontal * u  + vertical * v - origin;
            
            let r = Ray(origin, direction);
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color)
       }
    }
    eprintln!("Done.");
}
