use crate::hittable::{HitRecord, Hittable, EPSILON};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::vec3::Color;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<PointLight>,
    /// 环境光颜色(各物体乘以自身 diffuse)
    pub ambient: Color,
    pub background: Color,
    /// 镜面反射最大递归深度
    pub max_depth: u32,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: Color::new(0.05, 0.05, 0.05),
            background: Color::ZERO,
            max_depth: 4,
        }
    }
}

impl Scene {
    pub fn nearest_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut best: Option<HitRecord> = None;
        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray) {
                if best.map_or(true, |b| hit.t < b.t) {
                    best = Some(hit);
                }
            }
        }
        best
    }

    /// 判断从 `point` 沿法线偏移后,到 `light` 之间是否被遮挡(hard shadow)。
    pub fn in_shadow(&self, point: crate::vec3::Point3, normal: crate::vec3::Vec3, light: &PointLight) -> bool {
        let to_light = light.position - point;
        let dist = to_light.length();
        let dir = to_light / dist;
        // 沿法线偏移 epsilon,避免自阴影
        let shadow_ray = Ray::new(point + normal * EPSILON, dir);
        if let Some(hit) = self.nearest_hit(&shadow_ray) {
            hit.t < dist
        } else {
            false
        }
    }

    pub fn trace(&self, ray: &Ray, depth: u32) -> Color {
        let hit = match self.nearest_hit(ray) {
            Some(h) => h,
            None => return self.background,
        };
        let mat = hit.material;
        let n = hit.normal;
        let view_dir = -ray.direction;

        // 环境光
        let mut local = self.ambient.mul_color(mat.diffuse);

        for light in &self.lights {
            if self.in_shadow(hit.point, n, light) {
                continue; // hard shadow:该光源被完全遮挡
            }
            let to_light = light.position - hit.point;
            let light_dir = to_light.normalized();

            // Lambert 漫反射
            let ndotl = n.dot(light_dir).max(0.0);
            local += light.color.mul_color(mat.diffuse) * ndotl;

            // Phong 高光
            if ndotl > 0.0 && mat.shininess > 0.0 {
                let reflect_dir = (-light_dir).reflect(n);
                let spec = reflect_dir.dot(view_dir).max(0.0).powf(mat.shininess);
                local += light.color.mul_color(mat.specular) * spec;
            }
        }

        // 镜面反射(受递归深度限制)
        if mat.reflectivity > 0.0 && depth < self.max_depth {
            let reflect_dir = ray.direction.reflect(n).normalized();
            let reflect_ray = Ray::new(hit.point + n * EPSILON, reflect_dir);
            let reflected = self.trace(&reflect_ray, depth + 1);
            local * (1.0 - mat.reflectivity) + reflected * mat.reflectivity
        } else {
            local
        }
    }
}
