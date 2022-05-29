mod vec3;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use hit::{HitRecord, Hittable};
use vec3::{Vec3,Color};
use ray::Ray;
use sphere::Sphere;
use camera::Camera;
use rand::random;
use material::{Dielectric, Lambertian, Metal};

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let c = 256.0;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / (samples_per_pixel as f32);
    let r = f32::sqrt(pixel_color.x() * scale);
    let g = f32::sqrt(pixel_color.y() * scale);
    let b = f32::sqrt(pixel_color.z() * scale);


    println!("{} {} {}",
             (c * clamp(r, 0.0, 0.999)) as i32,
             (c * clamp(g, 0.0, 0.999)) as i32,
             (c * clamp(b, 0.0, 0.999)) as i32)
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
/// - `depth`: max depth allowed when calling ray_color recursively.
fn ray_color<T: Hittable>(r: Ray, world: &T, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0)
    }

    // Ignore hits very near zero to fix shadow acne.
    let has_hit: Option<HitRecord> = world.hit(r, 0.001, f32::INFINITY);
 
    match has_hit {
        Some(record) => {
            match record.material.scatter(r, record) {
                Some(scattered) => 
                    scattered.attenuation * ray_color(scattered.r, world, depth-1),
                None => {
                    Vec3(0.0, 0.0, 0.0)
                }
            }
        }
        None => {
            let unit_direction: Vec3 = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            // Here Vec3(1.0, 1.0, 1.0) is the color white; and Vec3(0.5, 0.7, 1.0
            // is the color blue. Both are expressed as RGB values.
            // We scale 0.0 <= t <= 1 so that when t = 1.0 we get blue. When t = 0.0 we get white.
            // In between, we get a linear blend.
            Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
        }
    }
}


fn main() {
    // World
    let R = (std::f32::consts::PI/4.0).tan();


    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    // Camera
    let lookfrom = Vec3(3.0, 3.0, 2.0);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).length();
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, 2.0, dist_to_focus);
    let samples_per_pixel = 100;
     // Limit the number of child rays
    let max_depth = 50;


    // World
    let world: Vec<Box<dyn Hittable>> = vec![
 /*       Box::new(Sphere {  
            center: Vec3(-R, 0.0, -1.0),
            radius: R,
            material: Box::new(Lambertian {
                albedo: Vec3(0.0, 0.0, 1.0)
            })
        }),
        Box::new(Sphere {
            center: Vec3(R, 0.0, -1.0),
            radius: R,
            material: Box::new(Lambertian {
                albedo: Vec3(1.0, 0.0, 0.0)
            })
        }),
*/
        Box::new(Sphere {  // center sphere
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: Vec3(0.1, 0.2, 0.5)
            })
        }),
        Box::new(Sphere {  // ground shpehre
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.8, 0.8, 0.0)
            })
        }),
        Box::new(Sphere {  // left sphere
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Dielectric {
                index_of_refraction: 1.5 // glass
            })
        }),
        Box::new(Sphere { // left sphere
            center: Vec3(-1.0, 0.0, -1.0),
            radius: -0.45, // the surface normal points inward
             material: Box::new(Dielectric {
                index_of_refraction: 1.5 // glass
            })
        }),
        Box::new(Sphere {  // right sphere
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Vec3(0.8, 0.6, 0.2),
                fuzz: 0.0
            })
        }),
    ];

    println!("P3\n{} {}\n{}", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u: f32 = (i as f32 + random::<f32>()) / (image_width - 1) as f32;
                let v: f32 = (j as f32 + random::<f32>()) / (image_height - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
       }
    }
    eprintln!("Done.");
}
