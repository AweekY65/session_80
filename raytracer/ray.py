class Ray:
    """A ray with an origin and a (not necessarily unit) direction."""

    __slots__ = ("origin", "direction")

    def __init__(self, origin, direction):
        self.origin = origin
        self.direction = direction

    def at(self, t):
        return self.origin + self.direction * t
