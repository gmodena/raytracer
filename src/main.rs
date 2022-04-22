mod vec3;

use vec3::{Vec3, Color};

fn write_color(pixel_color: Color) {
    let c: f32 = 255.999;
    println!("{} {} {}\n", (pixel_color.x() * c) as i32, (pixel_color.y() * c) as i32, (pixel_color.z() * c) as i32)
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n{}", image_width, image_height, 255);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f32 / (image_width-1) as f32;
            let g = j as f32 / (image_height-1) as f32;
            let b = 0.25;
            let pixel_color: Color = Vec3(r, g, b);
            write_color(pixel_color)
       }
    }
    eprintln!("Done.");
}
