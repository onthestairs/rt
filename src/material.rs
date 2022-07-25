use crate::{
    colour::Colour,
    hittable::HitRecord,
    ray::Ray,
    v3::{random_in_unit_sphere, random_unit_vector, reflect, unit_vector, V3},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        return Lambertian { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        return Some((scattered, self.albedo.clone()));
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        let limited_fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        return Metal {
            albedo,
            fuzz: limited_fuzz,
        };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        if V3::dot(scattered.direction, hit_record.normal) <= 0.0 {
            return None;
        }
        return Some((scattered, self.albedo));
    }
}
