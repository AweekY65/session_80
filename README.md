# CPU Ray Tracer

纯 CPU、零第三方依赖的本地基础光线追踪器(Rust,仅标准库)。所有场景、几何体、
材质与输出图像仅存在于本地内存或文件中;不依赖 GPU、游戏引擎、云渲染或任何外部服务。
输出为 ASCII PPM (P3) 图像,测试全部在终端执行,无需显示窗口。

## 构建与运行

```bash
cargo run --release      # 渲染内置场景,输出 output.ppm (320x240)
cargo test               # 运行全部自动化测试
```

## 坐标系

右手坐标系:+X 向右,+Y 向上,+Z 朝向观察者(相机默认看向 -Z)。
相机由 `look_from` / `look_at` / `up` 构造正交基 `(u, v, w)`,其中
`w = normalize(look_from - look_at)`(相机后方)、`u = normalize(up × w)`、`v = w × u`。

## 相交公式

射线:`P(t) = O + t·D`(D 为单位向量),仅接受 `t > EPSILON`(`EPSILON = 1e-4`)
的交点,避免 self-intersection 导致的自阴影 acne。

- **球体**:解 `|O + tD - C|² = r²`,即 `a·t² + 2b·t + c = 0`,其中
  `a = D·D`,`b = (O-C)·D`,`c = (O-C)·(O-C) - r²`。
  判别式 `Δ = b² - a·c`:`Δ < 0` 不相交;`Δ = 0` 相切(单根);`Δ > 0` 两根,
  取满足 `t > EPSILON` 的最近根。法线为 `(P - C) / r`。
- **平面**:点 `P₀`、单位法线 `N`,`t = (P₀ - O)·N / (D·N)`;分母近似为 0
  时射线与平面平行,不相交。

命中记录的法线始终翻转至与入射射线相对的一侧,场景遍历所有几何体并返回
`t` 最小的最近有效交点。

## 透视相机

由垂直视场角 `vfov` 与分辨率推出视平面半尺寸
`half_h = tan(vfov/2)`,`half_w = aspect · half_h`。像素 `(i, j)` 的主射线
穿过像素中心:`s = (i+0.5)/w`,`t = (j+0.5)/h`,
`D = normalize(lower_left + s·horizontal + t·vertical - origin)`,生成完全确定、稳定。

## 光照模型

- **环境光**:`ambient × diffuse`。
- **Lambert 漫反射**:`light_color × diffuse × max(0, N·L)`。
- **Phong 高光**:`light_color × specular × max(0, R·V)^shininess`,
  其中 `R = reflect(-L, N)`。
- **Hard shadow**:从命中点沿法线偏移 `EPSILON` 后向点光源发射 shadow ray,
  若遮挡物交点 `t` 小于到光源的距离,则该光源完全不计入。
- **镜面反射**:最终颜色为 `local × (1 - reflectivity) + trace(reflect_ray) × reflectivity`,
  反射射线同样沿法线偏移 `EPSILON`,递归深度由 `scene.max_depth` 限制,避免无限反射。

## 输出

`render::render` 返回 RGB 字节缓冲(行自顶向下),`render::write_ppm`
写出 P3 格式 PPM。另提供 `render::fnv1a64` 用于图像内容的确定性哈希校验。

## 测试

```bash
cargo test
```

覆盖:射线-球体相交/相切/不相交/背向剔除、多球最近交点、平面相交与平行、
相机中心射线与对称稳定性、阴影遮挡与环境光、无自阴影 acne、双面镜反射递归上限、
镜面反射颜色,以及 32×24 小图的固定像素与整体 FNV-1a 哈希校验、PPM 文件写出。
