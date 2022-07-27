use std::sync::Arc;

use crate::{
    colour::Colour,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColour, Texture},
    v3::{random_in_unit_sphere, random_unit_vector, reflect, unit_vector, V3},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        let texture = SolidColour::new(albedo);
        return Lambertian {
            albedo: Arc::new(texture),
        };
    }
    pub fn new_from_texture<T: 'static>(texture: T) -> Self
    where
        T: Texture + Send + Sync,
    {
        return Lambertian {
            albedo: Arc::new(texture),
        };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let colour = self
            .albedo
            .colour(hit_record.u, hit_record.v, hit_record.point);
        return Some((scattered, colour));
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

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        return Dielectric {
            index_of_refraction,
        };
    }
}

fn refract(uv: V3, n: V3, etai_over_etat: f64) -> V3 {
    let cos_theta = f64::min(V3::dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5);
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Colour)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = unit_vector(ray_in.direction);

        let cos_theta = f64::min(V3::dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        // due to snells law, and a sin cannot be bigger than 0
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        let scattered_ray = Ray::new(hit_record.point, direction);
        return Some((scattered_ray, attenuation));
    }
}
