use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// 透视相机:右手坐标系,相机看向 -Z(由 look_at 决定)。
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    /// `vfov` 为垂直视场角(度)。
    pub fn new(look_from: Point3, look_at: Point3, up: Vec3, vfov: f64, width: u32, height: u32) -> Self {
        let aspect = width as f64 / height as f64;
        let theta = vfov.to_radians();
        let half_h = (theta / 2.0).tan();
        let half_w = aspect * half_h;

        let w = (look_from - look_at).normalized(); // 相机后方
        let u = up.cross(w).normalized(); // 相机右方
        let v = w.cross(u); // 相机上方

        Camera {
            origin: look_from,
            horizontal: u * (2.0 * half_w),
            vertical: v * (2.0 * half_h),
            lower_left_corner: look_from - u * half_w - v * half_h - w,
            width,
            height,
        }
    }

    /// 像素 (i, j),j 自下而上。返回穿过像素中心的主射线。
    pub fn ray_at(&self, i: u32, j: u32) -> Ray {
        let s = (i as f64 + 0.5) / self.width as f64;
        let t = (j as f64 + 0.5) / self.height as f64;
        Ray::new(
            self.origin,
            (self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin).normalized(),
        )
    }
}
