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
    pub fn new() -> Self {
		// Image
	    let aspect_ratio: f32 = 16.0 / 9.0;

        // Camera
    	// We setup a virtual viewport through which pass the scene rays.
    	let viewport_height = 2.0;
    	let viewport_width = aspect_ratio * viewport_height;
    	// The distance between the projection plane and the projection point.
    	let focal_length = 1.0;

        let origin = Vec3(0.0,
                          0.0,
                          0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0,
                                                                                  0.0,
                                                                                  focal_length);
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray(self.origin,
            self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}
