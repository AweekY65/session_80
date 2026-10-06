use raytracer::camera::Camera;
use raytracer::hittable::{Hittable, Plane, Sphere, EPSILON};
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::ray::Ray;
use raytracer::render;
use raytracer::scene::Scene;
use raytracer::vec3::{Color, Point3, Vec3};

const TOL: f64 = 1e-9;

fn assert_vec3_close(a: Vec3, b: Vec3, tol: f64) {
    assert!(
        (a.x - b.x).abs() <= tol && (a.y - b.y).abs() <= tol && (a.z - b.z).abs() <= tol,
        "expected {:?} ~= {:?}",
        a,
        b
    );
}

fn grey_lambert() -> Material {
    Material::lambert(Color::new(0.5, 0.5, 0.5))
}

// ---------- 射线-球体相交 ----------

#[test]
fn ray_sphere_hit_two_points() {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, grey_lambert());
    let ray = Ray::new(Point3::ZERO, Vec3::new(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray).expect("should hit");
    assert!((hit.t - 4.0).abs() < TOL); // 近端交点
    assert_vec3_close(hit.point, Point3::new(0.0, 0.0, -4.0), TOL);
    assert_vec3_close(hit.normal, Vec3::new(0.0, 0.0, 1.0), TOL);
}

#[test]
fn ray_sphere_tangent() {
    // 相切:判别式为 0,仅一个交点
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, grey_lambert());
    let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    let hit = sphere.hit(&ray).expect("tangent should hit once");
    assert!((hit.t - 5.0).abs() < TOL);
    assert_vec3_close(hit.point, Point3::new(1.0, 0.0, -5.0), TOL);
}

#[test]
fn ray_sphere_miss() {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, grey_lambert());
    let ray = Ray::new(Point3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    assert!(sphere.hit(&ray).is_none());
}

#[test]
fn ray_sphere_behind_origin_miss() {
    // 球在射线反方向,不算有效交点
    let sphere = Sphere::new(Point3::new(0.0, 0.0, 5.0), 1.0, grey_lambert());
    let ray = Ray::new(Point3::ZERO, Vec3::new(0.0, 0.0, -1.0));
    assert!(sphere.hit(&ray).is_none());
}

#[test]
fn nearest_hit_among_spheres() {
    let mut scene = Scene::default();
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -5.0),
        1.0,
        grey_lambert(),
    )));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -3.0),
        1.0,
        grey_lambert(),
    )));
    let ray = Ray::new(Point3::ZERO, Vec3::new(0.0, 0.0, -1.0));
    let hit = scene.nearest_hit(&ray).expect("should hit");
    assert!((hit.t - 2.0).abs() < TOL, "nearest sphere first, got t={}", hit.t);
}

#[test]
fn ray_plane_hit_and_parallel_miss() {
    let plane = Plane::new(Point3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), grey_lambert());
    let ray = Ray::new(Point3::ZERO, Vec3::new(0.0, -1.0, 0.0));
    let hit = plane.hit(&ray).expect("should hit plane");
    assert!((hit.t - 1.0).abs() < TOL);
    assert_vec3_close(hit.normal, Vec3::new(0.0, 1.0, 0.0), TOL);

    let parallel = Ray::new(Point3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    assert!(plane.hit(&parallel).is_none());
}

// ---------- 相机 ----------

#[test]
fn camera_center_ray_points_at_look_at() {
    let look_from = Point3::new(0.0, 0.0, 0.0);
    let look_at = Point3::new(0.0, 0.0, -5.0);
    let cam = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 60.0, 101, 101);
    let ray = cam.ray_at(50, 50);
    assert_vec3_close(ray.origin, look_from, TOL);
    assert_vec3_close(ray.direction, Vec3::new(0.0, 0.0, -1.0), 1e-6);
}

#[test]
fn camera_rays_are_stable_and_symmetric() {
    let cam = Camera::new(
        Point3::ZERO,
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        101,
        101,
    );
    let left = cam.ray_at(0, 50);
    let right = cam.ray_at(100, 50);
    // 左右边缘射线关于中心对称
    assert!((left.direction.x + right.direction.x).abs() < 1e-9);
    assert!((left.direction.z - right.direction.z).abs() < 1e-9);
    // 重复生成结果一致(确定性)
    let again = cam.ray_at(0, 50);
    assert_vec3_close(left.direction, again.direction, 0.0);
}

// ---------- 阴影 ----------

fn shadow_scene(with_blocker: bool) -> (Scene, Point3) {
    let mut scene = Scene {
        ambient: Color::new(0.1, 0.1, 0.1),
        ..Scene::default()
    };
    // 地面 y = -1
    scene.objects.push(Box::new(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        grey_lambert(),
    )));
    if with_blocker {
        // 位于地面点 (0,-1,0) 与光源 (0,5,0) 之间的遮挡球
        scene.objects.push(Box::new(Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            1.0,
            grey_lambert(),
        )));
    }
    scene.lights.push(PointLight::new(Point3::new(0.0, 5.0, 0.0), Color::new(1.0, 1.0, 1.0)));
    (scene, Point3::new(0.0, -1.0, 0.0))
}

#[test]
fn point_in_shadow_gets_only_ambient() {
    let (scene, p) = shadow_scene(true);
    let n = Vec3::new(0.0, 1.0, 0.0);
    assert!(scene.in_shadow(p, n, &scene.lights[0]));

    // 从正上方对该点发射光线着色
    let ray = Ray::new(p + n, -n);
    let shaded = scene.trace(&ray, 0);
    let expected = scene.ambient.mul_color(grey_lambert().diffuse);
    assert_vec3_close(shaded, expected, 1e-9);
}

#[test]
fn unoccluded_point_is_lit() {
    let (scene, p) = shadow_scene(false);
    let n = Vec3::new(0.0, 1.0, 0.0);
    assert!(!scene.in_shadow(p, n, &scene.lights[0]));

    let ray = Ray::new(p + n, -n);
    let shaded = scene.trace(&ray, 0);
    let ambient = scene.ambient.mul_color(grey_lambert().diffuse);
    assert!(shaded.x > ambient.x + 0.1, "lit point brighter than ambient");
}

#[test]
fn no_self_shadow_acne() {
    // 无遮挡物时,地面上的点不应被自身(平面)遮挡
    let (scene, p) = shadow_scene(false);
    let n = Vec3::new(0.0, 1.0, 0.0);
    assert!(!scene.in_shadow(p + n * EPSILON, n, &scene.lights[0]));
}

// ---------- 反射与递归限制 ----------

#[test]
fn reflection_recursion_is_bounded() {
    // 两面互相正对的镜子:若递归无上限将永不返回
    let mut scene = Scene {
        max_depth: 4,
        ..Scene::default()
    };
    let mirror = Material::mirror(Color::new(0.9, 0.9, 0.9), 1.0);
    scene.objects.push(Box::new(Plane::new(
        Point3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 1.0),
        mirror,
    )));
    scene.objects.push(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, -1.0),
        mirror,
    )));
    let ray = Ray::new(Point3::ZERO, Vec3::new(0.0, 0.0, -1.0));
    let c = scene.trace(&ray, 0); // 必须能返回
    assert!(c.x.is_finite() && c.y.is_finite() && c.z.is_finite());
}

#[test]
fn mirror_reflects_neighbor_color() {
    // 镜面球旁边放红色球,镜面上应能看到红色反射
    let mut scene = Scene {
        ambient: Color::ZERO,
        max_depth: 2,
        ..Scene::default()
    };
    scene.lights.push(PointLight::new(Point3::new(0.0, 5.0, 2.0), Color::new(1.0, 1.0, 1.0)));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -5.0),
        1.0,
        Material::mirror(Color::ZERO, 1.0), // 纯镜面
    )));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(2.5, 0.0, -5.0),
        1.0,
        Material::lambert(Color::new(1.0, 0.0, 0.0)),
    )));
    // 打向镜面球朝向红球一侧的边缘
    let ray = Ray::new(
        Point3::ZERO,
        Vec3::new(0.18, 0.0, -1.0).normalized(),
    );
    let c = scene.trace(&ray, 0);
    assert!(c.x > c.y && c.x > c.z, "mirror should reflect red, got {:?}", c);
}

// ---------- 整图渲染与哈希 ----------

fn demo_scene() -> Scene {
    let mut scene = Scene {
        ambient: Color::new(0.08, 0.08, 0.1),
        background: Color::new(0.02, 0.02, 0.05),
        max_depth: 4,
        ..Scene::default()
    };
    scene.objects.push(Box::new(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::lambert(Color::new(0.6, 0.6, 0.6)),
    )));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(-1.2, 0.0, -4.0),
        1.0,
        Material::phong(Color::new(0.8, 0.15, 0.15), Color::new(1.0, 1.0, 1.0), 32.0),
    )));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(1.2, -0.2, -3.0),
        0.8,
        Material::mirror(Color::new(0.2, 0.3, 0.7), 0.6),
    )));
    scene.objects.push(Box::new(Sphere::new(
        Point3::new(0.0, -0.6, -2.2),
        0.4,
        Material::lambert(Color::new(0.2, 0.8, 0.2)),
    )));
    scene.lights.push(PointLight::new(Point3::new(4.0, 5.0, 1.0), Color::new(1.0, 1.0, 1.0)));
    scene
}

fn demo_camera(w: u32, h: u32) -> Camera {
    Camera::new(
        Point3::new(0.0, 0.5, 2.0),
        Point3::new(0.0, -0.2, -3.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        w,
        h,
    )
}

#[test]
fn rendered_image_matches_fixed_pixels_and_hash() {
    let (w, h) = (32, 24);
    let scene = demo_scene();
    let cam = demo_camera(w, h);
    let rgb = render::render(&scene, &cam);
    assert_eq!(rgb.len(), (w * h * 3) as usize);

    // 固定像素校验:左上角应为背景色
    assert_eq!(&rgb[0..3], &[5, 5, 13]);
    // 图像中心附近(红球区域)应偏红
    let center = ((12 * w + 14) * 3) as usize;
    let (r, g, b) = (rgb[center], rgb[center + 1], rgb[center + 2]);
    assert!(r > g && r > b, "center pixel should be reddish, got ({r},{g},{b})");

    // 整体哈希校验(确定性渲染)
    let hash = render::fnv1a64(&rgb);
    assert_eq!(hash, 14436556416335645920u64, "image hash changed");
}

#[test]
fn ppm_output_roundtrip() {
    let dir = std::env::temp_dir();
    let path = dir.join("raytracer_test_out.ppm");
    let rgb: Vec<u8> = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 9, 8, 7];
    render::write_ppm(&path, 2, 2, &rgb).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.starts_with("P3\n2 2\n255\n"));
    assert!(text.contains("255 0 0\n"));
    assert!(text.contains("9 8 7\n"));
    std::fs::remove_file(&path).ok();
}
