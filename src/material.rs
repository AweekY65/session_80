use crate::vec3::Color;

/// Blinn-Phong 风格材质:漫反射 + 高光 + 镜面反射系数。
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// 漫反射反照率 (Lambert 项)
    pub diffuse: Color,
    /// 高光颜色 (Phong 项)
    pub specular: Color,
    /// Phong 高光指数
    pub shininess: f64,
    /// 镜面反射占比 [0,1],0 为纯漫反射
    pub reflectivity: f64,
}

impl Material {
    pub fn lambert(diffuse: Color) -> Self {
        Material {
            diffuse,
            specular: Color::ZERO,
            shininess: 1.0,
            reflectivity: 0.0,
        }
    }

    pub fn phong(diffuse: Color, specular: Color, shininess: f64) -> Self {
        Material {
            diffuse,
            specular,
            shininess,
            reflectivity: 0.0,
        }
    }

    pub fn mirror(diffuse: Color, reflectivity: f64) -> Self {
        Material {
            diffuse,
            specular: Color::new(1.0, 1.0, 1.0),
            shininess: 64.0,
            reflectivity,
        }
    }
}
