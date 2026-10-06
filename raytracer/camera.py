import math

from raytracer.ray import Ray


class Camera:
    """Pinhole perspective camera.

    Right-handed coordinate system, +Y up. The camera sits at `position`
    and looks toward `look_at`. `fov` is the vertical field of view in
    degrees. Primary rays are generated through pixel centers.
    """

    def __init__(self, position, look_at, up, fov, width, height):
        self.position = position
        self.width = int(width)
        self.height = int(height)
        self.forward = (look_at - position).normalized()
        self.right = self.forward.cross(up).normalized()
        self.up = self.right.cross(self.forward)
        half_height = math.tan(math.radians(fov) / 2.0)
        half_width = half_height * (self.width / self.height)
        self.half_height = half_height
        self.half_width = half_width

    def generate_ray(self, px, py):
        """Ray through the center of pixel (px, py); (0, 0) is top-left."""
        u = (2.0 * (px + 0.5) / self.width - 1.0) * self.half_width
        v = (1.0 - 2.0 * (py + 0.5) / self.height) * self.half_height
        direction = (self.forward + self.right * u + self.up * v).normalized()
        return Ray(self.position, direction)
