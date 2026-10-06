use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// 判定有效交点的最小 t,用于避免 self-intersection(自相交/自阴影)。
pub const EPSILON: f64 = 1e-4;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    /// 始终朝向射线入射一侧的单位法线
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hittable {
    /// 返回 t > EPSILON 的最近有效交点。
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

fn make_record(ray: &Ray, t: f64, outward_normal: Vec3, material: Material) -> HitRecord {
    let point = ray.at(t);
    // 法线始终翻转至与射线方向相对的一侧
    let normal = if ray.direction.dot(outward_normal) < 0.0 {
        outward_normal
    } else {
        -outward_normal
    };
    HitRecord { t, point, normal, material }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // 解 |o + t d - c|^2 = r^2,要求 d 为单位向量
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        // 取最近的有效根(t > EPSILON)
        let mut t = (-half_b - sqrt_d) / a;
        if t <= EPSILON {
            t = (-half_b + sqrt_d) / a;
            if t <= EPSILON {
                return None;
            }
        }
        let outward = (ray.at(t) - self.center) / self.radius;
        Some(make_record(ray, t, outward, self.material))
    }
}

pub struct Plane {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalized(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // t = (p0 - o)·n / (d·n)
        let denom = ray.direction.dot(self.normal);
        if denom.abs() < 1e-12 {
            return None; // 射线与平面平行
        }
        let t = (self.point - ray.origin).dot(self.normal) / denom;
        if t <= EPSILON {
            return None;
        }
        Some(make_record(ray, t, self.normal, self.material))
    }
}
