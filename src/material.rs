use crate::sphere::*;
use crate::rgb::*;
use crate::ray::*;
use crate::vector::*;
use crate::xorshift::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool;
}

pub struct Lambertian {
    albedo: RGB,
}

impl Lambertian {
    pub fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        let scatter_direction = record.normal + Vector3::randomized(rand);
        *scattered = Ray::new(record.position, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: RGB,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), record.normal);
        *scattered = Ray::new(record.position, reflected + self.fuzz * rand_unit_sphere(rand));
        *attenuation = self.albedo;
        dot(scattered.direction, record.normal) > 0.0
    }
}

fn rand_unit_sphere(rand: &mut XorShift) -> Vector3 {
    loop {
        let p = Vector3::new(rand.next_bounded(-1.0, 1.0), rand.next_bounded(-1.0, 1.0), rand.next_bounded(-1.0, 1.0));
        if dot(p, p) < 1.0 { return p; }
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut RGB, scattered: &mut Ray, rand: &mut XorShift) -> bool {
        *attenuation = RGB::new(1.0, 1.0, 1.0);
        let eoe = if record.front { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit = ray_in.direction.normalize();
        let refracted = refract(unit, record.normal, eoe);
        *scattered = Ray::new(record.position, refracted);
        true
    }
}