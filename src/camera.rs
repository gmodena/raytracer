use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0; // convert degrees to radians
        let h = (theta/2.0).tan();
        

        // Camera
    	// We setup a virtual viewport through which pass the scene rays.
    	let viewport_height = 2.0 * h;
    	let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
 
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray(self.origin,
            self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin)
    }
}
