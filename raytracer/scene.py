from raytracer.geometry import EPSILON
from raytracer.ray import Ray
from raytracer.vec3 import Vec3


class Scene:
    def __init__(self, objects, lights, ambient_color=Vec3(1.0, 1.0, 1.0),
                 background=Vec3(0.0, 0.0, 0.0), max_depth=5):
        self.objects = list(objects)
        self.lights = list(lights)
        self.ambient_color = ambient_color
        self.background = background
        self.max_depth = int(max_depth)

    def closest_hit(self, ray, t_min=EPSILON, t_max=float("inf")):
        """Return the nearest valid Hit along the ray, or None."""
        closest = None
        closest_t = t_max
        for obj in self.objects:
            hit = obj.intersect(ray, t_min, closest_t)
            if hit is not None:
                closest = hit
                closest_t = hit.t
        return closest

    def in_shadow(self, point, normal, light):
        """Cast a shadow ray toward the light; True if anything occludes it."""
        to_light = light.position - point
        distance = to_light.length()
        direction = to_light / distance
        origin = point + normal * EPSILON
        shadow_ray = Ray(origin, direction)
        return self.closest_hit(shadow_ray, EPSILON, distance - EPSILON) is not None

    def trace(self, ray, depth=0):
        hit = self.closest_hit(ray)
        if hit is None:
            return self.background
        return self.shade(hit, ray, depth)

    def shade(self, hit, ray, depth):
        material = hit.material
        normal = hit.normal
        view_dir = (-ray.direction).normalized()

        color = material.color * self.ambient_color * material.ambient

        for light in self.lights:
            if self.in_shadow(hit.point, normal, light):
                continue
            light_dir = (light.position - hit.point).normalized()
            light_color = light.color * light.intensity
            # Lambert diffuse
            diffuse = max(0.0, normal.dot(light_dir))
            color = color + material.color * light_color * diffuse
            # Phong specular
            if material.specular > 0.0:
                reflect_dir = (-light_dir).reflect(normal)
                spec = max(0.0, reflect_dir.dot(view_dir)) ** material.shininess
                color = color + light_color * (material.specular * spec)

        # Recursive mirror reflection, bounded by max_depth.
        if material.reflectivity > 0.0 and depth < self.max_depth:
            reflect_dir = ray.direction.reflect(normal).normalized()
            origin = hit.point + normal * EPSILON
            reflected = self.trace(Ray(origin, reflect_dir), depth + 1)
            color = color * (1.0 - material.reflectivity) \
                + reflected * material.reflectivity

        return color
