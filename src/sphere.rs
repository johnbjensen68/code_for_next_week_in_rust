use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
}
 
impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: Ray::new(cen, Vec3::new(0.0,0.0,0.0), 0.0),
            radius: r,
            mat: m,
        }
    }

    pub fn new_moving(cen1: Point3, cen2: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: Ray::new(cen1, cen2 - cen1, 0.0),
            radius: r,
            mat: m,
        }
    }
}
 
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc =  r.origin() - current_center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
 
        let sqrt_d = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }
 
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}