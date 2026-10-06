from raytracer.vec3 import Vec3


def render(scene, camera):
    """Render the scene; returns a list of rows of Vec3 colors in [0, 1]."""
    pixels = []
    for py in range(camera.height):
        row = []
        for px in range(camera.width):
            color = scene.trace(camera.generate_ray(px, py))
            row.append(color.clamp())
        pixels.append(row)
    return pixels


def to_ppm(pixels):
    """Encode pixels (rows of Vec3 in [0, 1]) as an ASCII PPM (P3) string."""
    height = len(pixels)
    width = len(pixels[0]) if height else 0
    lines = ["P3", f"{width} {height}", "255"]
    for row in pixels:
        lines.append(" ".join(
            f"{int(c.x * 255 + 0.5)} {int(c.y * 255 + 0.5)} {int(c.z * 255 + 0.5)}"
            for c in row
        ))
    return "\n".join(lines) + "\n"


def write_ppm(pixels, path):
    with open(path, "w", encoding="ascii") as fh:
        fh.write(to_ppm(pixels))
