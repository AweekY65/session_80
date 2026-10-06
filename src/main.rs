use raytracer::camera::Camera;
use raytracer::hittable::{Plane, Sphere};
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::render;
use raytracer::scene::Scene;
use raytracer::vec3::{Color, Point3, Vec3};
use std::path::Path;

fn main() {
    let (width, height) = (320, 240);

    let mut scene = Scene {
        ambient: Color::new(0.08, 0.08, 0.1),
        background: Color::new(0.02, 0.02, 0.05),
        max_depth: 4,
        ..Scene::default()
    };

    // 地面
    scene.objects.push(Box::new(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::lambert(Color::new(0.6, 0.6, 0.6)),
    )));
    // 红色漫反射球
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(-1.2, 0.0, -4.0),
        1.0,
        Material::phong(Color::new(0.8, 0.15, 0.15), Color::new(1.0, 1.0, 1.0), 32.0),
    )));
    // 镜面反射球
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(1.2, -0.2, -3.0),
        0.8,
        Material::mirror(Color::new(0.2, 0.3, 0.7), 0.6),
    )));
    // 绿色小球(用于制造阴影)
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -0.6, -2.2),
        0.4,
        Material::lambert(Color::new(0.2, 0.8, 0.2)),
    )));

    scene.lights.push(PointLight::new(
        Point3::new(4.0, 5.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let camera = Camera::new(
        Point3::new(0.0, 0.5, 2.0),
        Point3::new(0.0, -0.2, -3.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        width,
        height,
    );

    let rgb = render::render(&scene, &camera);
    let out = Path::new("output.ppm");
    render::write_ppm(out, width, height, &rgb).expect("write ppm");
    println!("wrote {} ({}x{})", out.display(), width, height);
}
