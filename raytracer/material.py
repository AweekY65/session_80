from raytracer.vec3 import Vec3


class Material:
    """Lambert/Phong material.

    color: diffuse albedo; ambient: ambient coefficient; specular: Phong
    specular coefficient; shininess: Phong exponent; reflectivity: fraction
    of the reflected ray's radiance mixed in (0 = matte, 1 = mirror).
    """

    def __init__(self, color, ambient=0.1, specular=0.0, shininess=32.0,
                 reflectivity=0.0):
        self.color = color
        self.ambient = float(ambient)
        self.specular = float(specular)
        self.shininess = float(shininess)
        self.reflectivity = float(reflectivity)
