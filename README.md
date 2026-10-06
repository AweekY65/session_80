# CPU Local Ray Tracer

一个纯 Python（仅标准库）实现的 CPU 本地基础光线追踪器。所有场景、几何体、
材质和输出图像只存在于本地内存或文件中：不依赖 GPU、游戏引擎、云渲染或任何
外部服务，也不需要显示窗口。输出为 ASCII PPM（P3）格式。

## 功能

- Ray / Sphere / Plane 解析求交，返回最近有效交点
- 透视（针孔）相机，按分辨率与垂直 FOV 生成主射线（穿过像素中心）
- Lambert 漫反射 + Phong 高光 + 环境光 + 点光源
- Hard shadow：向光源发射 shadow ray 判断遮挡
- 基础镜面反射，递归深度受 `max_depth` 限制
- 自相交 epsilon（`EPSILON = 1e-4`），避免 shadow acne / 自阴影

## 坐标系

右手坐标系，+Y 向上。相机位于 `position`，朝向 `look_at`，`up` 参考向量为
+Y。成像平面像素 `(0, 0)` 在左上角，主射线穿过像素中心：

```
u = (2*(px+0.5)/width  - 1) * tan(fov/2) * aspect
v = (1 - 2*(py+0.5)/height) * tan(fov/2)
dir = normalize(forward + u*right + v*up)
```

## 相交公式

- **球**：解二次方程 `|O + tD - C|^2 = r^2`，即
  `a = D·D, b = 2(O-C)·D, c = (O-C)·(O-C) - r^2`，判别式 `b^2 - 4ac`，
  取 `t_min < t` 的最小根；法线为 `(P - C) / r`。
- **平面**：`t = ((P0 - O)·N) / (D·N)`，`|D·N|` 过小视为平行不相交；
  法线始终翻转至朝向入射射线一侧。
- 场景遍历所有物体，保留 `t` 最小的有效交点（`t > EPSILON`）。

## 光照模型

对每个交点：

```
color = ambient * albedo * ambient_light
      + Σ_lights (1 - shadow) * [ albedo * I_l * max(0, N·L)                # Lambert
                                  + I_l * ks * max(0, R·V)^shininess ]      # Phong
      + reflectivity * trace(reflect_ray, depth+1)                          # 镜面反射
```

- **阴影**：从 `P + N*EPSILON` 向点光源发射 shadow ray，若任一物体在
  `t < dist(light) - EPSILON` 内相交则该光源被完全遮挡（hard shadow）。
- **反射**：`R = D - 2(D·N)N`，从偏移后的原点递归追踪，直到
  `depth == max_depth` 停止，避免镜面间无限反射。
- **自相交**：所有次级射线（阴影/反射）原点沿法线偏移 `EPSILON = 1e-4`，
  求交时也拒绝 `t <= EPSILON` 的命中，消除表面自阴影瑕疵。

## 运行

```bash
python3 render.py [output.ppm]   # 渲染 320x240 演示场景，默认写 output.ppm
```

## 测试

全部测试在终端执行，无需显示窗口：

```bash
python3 -m unittest discover -s tests -v
```

覆盖：射线-球相交（命中取最近根 / 相切 / 不相交 / 球在射线后方）、多物体
最近交点、平面相交与平行未命中、相机中心射线与视锥对称性、阴影遮挡与
epsilon 防自阴影、镜面反射颜色与递归深度上限、小分辨率（32x24）渲染的
固定像素值与整图 SHA-256 哈希稳定性。

## 目录结构

```
raytracer/
  vec3.py       向量/颜色运算
  ray.py        射线
  geometry.py   Sphere / Plane 求交与 Hit，EPSILON 常量
  material.py   Lambert/Phong 材质参数
  light.py      点光源
  camera.py     透视相机
  scene.py      最近交点、阴影、着色与递归反射
  renderer.py   渲染循环与 PPM 输出
render.py       演示场景入口
tests/          unittest 测试
```
