use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::{self, Vec3, random_in_unit_sphere};

pub struct Scatter {
    pub r: Ray,
    pub attenuation: Vec3
}

pub trait Material {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter> {
        let scatter_direction = if (record.normal + vec3::random_unit_vector()).near_zero() {
            record.normal
        } else {
            record.normal + vec3::random_unit_vector()
        };

        let scattered = Ray(record.p, scatter_direction);
        let attenuation = self.albedo;

        Some(Scatter{r: scattered, attenuation})
    }
} 

impl Material for Metal {
    fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<Scatter> {
        let reflected = r_in.direction().unit_vector().reflect(record.normal);
        let scattered = Ray(record.p, reflected + random_in_unit_sphere() * f32::max(self.fuzz, 1.0));
        let attenuation = self.albedo;

        if scattered.direction().dot(record.normal) > 0.0 {
            Some(Scatter{r: scattered, attenuation})
        } else {
            None
        }
    }
}
