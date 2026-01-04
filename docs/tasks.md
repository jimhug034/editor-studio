# MVP 任务拆解

按照 MVP 核心需求拆解的开发任务，按依赖关系和优先级排序。

---

## Phase 1: 项目基础设施 (Week 1)

### 1.1 项目初始化

**优先级**: P0 | **估时**: 1 天

```
[ ] 创建 Monorepo 结构
[ ] 配置 pnpm workspace
[ ] 创建 packages/frontend (Vue3)
[ ] 创建 packages/wasm-engine (Rust)
[ ] 配置 TypeScript
[ ] 配置 Vite > 8.x
[ ] 配置 Oxlint + Oxfmt
```

**目录结构**:
```
editor-studio/
├── packages/
│   ├── frontend/           # Vue3 前端
│   │   ├── src/
│   │   ├── package.json
│   │   └── vite.config.ts
│   └── wasm-engine/        # Rust WASM
│       ├── Cargo.toml
│       └── src/
├── docs/
├── package.json            # 根 package.json
└── pnpm-workspace.yaml
```

---

### 1.2 Rust WASM 基础框架

**优先级**: P0 | **估时**: 2 天

**依赖**: 1.1

```
[ ] 配置 Cargo.toml (wgpu, wasm-bindgen, image)
[ ] 创建 lib.rs (wasm-bindgen 入口)
[ ] 实现 WasmImageEditor 结构体
[ ] 实现 wgpu 设备初始化
[ ] 实现 surface 配置
[ ] 编写基础 WGSL 着色器 (全屏四边形)
[ ] 测试 WASM 编译 (wasm-pack)
```

**验收标准**:
- [ ] `wasm-pack build --target web` 成功编译
- [ ] 生成的 .wasm 文件 < 500KB (gzip 后)

---

### 1.3 前端基础框架

**优先级**: P0 | **估时**: 1-2 天

**依赖**: 1.1

```
[ ] 创建 Vue3 应用入口
[ ] 配置 Vue Router
[ ] 配置 Pinia (状态管理)
[ ] 创建基础布局组件
[ ] 集成 Tailwind CSS
[ ] 创建 WASM 加载封装
[ ] 实现错误边界
```

**验收标准**:
- [ ] 开发服务器启动成功
- [ ] WASM 模块成功加载
- [ ] 页面显示 "Editor Studio" 标题

---

## Phase 2: 图像渲染引擎 (Week 1-2)

### 2.1 图像加载与显示 (R1)

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 1.2, 1.3

```
Rust 侧:
[ ] 实现图像解码 (image crate)
[ ] 创建 wgpu 纹理
[ ] 实现纹理上传到 GPU
[ ] 实现全屏渲染
[ ] 实现 contain/cover 模式
[ ] 暴露 load_image() 接口

前端侧:
[ ] 创建文件上传组件
[ ] 实现拖放上传
[ ] 实现点击选择文件
[ ] 显示加载进度
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn load_image(&mut self, data: &[u8]) -> Result<(), JsValue>;
    pub fn get_image_size(&self) -> JsValue; // {width, height}
}
```

**验收标准**:
- [ ] 支持 JPEG/PNG/WEBP 格式
- [ ] 12MP 图像加载 < 1 秒
- [ ] 图像居中显示，保持纵横比
- [ ] 拖放上传工作正常

---

### 2.2 图像变换 (平移/缩放)

**优先级**: P0 | **估时**: 2 天

**依赖**: 2.1

```
Rust 侧:
[ ] 实现视图变换矩阵
[ ] 支持平移 (offset_x, offset_y)
[ ] 支持缩放 (scale: 0.25 - 4.0)
[ ] 实现边界约束
[ ] 暴露 transform 相关接口

前端侧:
[ ] 实现鼠标拖拽平移
[ ] 实现滚轮缩放
[ ] 实现双击切换适应/100%
[ ] 添加缩放级别指示器
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn set_transform(&mut self, offset_x: f32, offset_y: f32, scale: f32);
    pub fn reset_view(&mut self);
}
```

**验收标准**:
- [ ] 平移/缩放 60fps 流畅
- [ ] 缩放级别: 25%, 50%, 100%, 200%, 400%
- [ ] 双击在适应和 100% 之间切换

---

## Phase 3: UI 叠加层 (Week 2)

### 3.1 Canvas 2D 叠加层

**优先级**: P0 | **估时**: 1-2 天

**依赖**: 2.1

```
前端侧:
[ ] 创建 Canvas 2D 叠加层
[ ] 实现层级管理 (wgpu canvas 下方, canvas 2d 上方)
[ ] 实现响应式尺寸同步
[ ] 处理 DPI/Retina 显示
```

**验收标准**:
- [ ] 两个 Canvas 完美重叠
- [ ] 窗口调整时正确同步
- [ ] 支持高 DPI 显示

---

### 3.2 裁剪框 UI (R5 前置)

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 3.1

```
前端侧 (Canvas 2D):
[ ] 绘制裁剪框矩形
[ ] 绘制角手柄 (4 个)
[ ] 绘制边缘手柄 (4 个)
[ ] 实现手柄悬停效果
[ ] 实现手柄光标变化
[ ] 绘制三分法网格

前端侧 (交互):
[ ] 实现手柄拖拽检测
[ ] 实现裁剪框移动
[ ] 实现角拖拽 (保持比例)
[ ] 实现边缘拖拽
[ ] 传递裁剪坐标到 Rust
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn set_crop(&mut self, x: f32, y: f32, width: f32, height: f32);
    pub fn set_crop_ratio(&mut self, ratio: Option<f32>); // None = 自由
}
```

**验收标准**:
- [ ] 裁剪框响应 60fps
- [ ] 角拖拽保持纵横比
- [ ] 三分法网格可切换

---

## Phase 4: 图像调整功能 (Week 2-3)

### 4.1 基础调整 (R6)

**优先级**: P0 | **估时**: 3-4 天

**依赖**: 2.1

```
Rust 侧:
[ ] 创建 Uniform Buffer (AdjustmentParams)
[ ] 实现亮度着色器
[ ] 实现对比度着色器
[ ] 实现饱和度着色器
[ ] 实现参数更新机制
[ ] 暴露调整接口

前端侧:
[ ] 创建调整面板组件
[ ] 实现亮度滑块 (-100 到 +100)
[ ] 实现对比度滑块 (-100 到 +100)
[ ] 实现饱和度滑块 (-100 到 +100)
[ ] 实时预览
[ ] 添加重置按钮
```

**着色器实现**:
```wgsl
struct AdjustmentParams {
    brightness: f32,  // -1.0 到 1.0
    contrast: f32,    // 0.0 到 2.0
    saturation: f32,  // 0.0 到 2.0
    _padding: f32,
}
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn set_brightness(&mut self, value: f32);
    pub fn set_contrast(&mut self, value: f32);
    pub fn set_saturation(&mut self, value: f32);
    pub fn reset_adjustments(&mut self);
}
```

**验收标准**:
- [ ] 滑块拖动 <16ms 响应 (60fps)
- [ ] 调整效果实时可见
- [ ] 重置恢复默认值

---

### 4.2 自动增强

**优先级**: P1 | **估时**: 2-3 天

**依赖**: 4.1

```
Rust 侧:
[ ] 创建计算着色器 (直方图)
[ ] 实现直方图计算
[ ] 实现最佳参数计算算法
[ ] 暴露 auto_enhance() 接口

前端侧:
[ ] 添加"自动"按钮
[ ] 显示加载状态
[ ] 应用计算结果
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub async fn auto_enhance(&mut self) -> Result<(), JsValue>;
}
```

**验收标准**:
- [ ] 自动增强 <500ms 完成
- [ ] 结果视觉上改善明显

---

## Phase 5: AI 裁剪功能 (Week 3-4)

### 5.1 ONNX Runtime 集成

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 1.3

```
前端侧:
[ ] 安装 onnxruntime-web
[ ] 下载 YOLOv8-nano 模型
[ ] 创建 AI 服务模块
[ ] 实现模型初始化
[ ] 实现图像预处理 (resize to 640x640)
[ ] 实现推理调用
[ ] 实现后处理 (NMS, 置信度过滤)
```

**接口定义**:
```typescript
interface AIDetection {
  label: string;      // "person", "object", etc.
  confidence: number; // 0.0 - 1.0
  bbox: {            // 归一化坐标
    x: number;
    y: number;
    width: number;
    height: number;
  };
}

class AIService {
  async init(): Promise<void>;
  async detect(imageData: ImageData): Promise<AIDetection[]>;
}
```

**验收标准**:
- [ ] 模型加载 <3 秒
- [ ] 单张推理 <2 秒
- [ ] 支持人物检测准确率 >85%

---

### 5.2 AI 裁剪建议计算 (R2)

**优先级**: P0 | **估时**: 2 天

**依赖**: 5.1

```
Rust 侧:
[ ] 实现裁剪建议算法
[ ] 计算带填充的主体边界框
[ ] 计算不同比例的裁剪
[ ] 评分裁剪质量 (构图规则)
[ ] 暴露裁剪建议接口

前端侧:
[ ] 调用 AI 检测
[ ] 传递结果到 Rust
[ ] 显示分析状态
[ ] 处理检测失败情况
```

**接口定义**:
```rust
#[wasm_bindgen]
#[derive(Clone)]
pub struct CropSuggestion {
    pub ratio: f32,        // 宽高比
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub score: f32,        // 0.0 - 1.0
    pub label: String,
}

#[wasm_bindgen]
impl WasmImageEditor {
    pub fn calculate_crop_suggestions(
        &self,
        bbox: &AIBoundingBox,
    ) -> JsValue; // Vec<CropSuggestion>
}
```

**验收标准**:
- [ ] 返回 3-4 个裁剪建议
- [ ] 建议正确框住主体
- [ ] 计算时间 <100ms

---

### 5.3 裁剪预览 (R3)

**优先级**: P0 | **估时**: 2 天

**依赖**: 5.2

```
Rust 侧:
[ ] 实现裁剪预览渲染
[ ] 支持同时显示多个裁剪
[ ] 生成缩略图纹理

前端侧:
[ ] 创建裁剪预览组件
[ ] 网格显示 3-4 个预览
[ ] 标记推荐选项
[ ] 显示平台标签
[ ] 处理预览点击
```

**验收标准**:
- [ ] 4 个预览 <500ms 渲染
- [ ] 推荐选项有视觉突出
- [ ] 点击预览选中裁剪

---

### 5.4 一键应用 (R4)

**优先级**: P0 | **估时**: 1 天

**依赖**: 5.3

```
前端侧:
[ ] 实现"应用推荐"按钮
[ ] 点击应用最佳裁剪
[ ] 切换到编辑界面
[ ] 添加撤销支持
```

**验收标准**:
- [ ] 单击应用裁剪
- [ ] <100ms 视觉反馈
- [ ] 无确认对话框

---

### 5.5 手动裁剪调整 (R5)

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 3.2, 5.2

```
Rust 侧:
[ ] 实现裁剪区域约束
[ ] 实现比例锁定
[ ] 支持预设比例切换
[ ] 暴露裁剪更新接口

前端侧:
[ ] 连接裁剪框 UI 与 Rust
[ ] 实时更新裁剪预览
[ ] 添加比例选择器
[ ] 添加重置按钮 (恢复 AI 建议)
```

**预设比例**:
```typescript
const ASPECT_RATIOS = [
  { label: '4:5 竖版', ratio: 4/5, platform: 'Instagram' },
  { label: '1:1 正方', ratio: 1, platform: 'LinkedIn' },
  { label: '16:9 横版', ratio: 16/9, platform: 'Twitter' },
  { label: '9:16 故事', ratio: 9/16, platform: 'Instagram Story' },
  { label: '原始', ratio: null, platform: null },
];
```

**验收标准**:
- [ ] 拖拽 60fps 更新
- [ ] 比例锁定正确工作
- [ ] 重置恢复 AI 建议

---

## Phase 6: 导出功能 (Week 4)

### 6.1 图像导出基础 (R8)

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 2.1, 4.1

```
Rust 侧:
[ ] 实现纹理读取 (GPU → CPU)
[ ] 实现 JPEG 编码
[ ] 实现 PNG 编码
[ ] 实现 WEBP 编码
[ ] 支持质量控制
[ ] 暴露导出接口

前端侧:
[ ] 创建导出面板组件
[ ] 实现格式选择器
[ ] 实现质量滑块
[ ] 实现下载功能
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn export(&self, format: ExportFormat, quality: u8) -> Vec<u8>;
}

#[wasm_bindgen]
pub enum ExportFormat {
    Jpeg,
    Png,
    WebP,
}
```

**验收标准**:
- [ ] 单张导出 <2 秒
- [ ] 支持 JPG/PNG/WEBP
- [ ] 质量参数生效

---

### 6.2 批量导出 (R9)

**优先级**: P0 | **估时**: 2-3 天

**依赖**: 6.1

```
Rust 侧:
[ ] 实现批量调整大小
[ ] 使用计算着色器并行处理
[ ] 实现多文件打包
[ ] 暴露批量导出接口

前端侧:
[ ] 创建批量导出面板
[ ] 显示尺寸预设列表
[ ] 支持多选
[ ] 显示预计文件大小
[ ] 实现 ZIP 打包下载
```

**接口定义**:
```rust
#[wasm_bindgen]
#[derive(Clone)]
pub struct ExportSize {
    pub label: String,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
impl WasmImageEditor {
    pub async fn export_batch(
        &self,
        sizes: &[ExportSize],
        format: ExportFormat,
        quality: u8,
    ) -> JsValue; // Vec<EncodedFile>
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct EncodedFile {
    pub name: String,
    pub data: Vec<u8>,
    pub size: usize,
}
```

**验收标准**:
- [ ] 5 个尺寸导出 <5 秒
- [ ] 支持预设平台尺寸
- [ ] 生成 ZIP 文件下载

---

## Phase 7: 撤销/重做 (Week 5)

### 7.1 历史管理 (R10)

**优先级**: P1 | **估时**: 2-3 天

**依赖**: 4.1, 5.5

```
Rust 侧:
[ ] 设计历史栈结构
[ ] 实现操作记录 (裁剪, 调整)
[ ] 实现撤销逻辑
[ ] 实现重做逻辑
[ ] 限制历史大小 (50 步)
[ ] 暴露撤销/重做接口

前端侧:
[ ] 添加撤销/重做按钮
[ ] 绑定键盘快捷键 (Ctrl+Z, Ctrl+Y)
[ ] 显示历史状态指示器
```

**接口定义**:
```rust
#[wasm_bindgen]
impl WasmImageEditor {
    pub fn undo(&mut self) -> bool;
    pub fn redo(&mut self) -> bool;
    pub fn can_undo(&self) -> bool;
    pub fn can_redo(&self) -> bool;
}
```

**验收标准**:
- [ ] 最多保存 50 步历史
- [ ] 撤销/重做 <100ms
- [ ] 内存占用 <50MB (50 步)

---

## Phase 8: UI/UX 完善 (Week 5)

### 8.1 前后对比

**优先级**: P1 | **估时**: 1-2 天

```
前端侧:
[ ] 实现按住对比功能
[ ] 实现并排对比视图
[ ] 添加对比按钮
[ ] 平滑过渡动画
```

---

### 8.2 响应式设计

**优先级**: P1 | **估时**: 2 天

```
前端侧:
[ ] 适配桌面 (>1024px)
[ ] 适配平板 (768-1024px)
[ ] 测试不同分辨率
[ ] 优化触摸交互
```

---

### 8.3 加载与错误状态

**优先级**: P1 | **估时**: 1 天

```
前端侧:
[ ] 添加加载动画
[ ] 添加错误提示
[ ] 添加空状态提示
[ ] 优化 WebGPU 不支持的降级提示
```

---

## Phase 9: 测试与优化 (Week 6)

### 9.1 性能优化

**优先级**: P0 | **估时**: 2-3 天

```
[ ] 性能分析 (Chrome DevTools)
[ ] 优化 WASM 文件大小
[ ] 实现纹理懒加载
[ ] 优化着色器编译
[ ] 减少不必要的渲染
[ ] 实现防抖/节流
```

---

### 9.2 浏览器兼容性

**优先级**: P1 | **估时**: 1-2 天

```
[ ] Chrome 测试
[ ] Edge 测试
[ ] Firefox 测试 (需手动开启 WebGPU)
[ ] Safari 降级提示
```

---

### 9.3 真机测试

**优先级**: P0 | **估时**: 1-2 天

```
[ ] 测试大图片 (20MP+)
[ ] 测试批量处理
[ ] 测试不同格式
[ ] 内存泄漏检查
```

---

## 任务依赖图

```
Phase 1: 项目基础设施
    ├── 1.1 项目初始化
    ├── 1.2 Rust WASM 基础框架 ─────────────┐
    └── 1.3 前端基础框架 ────────────┐       │
                                       │       │
Phase 2: 图像渲染引擎                 │       │
    ├── 2.1 图像加载与显示 ───────────┼───────┘
    └── 2.2 图像变换 ─────────────────┘
            │
Phase 3: UI 叠加层                   │
    ├── 3.1 Canvas 2D 叠加层 ─────────┘
    └── 3.2 裁剪框 UI ──────────┐
                              │
Phase 4: 图像调整功能           │
    ├── 4.1 基础调整 ───────────┘
    └── 4.2 自动增强
            │
Phase 5: AI 裁剪功能
    ├── 5.1 ONNX Runtime 集成 ───┐
    ├── 5.2 AI 裁剪建议计算 ──────┼───┐
    ├── 5.3 裁剪预览 ─────────────┼───┼───┐
    ├── 5.4 一键应用 ─────────────┼───┼───┤
    └── 5.5 手动裁剪调整 ─────────┼───┼───┤
                                     │   │   │
Phase 6: 导出功能                   │   │   │
    ├── 6.1 图像导出基础 ───────────┘   │   │
    └── 6.2 批量导出 ───────────────────┘   │
                                            │
Phase 7: 撤销/重做 ─────────────────────────┘
            │
Phase 8: UI/UX 完善
Phase 9: 测试与优化
```

---

## 里程碑

| 里程碑 | 目标 | 时间 |
|--------|------|------|
| **M1** | 项目初始化完成，WASM 成功加载 | Week 1 |
| **M2** | 图像加载、显示、平移/缩放完成 | Week 2 |
| **M3** | 基础调整 (亮度/对比度/饱和度) 完成 | Week 3 |
| **M4** | AI 裁剪功能完成 | Week 4 |
| **M5** | 导出功能完成 | Week 4-5 |
| **M6** | MVP 功能完整，可发布 | Week 6 |

---

## 风险与备注

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Safari WebGPU 不支持 | 丢失桌面用户 | 显示降级提示，未来考虑 WebGL2 后备 |
| ONNX 模型文件大 | 首次加载慢 | 懒加载，显示进度 |
| WASM 文件过大 | 加载时间 | 优化编译选项，代码分割 |
| 大图片内存占用 | 崩溃 | 限制最大尺寸，流式处理 |
