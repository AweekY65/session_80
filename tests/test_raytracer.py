import hashlib
import unittest

from raytracer.camera import Camera
from raytracer.geometry import EPSILON, Plane, Sphere
from raytracer.light import PointLight
from raytracer.material import Material
from raytracer.ray import Ray
from raytracer.renderer import render, to_ppm
from raytracer.scene import Scene
from raytracer.vec3 import Vec3

WHITE = Material(Vec3(1.0, 1.0, 1.0))


def make_sphere(center=Vec3(0.0, 0.0, -5.0), radius=1.0, material=WHITE):
    return Sphere(center, radius, material)


class TestSphereIntersection(unittest.TestCase):
    def test_hit_returns_nearest_root(self):
        sphere = make_sphere()
        ray = Ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0))
        hit = sphere.intersect(ray)
        self.assertIsNotNone(hit)
        self.assertAlmostEqual(hit.t, 4.0)  # near root, not t=6
        self.assertAlmostEqual(hit.point.z, -4.0)
        self.assertAlmostEqual(hit.normal.z, 1.0)

    def test_tangent_grazes_sphere(self):
        sphere = make_sphere()
        ray = Ray(Vec3(1.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0))
        hit = sphere.intersect(ray)
        self.assertIsNotNone(hit)
        self.assertAlmostEqual(hit.t, 5.0)
        self.assertAlmostEqual(hit.point.x, 1.0)

    def test_miss_returns_none(self):
        sphere = make_sphere()
        ray = Ray(Vec3(2.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0))
        self.assertIsNone(sphere.intersect(ray))

    def test_hit_behind_origin_rejected(self):
        sphere = make_sphere()
        ray = Ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0))
        self.assertIsNone(sphere.intersect(ray))

    def test_closest_hit_among_objects(self):
        near = make_sphere(center=Vec3(0.0, 0.0, -3.0), radius=0.5)
        far = make_sphere(center=Vec3(0.0, 0.0, -8.0), radius=1.0)
        scene = Scene([far, near], [])
        ray = Ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0))
        hit = scene.closest_hit(ray)
        self.assertIsNotNone(hit)
        self.assertAlmostEqual(hit.t, 2.5)
        self.assertIs(hit.material, near.material)


class TestPlaneIntersection(unittest.TestCase):
    def test_plane_hit(self):
        plane = Plane(Vec3(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0), WHITE)
        ray = Ray(Vec3(0.0, 1.0, 0.0), Vec3(0.0, -1.0, 0.0))
        hit = plane.intersect(ray)
        self.assertIsNotNone(hit)
        self.assertAlmostEqual(hit.t, 2.0)
        self.assertAlmostEqual(hit.point.y, -1.0)

    def test_plane_parallel_miss(self):
        plane = Plane(Vec3(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0), WHITE)
        ray = Ray(Vec3(0.0, 1.0, 0.0), Vec3(1.0, 0.0, 0.0))
        self.assertIsNone(plane.intersect(ray))


class TestCamera(unittest.TestCase):
    def test_center_ray_points_at_look_at(self):
        camera = Camera(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -5.0),
                        Vec3(0.0, 1.0, 0.0), fov=60.0, width=101, height=101)
        ray = camera.generate_ray(50, 50)
        self.assertEqual(ray.origin, camera.position)
        self.assertAlmostEqual(ray.direction.x, 0.0, places=6)
        self.assertAlmostEqual(ray.direction.y, 0.0, places=6)
        self.assertAlmostEqual(ray.direction.z, -1.0, places=6)

    def test_corner_rays_are_symmetric(self):
        camera = Camera(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0),
                        Vec3(0.0, 1.0, 0.0), fov=90.0, width=100, height=100)
        tl = camera.generate_ray(0, 0).direction
        br = camera.generate_ray(99, 99).direction
        self.assertAlmostEqual(tl.x, -br.x, places=9)
        self.assertAlmostEqual(tl.y, -br.y, places=9)
        self.assertAlmostEqual(tl.z, br.z, places=9)


class TestShadow(unittest.TestCase):
    def setUp(self):
        self.light = PointLight(Vec3(0.0, 5.0, -5.0), Vec3(1.0, 1.0, 1.0))
        self.point = Vec3(0.0, 0.0, 0.0)
        self.normal = Vec3(0.0, 1.0, 0.0)

    def test_occluder_creates_shadow(self):
        blocker = make_sphere(center=Vec3(0.0, 2.5, -2.5), radius=1.0)
        scene = Scene([blocker], [self.light])
        self.assertTrue(scene.in_shadow(self.point, self.normal, self.light))

    def test_no_occluder_no_shadow(self):
        far = make_sphere(center=Vec3(10.0, 0.0, -5.0), radius=1.0)
        scene = Scene([far], [self.light])
        self.assertFalse(scene.in_shadow(self.point, self.normal, self.light))

    def test_shadowed_point_only_gets_ambient(self):
        mat = Material(Vec3(1.0, 0.0, 0.0), ambient=0.2)
        floor = Plane(Vec3(0.0, -0.5, 0.0), Vec3(0.0, 1.0, 0.0), mat)
        blocker = make_sphere(center=Vec3(0.0, 2.5, -2.5), radius=1.0,
                              material=Material(Vec3(0.0, 0.0, 0.0)))
        scene = Scene([floor, blocker], [self.light])
        color = scene.trace(Ray(Vec3(0.0, 0.5, 0.0), Vec3(0.0, -1.0, 0.0)))
        self.assertAlmostEqual(color.x, 0.2, places=6)
        self.assertAlmostEqual(color.y, 0.0, places=6)

    def test_no_self_shadow_from_epsilon(self):
        # A lit surface point must not shadow itself.
        mat = Material(Vec3(1.0, 1.0, 1.0), ambient=0.0)
        floor = Plane(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 1.0, 0.0), mat)
        scene = Scene([floor], [self.light])
        color = scene.trace(Ray(Vec3(0.0, 5.0, 0.0), Vec3(0.0, -1.0, 0.0)))
        self.assertGreater(color.x, 0.5)


class TestReflection(unittest.TestCase):
    def test_mirror_reflects_neighbor_color(self):
        red = Material(Vec3(1.0, 0.0, 0.0), ambient=1.0)
        mirror = Material(Vec3(1.0, 1.0, 1.0), ambient=0.0, reflectivity=1.0)
        ball = make_sphere(center=Vec3(0.0, 0.0, -4.0), radius=1.0,
                           material=mirror)
        wall = Plane(Vec3(0.0, 0.0, 10.0), Vec3(0.0, 0.0, -1.0), red)
        scene = Scene([ball, wall], [], max_depth=4)
        color = scene.trace(Ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0)))
        self.assertGreater(color.x, 0.9)
        self.assertAlmostEqual(color.y, 0.0, places=6)

    def test_recursion_depth_is_bounded(self):
        # Two facing mirrors with ambient-only materials: tracing must
        # terminate and respect max_depth.
        calls = []
        mirror = Material(Vec3(1.0, 1.0, 1.0), ambient=0.0, reflectivity=1.0)

        class CountingPlane(Plane):
            def intersect(self, ray, t_min=EPSILON, t_max=float("inf")):
                calls.append(ray)
                return super().intersect(ray, t_min, t_max)

        a = CountingPlane(Vec3(0.0, 0.0, -1.0), Vec3(0.0, 0.0, 1.0), mirror)
        b = CountingPlane(Vec3(0.0, 0.0, 1.0), Vec3(0.0, 0.0, -1.0), mirror)
        scene = Scene([a, b], [], max_depth=3)
        color = scene.trace(Ray(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0)))
        self.assertEqual(color, Vec3(0.0, 0.0, 0.0))
        # 1 primary + max_depth reflection traces, each testing both planes.
        self.assertEqual(len(calls), 2 * (1 + scene.max_depth))


class TestRenderOutput(unittest.TestCase):
    EXPECTED_HASH = "667ca9882d4f522308fddb01fc74be8cd04cb50417fd478b181a4e0929bcb1f4"
    WIDTH, HEIGHT = 32, 24

    @classmethod
    def build(cls):
        red = Material(Vec3(0.8, 0.2, 0.2), ambient=0.1, specular=0.5,
                       shininess=64.0, reflectivity=0.3)
        gray = Material(Vec3(0.8, 0.8, 0.8), ambient=0.1)
        objects = [
            Sphere(Vec3(0.0, 0.0, -5.0), 1.0, red),
            Plane(Vec3(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0), gray),
        ]
        lights = [PointLight(Vec3(5.0, 8.0, -2.0), Vec3(1.0, 1.0, 1.0))]
        scene = Scene(objects, lights, background=Vec3(0.05, 0.07, 0.1),
                      max_depth=5)
        camera = Camera(Vec3(0.0, 0.5, 0.0), Vec3(0.0, 0.0, -5.0),
                        Vec3(0.0, 1.0, 0.0), fov=60.0,
                        width=cls.WIDTH, height=cls.HEIGHT)
        return render(scene, camera)

    def test_center_pixel_hits_sphere(self):
        pixels = self.build()
        center = pixels[self.HEIGHT // 2][self.WIDTH // 2]
        # Center of the image is the red sphere, clearly not background.
        self.assertGreater(center.x, 0.3)
        self.assertGreater(center.x, center.y + 0.1)
        self.assertGreater(center.x, center.z + 0.1)

    def test_top_corner_pixel_is_background(self):
        pixels = self.build()
        corner = pixels[0][0]
        self.assertAlmostEqual(corner.x, 0.05, places=6)
        self.assertAlmostEqual(corner.y, 0.07, places=6)
        self.assertAlmostEqual(corner.z, 0.1, places=6)

    def test_image_hash_is_stable(self):
        ppm = to_ppm(self.build())
        digest = hashlib.sha256(ppm.encode("ascii")).hexdigest()
        self.assertEqual(digest, self.EXPECTED_HASH)


if __name__ == "__main__":
    unittest.main()
