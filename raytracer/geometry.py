from raytracer.vec3 import Vec3

# Epsilon used to avoid self-intersection (shadow acne) when spawning
# secondary rays (shadow / reflection) and to reject hits behind the origin.
EPSILON = 1e-4


class Hit:
    __slots__ = ("t", "point", "normal", "material")

    def __init__(self, t, point, normal, material):
        self.t = t
        self.point = point
        self.normal = normal
        self.material = material


class Sphere:
    def __init__(self, center, radius, material):
        self.center = center
        self.radius = float(radius)
        self.material = material

    def intersect(self, ray, t_min=EPSILON, t_max=float("inf")):
        """Analytic ray-sphere intersection via the quadratic equation.

        Solves |O + tD - C|^2 = r^2 and returns the nearest valid Hit.
        """
        oc = ray.origin - self.center
        a = ray.direction.dot(ray.direction)
        b = 2.0 * oc.dot(ray.direction)
        c = oc.dot(oc) - self.radius * self.radius
        discriminant = b * b - 4.0 * a * c
        if discriminant < 0.0:
            return None
        sqrt_disc = discriminant ** 0.5
        for t in ((-b - sqrt_disc) / (2.0 * a), (-b + sqrt_disc) / (2.0 * a)):
            if t_min < t < t_max:
                point = ray.at(t)
                normal = (point - self.center) / self.radius
                return Hit(t, point, normal, self.material)
        return None


class Plane:
    def __init__(self, point, normal, material):
        self.point = point
        self.normal = normal.normalized()
        self.material = material

    def intersect(self, ray, t_min=EPSILON, t_max=float("inf")):
        """Ray-plane intersection: t = ((P0 - O) . N) / (D . N)."""
        denom = ray.direction.dot(self.normal)
        if abs(denom) < 1e-12:
            return None
        t = (self.point - ray.origin).dot(self.normal) / denom
        if t_min < t < t_max:
            normal = self.normal if denom < 0.0 else -self.normal
            return Hit(t, ray.at(t), normal, self.material)
        return None
