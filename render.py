"""Render a demo scene to a local PPM file. Usage: python3 render.py [out.ppm]"""
import sys

from raytracer.camera import Camera
from raytracer.geometry import Plane, Sphere
from raytracer.light import PointLight
from raytracer.material import Material
from raytracer.renderer import render, write_ppm
from raytracer.scene import Scene
from raytracer.vec3 import Vec3


def build_scene():
    red = Material(Vec3(0.8, 0.2, 0.2), ambient=0.1, specular=0.5,
                   shininess=64.0, reflectivity=0.3)
    gray = Material(Vec3(0.8, 0.8, 0.8), ambient=0.1, specular=0.2,
                    shininess=16.0)
    mirror = Material(Vec3(0.9, 0.9, 0.95), ambient=0.05, specular=0.8,
                      shininess=128.0, reflectivity=0.6)
    objects = [
        Sphere(Vec3(-1.0, 0.0, -5.0), 1.0, red),
        Sphere(Vec3(1.2, -0.3, -4.0), 0.7, mirror),
        Plane(Vec3(0.0, -1.0, 0.0), Vec3(0.0, 1.0, 0.0), gray),
    ]
    lights = [PointLight(Vec3(5.0, 8.0, -2.0), Vec3(1.0, 1.0, 1.0), 1.0)]
    return Scene(objects, lights, background=Vec3(0.05, 0.07, 0.1), max_depth=5)


def main():
    out_path = sys.argv[1] if len(sys.argv) > 1 else "output.ppm"
    width, height = 320, 240
    camera = Camera(Vec3(0.0, 0.5, 0.0), Vec3(0.0, 0.0, -5.0),
                    Vec3(0.0, 1.0, 0.0), fov=60.0, width=width, height=height)
    pixels = render(build_scene(), camera)
    write_ppm(pixels, out_path)
    print(f"wrote {out_path} ({width}x{height})")


if __name__ == "__main__":
    main()
