import math


class Vec3:
    """Simple immutable-ish 3D vector used for points, directions and colors."""

    __slots__ = ("x", "y", "z")

    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.x = float(x)
        self.y = float(y)
        self.z = float(z)

    def __add__(self, other):
        return Vec3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other):
        return Vec3(self.x - other.x, self.y - other.y, self.z - other.z)

    def __neg__(self):
        return Vec3(-self.x, -self.y, -self.z)

    def __mul__(self, other):
        if isinstance(other, Vec3):
            return Vec3(self.x * other.x, self.y * other.y, self.z * other.z)
        return Vec3(self.x * other, self.y * other, self.z * other)

    __rmul__ = __mul__

    def __truediv__(self, scalar):
        return Vec3(self.x / scalar, self.y / scalar, self.z / scalar)

    def __eq__(self, other):
        return (
            isinstance(other, Vec3)
            and self.x == other.x
            and self.y == other.y
            and self.z == other.z
        )

    def __repr__(self):
        return f"Vec3({self.x}, {self.y}, {self.z})"

    def dot(self, other):
        return self.x * other.x + self.y * other.y + self.z * other.z

    def cross(self, other):
        return Vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )

    def length(self):
        return math.sqrt(self.dot(self))

    def normalized(self):
        return self / self.length()

    def reflect(self, normal):
        """Reflect this (incident) vector about a unit normal."""
        return self - normal * (2.0 * self.dot(normal))

    def clamp(self, lo=0.0, hi=1.0):
        return Vec3(
            min(hi, max(lo, self.x)),
            min(hi, max(lo, self.y)),
            min(hi, max(lo, self.z)),
        )
