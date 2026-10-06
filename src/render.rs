use crate::camera::Camera;
use crate::scene::Scene;
use crate::vec3::Color;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

/// 按行渲染整张图像,返回 RGB 缓冲(每像素 3 字节,行自顶向下)。
pub fn render(scene: &Scene, camera: &Camera) -> Vec<u8> {
    let mut buf = Vec::with_capacity((camera.width * camera.height * 3) as usize);
    for row in 0..camera.height {
        let j = camera.height - 1 - row; // 图像顶行对应最大 j
        for i in 0..camera.width {
            let ray = camera.ray_at(i, j);
            let c: Color = scene.trace(&ray, 0).clamp01();
            buf.push((c.x * 255.0 + 0.5) as u8);
            buf.push((c.y * 255.0 + 0.5) as u8);
            buf.push((c.z * 255.0 + 0.5) as u8);
        }
    }
    buf
}

/// 以 ASCII PPM (P3) 写出图像。
pub fn write_ppm(path: &Path, width: u32, height: u32, rgb: &[u8]) -> io::Result<()> {
    assert_eq!(rgb.len(), (width * height * 3) as usize);
    let mut s = String::with_capacity(rgb.len() * 4);
    writeln!(s, "P3").unwrap();
    writeln!(s, "{} {}", width, height).unwrap();
    writeln!(s, "255").unwrap();
    for px in rgb.chunks_exact(3) {
        writeln!(s, "{} {} {}", px[0], px[1], px[2]).unwrap();
    }
    fs::write(path, s)
}

/// FNV-1a 64 位哈希,用于测试中对图像做确定性校验。
pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
