mod vec3;
mod ray;

use vec3::{Vec3, Color};
use ray::Ray;

fn write_color(pixel_color: Color) {
    let c: f32 = 255.999;
    println!("{} {} {}\n", (pixel_color.x() * c) as i32, (pixel_color.y() * c) as i32, (pixel_color.z() * c) as i32)
}

/// Linearly blends white and blue depending on the height of the 𝑦 coordinate 
/// after scaling the ray direction to unit length (so −1.0<𝑦<1.0). 
/// Because we're looking at the 𝑦 height after normalizing the vector, 
/// you'll notice a horizontal gradient to the color in addition to the vertical gradient.
/// 
/// Arguments
///
/// * `Ray` - a struct defining origin and direction of  a ray.
fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
 
    // Here Vec3(1.0, 1.0, 1.0) is the color white; and Vec3(0.5, 0.7, 1.0
    // is the color blue. Both are expressed as RGB values.
    // We scale 0.0 <= t <= 1 so that when t = 1.0 we get blue. When t = 0.0 we get white. 
    // In between, we get a blend. This forms a linear blend.
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);


    println!("P3\n{} {}\n{}", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            /*
             *             auto u = double(i) / (image_width-1);
            auto v = double(j) / (image_height-1);
            ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            color pixel_color = ray_color(r);
            */
            let u: f32 = i as f32 / (image_width - 1) as f32;
            let v: f32 = j as f32 / (image_height - 1) as f32;
            let direction: Vec3 = lower_left_corner + horizontal * u + vertical*v - origin;
            let r = Ray(origin, direction);
            let pixel_color = ray_color(r);

            write_color(pixel_color)
       }
    }
    eprintln!("Done.");
}
