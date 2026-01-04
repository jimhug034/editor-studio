# 需求细分 - AI 照片编辑器

每个产品需求的详细规格说明。

---

## P0 需求（MVP 核心）

---

## R1：大而清晰的照片预览

### 描述
正在编辑的照片应占据最大屏幕空间，控件在不使用时隐藏或最小化。

### 用户场景
- 用户从手机上传照片
- 照片占据屏幕 80% 以上
- 用户在进行调整时可以清楚看到细节
- 没有微小的预览窗口

### 交互流程
```
1. 用户上传照片
2. 照片居中显示，适应屏幕（包含模式）
3. 控件显示为叠加层或滑入面板
4. 双击/点击在适应和实际大小之间切换
5. 捏合/拖动进行平移和缩放
```

### UI 规格说明
```
┌─────────────────────────────────────────────────┐
│                                                 │
│                                                 │
│                                                 │
│               [照片填满屏幕]                     │
│                                                 │
│                                                 │
│  [工具面板在需要时从底部滑上]                    │
└─────────────────────────────────────────────────┘

状态 1（空闲）：     照片占 90% 屏幕，控件隐藏
状态 2（编辑中）：   照片占 70% 屏幕，控件可见
状态 3（对比）：     分割前/后视图
```

### 技术实现
- **Rust crates**：`image`、`wgpu`、`wasm-bindgen`
- **wgpu 渲染**：所有图像操作在 GPU 上完成
- **纹理管理**：多级纹理（Mipmapped textures）实现平滑缩放
- **视口计算**：保持纵横比，中性颜色填充
- **缩放级别**：25%、50%、100%、200%、400%
- **平移**：偏移跟踪，边界限制
- **详见**：[技术架构文档](./architecture.md)

### 数据结构
```rust
struct ImageView {
    texture: wgpu::Texture,
    scale: f32,           // 当前缩放级别
    offset_x: f32,        // 平移偏移
    offset_y: f32,
    fit_mode: FitMode,    // 适应或填充
}

enum FitMode {
    Contain,   // 显示完整图像
    Cover,     // 填充视口
    Actual,    // 1:1 像素映射
}
```

### 验收标准
- [ ] 照片加载时占据视口 >75%
- [ ] 平滑缩放动画（200ms 过渡）
- [ ] 平移时有边界约束（除非缩放，否则无黑边）
- [ ] 双击/点击在适应和 100% 之间切换
- [ ] 平移/缩放 12MP 图像时最低 60fps

---

## R2：简洁的界面

### 描述
默认隐藏控件。仅显示必要的 UI。避免混乱、令人困惑的图标或过多的选项。

### 用户场景
- 初学者打开应用，看到简洁的界面
- 没有技术术语可见
- 控件仅在需要时显示
- 清晰的视觉层级

### 交互流程
```
默认状态：
┌─────────────────────────────────────────────┐
│                             [⁝] [导出]       │
│                                             │
│            [大照片预览]                      │
│                                             │
│                    [裁剪] [调整] [AI]       │
└─────────────────────────────────────────────┘

工具激活：
┌─────────────────────────────────────────────┐
│  ← 返回        调整              完成 →      │
│                                             │
│  [简化的滑块，最多 4-5 个]                   │
└─────────────────────────────────────────────┘
```

### UI 原则
| 原则 | 实现 |
|------|------|
| **渐进式披露** | 首先显示基本选项，高级选项放在"更多"后面 |
| **图标 + 标签** | 始终将图标与文字标签配对 |
| **单一操作** | 每个屏幕一个主要操作 |
| **清晰退出** | 始终可见取消/完成按钮 |
| **通俗语言** | "更亮"而非"曝光 +" |

### 技术实现
```rust
struct UIState {
    mode: UIMode,
    active_tool: Option<Tool>,
}

enum UIMode {
    Viewer,      // 最小 UI，仅照片
    Editing,     // 工具面板可见
    Comparing,   // 前/后分割
}

enum Tool {
    Crop,
    Adjust,
    Filter,
    Export,
}
```

### 验收标准
- [ ] 默认状态显示 <5 个 UI 元素
- [ ] 所有图标都有文字标签
- [ ] 主 UI 中没有技术术语
- [ ] 一键关闭任何面板
- [ ] 返回/确认按钮位置一致

---

## R3：AI 自动裁剪

### 描述
自动检测照片中的主要主体，并建议能够良好框住主体的最佳裁剪。

### 用户场景
- 用户上传一张人物偏离中心的照片
- AI 检测到人物
- AI 建议一个使主体居中和构图的裁剪
- 用户看到建议的裁剪叠加在原图上

### 交互流程
```
上传 → 分析中（1-2秒）→ 结果

┌─────────────────────────────────────────────┐
│  AI 分析完成 ✓                               │
│                                             │
│  发现：人物（94% 置信度）                    │
│  建议：4:5 竖版裁剪                          │
│                                             │
│  ┌─────────────────────────────────┐       │
│  │         ┌─────────┐             │       │
│  │         │  人物   │ ◄── 裁剪     │       │
│  │         │         │     框       │       │
│  │         └─────────┘             │       │
│  └─────────────────────────────────┘       │
│                                             │
│  [应用] [调整] [查看其他选项]                │
└─────────────────────────────────────────────┘
```

### 技术实现
- **模型**：YOLOv8-nano 用于目标检测（约 6MB）
- **输入**：图像 RGB 张量 640x640
- **输出**：边界框 + 置信度分数
- **裁剪计算**：
  - 获取主体边界框
  - 添加填充（每侧 10-20%）
  - 计算目标比例的裁剪尺寸
  - 在裁剪中居中主体

### 数据结构
```rust
struct AIDetection {
    label: String,           // "人物"、"风景"等
    confidence: f32,         // 0.0 - 1.0
    bbox: BoundingBox,       // x, y, width, height
}

struct BoundingBox {
    x: f32,  // 归一化 0-1
    y: f32,
    width: f32,
    height: f32,
}

struct CropSuggestion {
    ratio: AspectRatio,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    score: f32,  // 美学评分
}

enum AspectRatio {
    Original,
    Portrait,   // 4:5
    Square,     // 1:1
    Landscape,  // 16:9
    Story,      // 9:16
}
```

### AI 流水线
```
1. 加载图像
2. 调整大小至 640x640（保持比例，填充）
3. 运行 YOLO 推理
4. 按置信度过滤检测结果（>0.5）
5. 选择置信度最高的主体
6. 计算带填充的裁剪边界
7. 按构图规则评分（三分法）
8. 返回前 3 个建议
```

### 验收标准
- [ ] 12MP 照片分析在 <3 秒内完成
- [ ] 人物检测准确率 >85%
- [ ] 裁剪建议尊重纵横比
- [ ] 向用户显示置信度分数
- [ ] 处理多个主体（建议包含所有主体的裁剪）

---

## R4：多个裁剪预览

### 描述
并排显示 3-4 个不同的裁剪选项，允许用户比较并选择最佳选项。

### 用户场景
- AI 分析完成
- 用户看到 3-4 个带有不同裁剪的缩略图预览
- 每个预览标有用途
- 用户点击选择

### 交互流程
```
┌─────────────────────────────────────────────┐
│  选择你的最佳外观                            │
│                                             │
│  ┌────────┐  ┌────────┐  ┌────────┐       │
│  │        │  │        │  │        │       │
│  │  4:5   │  │  1:1   │  │ 16:9   │       │
│  │  竖版  │  │ 正方形 │  │ 横版   │       │
│  │        │  │        │  │        │       │
│  │  ⭐    │  │        │  │        │       │
│  └────────┘  └────────┘  └────────┘       │
│   Instagram   LinkedIn    Twitter           │
│                                             │
│  [点击选择，然后自定义/应用]                 │
└─────────────────────────────────────────────┘
```

### 技术实现
```rust
struct CropPreview {
    ratio: AspectRatio,
    thumbnail: ImageHandle,
    label: String,
    recommended: bool,
    crop_bounds: Rect,
}

fn generate_previews(detection: &AIDetection, image: &Image)
    -> Vec<CropPreview>
{
    vec![
        preview_for_ratio(detection, image, 4.0/5.0, "竖版"),
        preview_for_ratio(detection, image, 1.0, "正方形"),
        preview_for_ratio(detection, image, 16.0/9.0, "横版"),
    ]
}
```

### 验收标准
- [ ] 显示 3-4 个裁剪选项
- [ ] 每个预览清晰标记
- [ ] 推荐选项高亮显示
- [ ] 点击预览应用该裁剪
- [ ] 预览在 <500ms 内渲染

---

## R5：一键应用

### 描述
单击/单击即可应用选定的 AI 裁剪建议。无需确认对话框或多步骤。

### 用户场景
- 用户选择一个裁剪预览
- 单击"应用"按钮
- 裁剪立即应用
- 用户可以继续编辑或导出

### 交互流程
```
状态 A：已选择预览
┌─────────────────────────────────────────────┐
│  已选择：4:5 竖版                            │
│  ┌────────┐                                 │
│  │ 预览   │                                 │
│  └────────┘                                 │
│  [应用] [调整] [取消]                        │
└─────────────────────────────────────────────┘

状态 B：已应用（即时）
┌─────────────────────────────────────────────┐
│  ✓ 裁剪已应用                                │
│  ┌────────┐                                 │
│  │ 裁剪后 │  ← 实时预览                      │
│  │ 图片   │                                 │
│  └────────┘                                 │
│  [调整裁剪] [导出] [← 重新开始]              │
└─────────────────────────────────────────────┘
```

### 技术实现
```rust
struct EditState {
    original_image: Image,
    current_crop: Option<Crop>,
    edit_history: Vec<Edit>,
}

struct Crop {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    ratio: Option<f32>,
}

fn apply_crop(state: &mut EditState, crop: Crop) {
    state.current_crop = Some(crop);
    state.edit_history.push(Edit::Crop(crop));
    // 即时更新，无需确认
}
```

### 验收标准
- [ ] 单击应用裁剪
- [ ] 100ms 内视觉反馈
- [ ] 无确认对话框
- [ ] 撤销可用（R12）

---

## R6：手动微调

### 描述
应用 AI 裁剪后，用户可以手动调整裁剪框 - 移动、调整大小或更改纵横比。

### 用户场景
- AI 裁剪接近但不完美
- 用户进入"调整"模式
- 用户拖动裁剪框重新定位
- 用户拖动角调整大小
- 更改实时预览

### 交互流程
```
┌─────────────────────────────────────────────┐
│  调整你的裁剪                                │
│                                             │
│  ┌─────────────────────────────────┐       │
│  │  ╔═══════════════╗              │       │
│  │  ║    裁剪       │              │       │
│  │  ║    区域       │ ◄── 可拖动   │       │
│  │  ║              ╱│              │       │
│  │  ╚═══════════════╝              │       │
│  └─────────────────────────────────┘       │
│   ◉─── 角手柄用于调整大小                   │
│                                             │
│  纵横比：[4:5 ▼] [自由]                     │
│  [完成] [重置为 AI 建议]                     │
└─────────────────────────────────────────────┘
```

### 手势/控件
| 输入 | 操作 |
|------|------|
| 拖动中心 | 移动裁剪框 |
| 拖动角 | 调整大小（保持比例）|
| 拖动边缘 | 单方向调整大小 |
| 双指捏合 | 缩放裁剪框 |
| 点击外部 | 退出调整模式 |

### 技术实现
```rust
struct CropHandle {
    position: HandlePosition,
    hitbox: Rect,
}

enum HandlePosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Center,  // 用于移动
}

struct CropEditor {
    crop: Rect,
    ratio_constraint: Option<f32>,
    active_handle: Option<HandlePosition>,
    drag_start: Option<(f32, f32)>,
}

fn handle_touch(&mut self, pos: (f32, f32), phase: TouchPhase) {
    match phase {
        TouchPhase::Started => {
            self.active_handle = self.hit_test(pos);
            self.drag_start = Some(pos);
        }
        TouchPhase::Moved => {
            if let Some(handle) = self.active_handle {
                self.update_crop(handle, pos);
            }
        }
        TouchPhase::Ended => {
            self.active_handle = None;
            self.drag_start = None;
        }
    }
}
```

### 验收标准
- [ ] 拖动裁剪框实时更新（60fps）
- [ ] 角手柄在调整大小时保持纵横比
- [ ] 边手柄自由调整大小
- [ ] 纵横比可以锁定或解锁
- [ ] 重置按钮返回 AI 建议

---

## R7：快速性能（GPU 加速）

### 描述
所有图像操作使用 GPU 加速以实现流畅、响应的性能。调整滑块或平移时无延迟。

### 用户场景
- 用户拖动滑块
- 预览即时更新（无明显延迟）
- 用户缩放/平移图像
- 移动以 60fps 平滑进行
- 大照片（12MP）处理良好

### 性能目标
| 操作 | 目标 | 最大可接受 |
|------|------|-----------|
| 加载 12MP 照片 | 500ms | 1s |
| AI 分析 | 2s | 3s |
| 滑块更新 | 16ms（1 帧）| 33ms（2 帧）|
| 平移/缩放 | 60fps | 30fps |
| 导出 JPG | 1s | 2s |
| 内存使用 | <150MB | <250MB |

### 技术实现
```rust
// GPU 加速图像处理流水线
struct ImagePipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture: wgpu::Texture,
    sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

struct AdjustmentParams {
    brightness: f32,
    contrast: f32,
    saturation: f32,
    exposure: f32,
}

// 基于着色器的调整（WGSL）
fn apply_adjustments(&self, params: AdjustmentParams) {
    // 所有像素计算在 GPU 上
    // 无 CPU 端图像操作
}
```

### WGSL 着色器示例
```wgsl
@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var sampler: sampler;
@group(0) @binding(2) var<uniform> params: AdjustParams;

struct AdjustParams {
    brightness: f32,
    contrast: f32,
    saturation: f32,
    exposure: f32,
}

@fragment
fn main(@location(0) uv: vec2<f32>) -> vec4<f32> {
    let color = textureSample(texture, sampler, uv);

    // 在 GPU 上应用调整
    let adjusted = color * params.exposure;
    let contrasted = (adjusted - 0.5) * params.contrast + 0.5;
    let saturated = mix(
        vec3<f32>(dot(contrasted.rgb, vec3<f32>(0.299, 0.587, 0.114))),
        contrasted.rgb,
        params.saturation
    );

    return vec4<f32>(saturated + params.brightness, color.a);
}
```

### 优化策略
1. **多级纹理**：预生成多级纹理实现平滑缩放
2. **纹理流式传输**：仅在需要时加载全分辨率
3. **异步计算**：AI 推理在单独线程上
4. **防抖**：快速变化期间滑块更新批处理
5. **缓存**：渲染的裁剪结果被缓存

### 验收标准
- [ ] 平移/缩放 12MP 图像时 60fps
- [ ] 滑块更改在 1 帧（16ms）内可见
- [ ] 应用使用 <200MB RAM
- [ ] 中端手机上无明显 UI 延迟
- [ ] AI 分析 3 秒内完成

---

## R8：本地处理

### 描述
所有图像处理在设备上进行。照片不上传到外部服务器。可离线工作。

### 用户场景
- 用户在没有互联网的情况下打开应用
- 所有功能正常工作
- 无网络指示器
- 注重隐私的用户感到安全

### 架构
```
┌─────────────────────────────────────────────────────────────────┐
│                         浏览器应用                               │
│                                                                 │
│  ┌─────────────┐    ┌──────────────────────────────────────┐  │
│  │  Vue3 前端  │───→│      Rust WASM (wgpu)               │  │
│  │  UI + 状态  │    │  • 图像加载/解码                      │  │
│  └─────────────┘    │  • GPU 渲染                           │  │
│         │           │  • 滤镜处理                           │  │
│         ↓           │  • 导出编码                           │  │
│  ┌─────────────┐    └──────────────────────────────────────┘  │
│  │Canvas 2D UI │                                               │
│  │裁剪框/网格  │                                               │
│  └─────────────┘                                               │
│         ↓                                                       │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │      ONNX Runtime WebGPU (JS 侧)                          │ │
│  │  • YOLOv8-nano 模型                                       │ │
│  │  • GPU 加速推理                                           │ │
│  │  • 结果传递给 Rust                                        │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ❌ 照片数据无网络调用                                          │
│  ✓ 所有处理在本地完成 (WASM + WebGPU)                          │
└─────────────────────────────────────────────────────────────────┘
```

### 技术实现
```rust
// Rust WASM 图像处理引擎
pub struct WasmImageEditor {
    device: wgpu::Device,
    queue: wgpu::Queue,
    source_texture: wgpu::Texture,
}

// AI 推理在 JS 侧通过 ONNX Runtime WebGPU 完成
// JS 侧调用 WASM 接口传入检测结果
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn apply_detection(&mut self, bbox: AIBoundingBox) {
        // 根据 AI 检测结果计算裁剪建议
    }
}
```

### 隐私政策摘要
```
- 您的照片永远不会离开您的设备
- AI 处理在本地进行
- 不对照片内容进行分析
- 可选的匿名使用指标（选择加入）
```

### 验收标准
- [ ] 应用完全离线工作
- [ ] 无照片数据传输
- [ ] AI 模型本地运行（ONNX Runtime WebGPU）
- [ ] 向用户显示隐私通知
- [ ] 代码中无云 API 密钥

---

## R9：基本调整

### 描述
简单、必要的照片调整：亮度、对比度、饱和度。每个都有清晰的视觉标签。

### 用户场景
- 用户应用 AI 裁剪
- 照片看起来不错但可以更亮
- 用户打开"调整"面板
- 用户拖动亮度滑块
- 实时预览显示变化

### 交互流程
```
┌─────────────────────────────────────────────┐
│  调整                      [重置] [完成]    │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │                                     │   │
│  │      [照片预览]                      │   │
│  │      (实时更新)                      │   │
│  │                                     │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  自动                                       │
│  ━━━━━━━━━━━━━━━━━━━━━━━●○                 │
│                                             │
│  亮度              ━━━━━━●━━━━━━             │
│  ☀️           更暗  ◉  更亮                 │
│                                             │
│  对比度            ━━━━━━●━━━━━━             │
│  ◐           更少  ◉  更多                  │
│                                             │
│  饱和度            ━━━━━━●━━━━━━             │
│  🌈          更淡  ◉  更鲜艳                │
│                                             │
│  [取消]  [应用]                              │
└─────────────────────────────────────────────┘
```

### 调整规格

| 调整 | 范围 | 默认值 | 算法 |
|------|------|--------|------|
| **亮度** | -100 到 +100 | 0 | 向 RGB 添加常量 |
| **对比度** | -100 到 +100 | 0 | 围绕 128 的 S 曲线 |
| **饱和度** | -100 到 +100 | 0 | HSL 饱和度乘数 |
| **自动** | 不适用 | 不适用 | 基于直方图的优化 |

### 技术实现
```rust
struct Adjustments {
    brightness: f32,   // -1.0 到 1.0
    contrast: f32,     // -1.0 到 1.0
    saturation: f32,   // 0.0 到 2.0
}

fn apply_adjustments(pixel: RGB, adj: &Adjustments) -> RGB {
    let mut p = pixel;

    // 亮度
    p.r += adj.brightness * 255.0;
    p.g += adj.brightness * 255.0;
    p.b += adj.brightness * 255.0;

    // 对比度
    let factor = (259.0 * (adj.contrast * 255.0 + 255.0))
                / (255.0 * (259.0 - adj.contrast * 255.0));
    p.r = factor * (p.r - 128.0) + 128.0;
    p.g = factor * (p.g - 128.0) + 128.0;
    p.b = factor * (p.b - 128.0) + 128.0;

    // 饱和度（通过 RGB 到 HSL）
    let mut hsl = rgb_to_hsl(p);
    hsl.s *= (1.0 + adj.saturation);
    p = hsl_to_rgb(hsl);

    p.clamp(0.0, 255.0)
}
```

### 自动增强算法
```rust
fn auto_enhance(image: &Image) -> Adjustments {
    let hist = image.histogram();

    // 从直方图计算最佳亮度
    let mean_brightness = hist.mean();
    let target_brightness = 128.0;
    let brightness = (target_brightness - mean_brightness) / 128.0;

    // 从直方图分布计算对比度
    let std_dev = hist.std_deviation();
    let target_std = 85.0;
    let contrast = (target_std - std_dev) / 64.0;

    Adjustments {
        brightness: brightness.clamp(-0.5, 0.5),
        contrast: contrast.clamp(-0.5, 0.5),
        saturation: 0.0,
    }
}
```

### 验收标准
- [ ] 4 个调整：自动、亮度、对比度、饱和度
- [ ] 60fps 实时预览
- [ ] 带图标的视觉标签
- [ ] 重置按钮将所有恢复默认
- [ ] 值保持直到手动更改

---

## R10：前后对比

### 描述
轻松的方式比较原始照片和编辑版本。单击/单击或按住查看原始照片。

### 用户场景
- 用户进行了多次调整
- 用户想知道是否真的更好
- 用户按住"对比"按钮
- 按住时显示原始照片
- 释放再次显示编辑版本

### 交互流程
```
状态 1：编辑中
┌─────────────────────────────────────────────┐
│  按住对比                                    │
│  ┌─────────────────────────────────┐       │
│  │                                 │       │
│  │     [编辑后的照片]               │       │
│  │                                 │       │
│  └─────────────────────────────────┘       │
│  [👁️ 对比]  ← 按住                          │
└─────────────────────────────────────────────┘

状态 2：对比中（按住时）
┌─────────────────────────────────────────────┐
│  对比中...                                   │
│  ┌─────────────────────────────────┐       │
│  │                                 │       │
│  │     [原始照片]                   │       │
│  │                                 │       │
│  └─────────────────────────────────┘       │
│  [👁️ 对比中] ← 仍在按住                      │
└─────────────────────────────────────────────┘

替代：并排视图
┌─────────────────────────────────────────────┐
│  之前              之后                      │
│  ┌────────┐         ┌────────┐            │
│  │        │         │        │            │
│  │ 原始   │  →      │ 编辑   │            │
│  │        │         │        │            │
│  └────────┘         └────────┘            │
└─────────────────────────────────────────────┘
```

### 技术实现
```rust
struct ComparisonState {
    mode: ComparisonMode,
    is_comparing: bool,
}

enum ComparisonMode {
    PressAndHold,    // 按住查看原始
    SideBySide,      // 分割视图
    Slider,         // 可拖动分隔线
}

impl ComparisonState {
    fn display_image(&self) -> DisplayImage {
        match self.mode {
            ComparisonMode::PressAndHold => {
                if self.is_comparing {
                    DisplayImage::Original
                } else {
                    DisplayImage::Edited
                }
            }
            // ... 其他模式
        }
    }
}
```

### 验收标准
- [ ] 按住显示原始
- [ ] 释放返回编辑
- [ ] 并排视图可用
- [ ] 状态之间平滑过渡
- [ ] 清晰的视觉指示显示哪个正在显示

---

## R11：导出多种尺寸

### 描述
一次操作将编辑后的照片导出为多种尺寸/格式。针对不同社交媒体平台优化。

### 用户场景
- 用户完成编辑
- 用户点击"导出"
- 用户选择多个目标尺寸
- 用户点击"导出"
- 所有版本保存或下载为 ZIP

### 交互流程
```
┌─────────────────────────────────────────────┐
│  导出                                       │
│                                             │
│  格式：  [JPG ▼]  质量：[90%]               │
│                                             │
│  导出尺寸：                                  │
│  ☑ 原始 (4032×3024)                         │
│  ☑ 4:5 竖版 (1080×1350) - Instagram        │
│  ☐ 1:1 正方形 (1080×1080)                   │
│  ☐ 16:9 横版 (1920×1080)                    │
│  ☐ 9:16 故事 (1080×1920)                    │
│                                             │
│  文件名：[photo-edit_01]                     │
│  命名：photo-edit_01-{size}.jpg              │
│                                             │
│  [取消]  [导出 3 个文件]                     │
└─────────────────────────────────────────────┘
```

### 导出规格

| 平台 | 尺寸 | 最大尺寸 | 目标文件大小 |
|------|------|---------|-------------|
| 原始 | 按拍摄 | 最高 12MP | ~2-4MB |
| Instagram | 4:5 | 1080×1350 | ~500KB |
| 正方形 | 1:1 | 1080×1080 | ~400KB |
| Twitter | 16:9 | 1920×1080 | ~600KB |
| 故事 | 9:16 | 1080×1920 | ~500KB |
| 缩略图 | 自定义 | 400×400 | ~50KB |

### 技术实现
```rust
struct ExportConfig {
    format: ImageFormat,
    quality: u8,           // 1-100
    sizes: Vec<ExportSize>,
    filename_template: String,
}

struct ExportSize {
    label: String,
    width: u32,
    height: u32,
    fit: FitMode,
}

enum ImageFormat {
    JPG(u8),      // 质量
    PNG,
    WEBP(u8),     // 质量
}

async fn export(image: &Image, config: &ExportConfig) -> Vec<ExportedFile> {
    let mut results = Vec::new();

    for size in &config.sizes {
        let resized = image.resize(size.width, size.height, size.fit);
        let encoded = encode(&resized, config.format);

        let filename = config.filename_template
            .replace("{size}", &size.label.to_lowercase())
            .replace("{ext}", config.format.extension());

        results.push(ExportedFile {
            name: filename,
            data: encoded,
            size: encoded.len(),
        });
    }

    results
}
```

### 验收标准
- [ ] 导出为 JPG（可配置质量）
- [ ] 多种尺寸预设可用
- [ ] 批量导出创建多个文件
- [ ] 多个文件 ZIP 下载
- [ ] 自定义文件名模板

---

## R12：撤销/重做

### 描述
所有编辑操作的完整撤销/重做历史。非破坏性编辑 - 始终保留原始。

### 用户场景
- 用户进行多次调整
- 用户饱和度调整过度
- 用户点击撤销
- 最后一次更改被撤销
- 用户如需要可以重做

### 交互流程
```
┌─────────────────────────────────────────────┐
│  [←]                            [→]         │
│  撤销                            重做        │
│                                             │
│  编辑历史（可选视图）：                       │
│  ┌─────────────────────────────────┐       │
│  │ 应用了 AI 裁剪                   │       │
│  │ 调整亮度 +20                     │ ← 当前│
│  │ 调整对比度 +15                   │       │
│  │ ──────────────────────────       │       │
│  │ 调整饱和度（已撤销）             │       │
│  └─────────────────────────────────┘       │
│                                             │
│  历史：10 步（最多 50）                      │
└─────────────────────────────────────────────┘
```

### 技术实现
```rust
struct History {
    undo_stack: Vec<EditOperation>,
    redo_stack: Vec<EditOperation>,
    max_size: usize,
}

trait EditOperation {
    fn apply(&self, image: &mut Image);
    fn reverse(&self, image: &mut Image);
}

struct AdjustmentEdit {
    adjustments: Adjustments,
    previous: Adjustments,
}

impl EditOperation for AdjustmentEdit {
    fn apply(&self, image: &mut Image) {
        image.set_adjustments(self.adjustments);
    }

    fn reverse(&self, image: &mut Image) {
        image.set_adjustments(self.previous);
    }
}

impl History {
    fn push(&mut self, op: EditOperation) {
        self.undo_stack.push(op);
        self.redo_stack.clear(); // 新操作时清除重做
        self.limit_size();
    }

    fn undo(&mut self, image: &mut Image) -> Option<&EditOperation> {
        let op = self.undo_stack.pop()?;
        op.reverse(image);
        self.redo_stack.push(op.clone());
        Some(self.undo_stack.last().unwrap())
    }

    fn redo(&mut self, image: &mut Image) -> Option<&EditOperation> {
        let op = self.redo_stack.pop()?;
        op.apply(image);
        self.undo_stack.push(op.clone());
        Some(self.undo_stack.last().unwrap())
    }

    fn limit_size(&mut self) {
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }
}
```

### 内存管理
```rust
// 存储差异而非完整图像以节省内存
struct CropEdit {
    // 仅存储裁剪参数，而非完整图像
    previous_crop: Option<Crop>,
    new_crop: Crop,
}

// 对于需要完整状态的操作，使用高效存储
struct ImageState {
    // 仅存储最小表示
    adjustments: Adjustments,
    crop: Option<Crop>,
    // 原始图像引用（不复制）
    original: ImageHandle,
}
```

### 验收标准
- [ ] 通过按钮和键盘（Ctrl+Z）撤销
- [ ] 通过按钮和键盘（Ctrl+Y/Ctrl+Shift+Z）重做
- [ ] 最多存储 50 步
- [ ] 历史指示器显示位置
- [ ] 内存高效（50 步 <50MB）

---

## P1 需求（MVP 后）

---

## R13：AI 滤镜分解

### 描述
向用户显示滤镜实际更改的参数。帮助他们学习如何创建某些外观。

### 用户场景
- 用户应用"温暖夏日"滤镜
- 用户看到分解："这增加温暖，提升饱和度"
- 用户可以调整各个参数
- 用户了解滤镜如何工作

### 交互流程
```
┌─────────────────────────────────────────────┐
│  温暖夏日滤镜                                │
│                                             │
│  此滤镜的作用：                               │
│  • 增加暖色调（+15）                         │
│  • 提升饱和度（+20）                         │
│  • 略微增加对比度（+10）                      │
│                                             │
│  ┌────────┐         ┌────────┐            │
│  │ 原始   │  →      │ 滤镜后  │            │
│  └────────┘         └────────┘            │
│                                             │
│  参数（可编辑）：                             │
│  温度：    ━━━━━━━━━━━●━━━  +15            │
│  饱和度：  ━━━━━━━━━━━━●━━  +20            │
│  对比度：   ━━━━━━━━━●━━━━━  +10            │
│                                             │
│  [应用] [自定义] [取消]                       │
└─────────────────────────────────────────────┘
```

### 数据结构
```rust
struct Filter {
    name: String,
    description: String,
    parameters: Vec<FilterParam>,
    category: FilterCategory,
}

struct FilterParam {
    name: String,
    value: f32,
    adjustment_type: AdjustmentType,
    explanation: String,
}

enum FilterCategory {
    Color,      // 温度、冷色调
    Mood,       // 戏剧性、柔和
    Technical,  // 锐化、降噪
    Style,      // 复古、现代
}
```

### 验收标准
- [ ] 每个滤镜显示参数分解
- [ ] 应用后参数可编辑
- [ ] 每个参数的简单解释
- [ ] 前/后预览
- [ ] 将修改后的滤镜保存为自定义预设

---

## R14：批量 AI 裁剪

### 描述
一次用 AI 裁剪建议处理多张照片。非常适合处理摄影拍摄。

### 用户场景
- 用户度假归来有 50 张照片
- 用户选择所有照片
- 用户点击"批量 AI 裁剪"
- 应用在后台处理所有照片
- 用户审阅并批准/拒绝每张

### 交互流程
```
┌─────────────────────────────────────────────┐
│  批量处理（已选择 24 张照片）                │
│                                             │
│  进度：████████████░░░░ 12/24               │
│                                             │
│  处理中...                                   │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐          │
│  │ ✓   │ │ ✓   │ │ ⏳  │ │ ⏳  │          │
│  └─────┘ └─────┘ └─────┘ └─────┘          │
│                                             │
│  审阅已完成：                                 │
│  ┌─────┐ ┌─────┐ ┌─────┐                  │
│  │  ✓  │ │  ✓  │ │  ?  │ ← 点击审阅       │
│  └─────┘ └─────┘ └─────┘                  │
│                                             │
│  [全部批准] [导出] [取消]                     │
└─────────────────────────────────────────────┘
```

### 技术实现
```rust
struct BatchProcessor {
    queue: Vec<Image>,
    results: Vec<BatchResult>,
    max_concurrent: usize,
}

struct BatchResult {
    image: Image,
    crop_suggestions: Vec<CropSuggestion>,
    applied_crop: Option<Crop>,
    approved: bool,
}

impl BatchProcessor {
    async fn process(&mut self) -> Progress {
        let mut handles = Vec::new();

        for chunk in self.queue.chunks(self.max_concurrent) {
            for image in chunk {
                let handle = tokio::spawn(async move {
                    analyze_and_crop(image).await
                });
                handles.push(handle);
            }

            // 等待批次，收集结果
            for handle in handles {
                self.results.push(handle.await.unwrap());
            }
        }

        Progress::from_results(&self.results)
    }
}
```

### 验收标准
- [ ] 一批最多处理 100 张照片
- [ ] 显示进度指示器
- [ ] 允许单独审阅/批准
- [ ] 全部批准选项
- [ ] 后台处理（可以导航离开）

---

## R15：构图指南

### 描述
视觉叠加显示构图规则 - 三分法、黄金比例、引导线 - 帮助用户学习并改进。

### 用户场景
- 用户正在手动调整裁剪
- 用户启用"构图指南"
- 网格叠加出现在照片上
- 用户将主体与网格交点对齐
- 获得更好的构图

### 交互流程
```
┌─────────────────────────────────────────────┐
│  构图指南                      [隐藏]        │
│                                             │
│  ┌─────────────────────────────────┐       │
│  │ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄│       │
│  │ ┄                              ┄│       │
│  │ ┄   👤 ◄── 主体位于            ┄│       │
│  │ ┄   交点                        ┄│       │
│  │ ┄                              ┄│       │
│  │ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄│       │
│  │ ┄                              ┄│       │
│  │ ┄                              ┄│       │
│  │ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄│       │
│  └─────────────────────────────────┘       │
│                                             │
│  指南：  [三分法 ▼]                          │
│          ☑ 黄金螺旋                          │
│          ☐ 对角线                            │
│          ☐ 中心框架                          │
└─────────────────────────────────────────────┘
```

### 指南类型

| 指南 | 描述 | 视觉 |
|------|------|------|
| **三分法** | 带强点的 3×3 网格 | 虚线网格 |
| **黄金比例** | 基于 φ 的螺旋 | 螺旋叠加 |
| **黄金三角形** | 对角线构图 | 三角形线 |
| **中心框架** | 中心对齐 | 十字线 |
| **引导线** | 边缘检测叠加 | 高亮边缘 |

### 技术实现
```rust
struct CompositionOverlay {
    guide_type: GuideType,
    opacity: f32,
    color: Color,
}

enum GuideType {
    RuleOfThirds,
    GoldenSpiral,
    GoldenTriangle,
    CenterFrame,
}

impl CompositionOverlay {
    fn render(&self, canvas: &Canvas, bounds: Rect) {
        match self.guide_type {
            GuideType::RuleOfThirds => {
                // 绘制 3x3 网格
                for i in 1..=2 {
                    let x = bounds.width * (i as f32 / 3.0);
                    let y = bounds.height * (i as f32 / 3.0);
                    canvas.line((x, 0), (x, bounds.height), self.color);
                    canvas.line((0, y), (bounds.width, y), self.color);
                }
                // 绘制强点
                for i in 1..=2 {
                    for j in 1..=2 {
                        let x = bounds.width * (i as f32 / 3.0);
                        let y = bounds.height * (j as f32 / 3.0);
                        canvas.circle((x, y), 5, self.color);
                    }
                }
            }
            // ... 其他指南
        }
    }
}
```

### 验收标准
- [ ] 4+ 种构图指南类型
- [ ] 可调整不透明度
- [ ] 可以切换开/关
- [ ] 在图像上平滑渲染
- [ ] 每种指南类型的教育工具提示

---

## R16：社交媒体预设尺寸

### 描述
针对特定社交平台优化的一键导出预设。无需记住尺寸。

### 用户场景
- 用户想发布到 Instagram
- 用户选择"Instagram"预设
- 应用建议最佳尺寸（动态 4:5，故事 9:16）
- 用户自信地导出

### 预设规格

| 平台 | 格式 | 尺寸 | 最大大小 | 备注 |
|------|------|------|---------|------|
| **Instagram 动态** | 4:5 | 1080×1350 | 30MB | 最佳互动 |
| **Instagram 正方形** | 1:1 | 1080×1080 | 30MB | 经典 |
| **Instagram 故事** | 9:16 | 1080×1920 | 30MB | 全屏 |
| **Twitter/X** | 16:9 | 1920×1080 | 5MB | 横版 |
| **Twitter 正方形** | 1:1 | 1200×1200 | 5MB | 可选 |
| **LinkedIn** | 1:1 | 1200×1200 | 5MB | 专业 |
| **LinkedIn 横幅** | 16:9 | 1584×396 | 5MB | 封面 |
| **TikTok** | 9:16 | 1080×1920 | 50MB | 竖版 |
| **YouTube 缩略图** | 16:9 | 1280×720 | 2MB | 缩略图 |
| **Pinterest** | 2:3 | 1000×1500 | 10MB | 竖版 |

### 交互流程
```
┌─────────────────────────────────────────────┐
│  导出到...                                   │
│                                             │
│  📱 社交媒体                                  │
│  ┌─────────────────────────────────┐       │
│  │ Instagram                       │       │
│  │   • 动态帖子 (4:5)               │       │
│  │   • 故事 (9:16)                  │       │
│  │   • 个人资料照片 (1:1)           │       │
│  ├─────────────────────────────────┤       │
│  │ Twitter/X                       │       │
│  │   • 帖子 (16:9)                  │       │
│  │   • 正方形 (1:1)                 │       │
│  ├─────────────────────────────────┤       │
│  │ LinkedIn                        │       │
│  │   • 帖子 (1:1)                   │       │
│  │   • 横幅 (16:9)                  │       │
│  ├─────────────────────────────────┤       │
│  │ TikTok (9:16)                   │       │
│  │ YouTube 缩略图 (16:9)            │       │
│  └─────────────────────────────────┘       │
│                                             │
│  📐 自定义尺寸                                │
│                                             │
│  ☑ 导出所有选中的格式                         │
└─────────────────────────────────────────────┘
```

### 数据结构
```rust
struct ExportPreset {
    id: String,
    name: String,
    platform: String,
    dimensions: (u32, u32),
    max_file_size: Option<usize>,
    format: ImageFormat,
    quality: u8,
    description: String,
}

const PRESETS: &[ExportPreset] = &[
    ExportPreset {
        id: "insta-feed",
        name: "Instagram 动态",
        platform: "Instagram",
        dimensions: (1080, 1350),
        max_file_size: Some(30 * 1024 * 1024),
        format: ImageFormat::JPG(90),
        quality: 90,
        description: "最适合动态帖子",
    },
    // ... 更多预设
];
```

### 验收标准
- [ ] 10+ 平台预设
- [ ] 按平台分组
- [ ] 显示尺寸和最大文件大小
- [ ] 所选预设一键导出
- [ ] 批量导出多选

---

## R17：RAW 支持

### 描述
从相机导入和编辑 RAW 照片。保持 RAW 格式的质量和灵活性。

### 用户场景
- 用户从单反/无反相机导入 RAW
- RAW 正确加载并带有嵌入预览
- 用户使用扩展动态范围编辑
- 导出保持质量优势

### 支持的格式

| 格式 | 扩展名 | 备注 |
|------|--------|------|
| Canon RAW | .crw、.cr2、.cr3 | 最常见 |
| Nikon RAW | .nef、.nrw | |
| Sony RAW | .arw | |
| Fujifilm | .raf | 胶片模拟 |
| Adobe DNG | .dng | 通用 |
| Leica | .dng、.lfr | |
| Panasonic | .rw2 | |

### 技术挑战

| 挑战 | 解决方案 |
|------|----------|
| **文件大小** | 加载嵌入 JPEG 用于预览，导出时处理 RAW |
| **去马赛克** | 使用 LibRaw 或 rawloader 进行 Bayer 插值 |
| **色彩配置文件** | 处理期间应用相机色彩配置文件 |
| **内存** | 流式处理，不将完整 RAW 加载到内存 |

### 技术实现
```rust
// 使用 rawloader crate 或绑定 LibRaw
extern crate rawloader;

struct RawImage {
    preview: Image,      // 嵌入 JPEG 用于快速预览
    data: RawData,       // 延迟加载完整 RAW
    metadata: RawMetadata,
}

struct RawMetadata {
    camera: String,
    iso: u32,
    exposure: f32,
    aperture: f32,
    focal_length: f32,
    color_profile: ColorProfile,
}

impl RawImage {
    fn load(path: &Path) -> Result<Self> {
        let raw = rawloader::decode_file(path)?;
        let preview = raw.extract_preview()?;
        Ok(Self {
            preview,
            data: RawData::Lazy(path.to_path_buf()),
            metadata: raw.metadata(),
        })
    }

    fn develop(&self, adjustments: &Adjustments) -> Image {
        // 去马赛克 + 应用调整
        let raw = self.data.load();
        let rgb = raw.demosaic();
        let corrected = self.metadata.color_profile.apply(rgb);
        adjustments.apply_to_rgb(corrected)
    }
}
```

### 验收标准
- [ ] 支持主要相机 RAW 格式
- [ ] 使用嵌入 JPEG 快速预览
- [ ] 导出时完整 RAW 处理
- [ ] 显示 EXIF 元数据
- [ ] RAW 调整有更多空间

---

## R18：主体分离

### 描述
将主要主体与背景分离。用于创建肖像、替换背景或艺术效果。

### 用户场景
- 用户有分散注意力的背景的照片
- AI 检测并分离主体
- 背景被模糊/移除
- 主体突出

### 交互流程
```
┌─────────────────────────────────────────────┐
│  主体分离                                    │
│                                             │
│  ┌─────────────────────────────────┐       │
│  │                                 │       │
│  │      👤  检测到人物              │       │
│  │   ╔═════╗                       │       │
│  │   ║     ║ ← 选择                │       │
│  │   ╚═════╝                       │       │
│  │                                 │       │
│  └─────────────────────────────────┘       │
│                                             │
│  背景处理方式：                               │
│  ○ 模糊（肖像效果）                           │
│  ○ 移除（透明）                               │
│  ○ 替换为颜色                                │
│  ○ 替换为图像                                │
│                                             │
│  [应用] [微调蒙版] [取消]                     │
└─────────────────────────────────────────────┘
```

### 技术实现
- **模型**：SAM（Segment Anything Model）或轻量级变体
- **后备**：YOLO 检测 + GrabCut 算法
- **蒙版细化**：边缘感知平滑

```rust
struct SubjectMask {
    mask: BinaryImage,      // 0 = 背景, 1 = 主体
    confidence: f32,
    bbox: BoundingBox,
}

struct BackgroundReplacement {
    mode: ReplaceMode,
    blur_amount: Option<f32>,
    color: Option<Color>,
    replacement_image: Option<Image>,
}

enum ReplaceMode {
    Blur { amount: f32 },
    Remove,
    SolidColor(Color),
    Image(Image),
}
```

### 验收标准
- [ ] AI 自动检测主体
- [ ] 手动细化蒙版边缘
- [ ] 多种背景替换选项
- [ ] 可调强度的模糊
- [ ] 导出为带透明度的 PNG

---

## 反向需求（不要构建什么）

这些是用户明确不想要的东西，基于 Reddit 反馈：

### AR1：无复杂的目录/图库系统
**原因**：用户讨厌被锁定在专有目录中
**替代方案**：简单的基于文件的工作流程，开放文件格式

### AR2：无云端依赖
**原因**：隐私担忧，离线需求
**替代方案**：全部本地处理，仅可选云同步首选项

### AR3：无仅订阅模式
**原因**：市场订阅疲劳
**替代方案**：免费增值 + 终身购买选项

### AR4：无微小的编辑预览
**原因**：Google Photos、iOS Photos 的主要投诉
**替代方案**：始终以最大尺寸显示照片

### AR5：无手势冲突
**原因**：Google Photos 侧滑问题
**替代方案**：清晰的手势区域，无重叠操作

### AR6：无功能膨胀
**原因**：使初学者困惑
**替代方案**：渐进式披露，高级功能隐藏在"更多"后面

---

## 参考

用于痛点研究的 Reddit 来源：
- [r/GooglePixel - Google Photos 照片编辑器投诉](https://www.reddit.com/r/GooglePixel/comments/1mcg8ln/the_new_google_photos_editor_is_terrible_please/)
- [r/ios - iOS 18 照片应用问题](https://www.reddit.com/r/ios/comments/1gw228u/im_still_not_used_to_this_new_photo_app_on_ios_18/)
- [r/Lightroom - 易用性问题](https://www.reddit.com/r/Lightroom/comments/1kbguc3/why_is_lightroom_so_unusable_not_talking_about/)
- [r/photography - Lightroom 替代品](https://www.reddit.com/r/photography/comments/16hml4d/lightroom_alternatives/)
- [r/IPhoneApps - 照片编辑器测试](https://www.reddit.com/r/IPhoneApps/comments/1mlomvw/i_tested_21_photo_editing_apps_heres_whats/)
