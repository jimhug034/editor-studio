# 技术架构

## 总体架构

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              用户界面层 (Vue3)                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │   上传组件   │  │   编辑器    │  │  调整面板   │  │   导出      │       │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘       │
└─────────────────────────────────────────────────────────────────────────────┘
                                        ↕
                    ┌───────────────────┴───────────────────┐
                    │       Canvas 分层渲染                 │
                    │  ┌─────────────────────────────────┐  │
                    │  │ Layer 1: wgpu Canvas (底层)     │  │
                    │  │ • 图像渲染                      │  │
                    │  │ • 滤镜效果                      │  │
                    │  │ • 实时预览                      │  │
                    │  └─────────────────────────────────┘  │
                    │  ┌─────────────────────────────────┐  │
                    │  │ Layer 2: Canvas 2D (上层)      │  │
                    │  │ • 裁剪框                        │  │
                    │  │ • 网格线                        │  │
                    │  │ • 手柄/控制点                   │  │
                    │  └─────────────────────────────────┘  │
                    └───────────────────────────────────────┘
                                        ↕
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Rust WASM 层                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        图像处理引擎                                  │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐   │   │
│  │  │  图像加载   │ │  裁剪引擎   │ │  滤镜系统   │ │  导出引擎   │   │   │
│  │  │  image crate│ │  GPU计算    │ │  WGSL着色器 │ │  GPU编码    │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    ↕                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       wgpu 渲染引擎                                  │   │
│  │  ┌───────────────────────────────────────────────────────────────┐ │   │
│  │  │  GPU Pipeline                                                  │ │   │
│  │  │  • Render Pipeline  → 图像渲染与滤镜                           │ │   │
│  │  │  • Compute Pipeline → 批量处理、直方图计算                     │ │   │
│  │  │  • Texture Management → 多级纹理、纹理视图                    │ │   │
│  │  └───────────────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                        ↕
┌─────────────────────────────────────────────────────────────────────────────┐
│                            AI 推理层                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    ONNX Runtime WebGPU                               │   │
│  │  • YOLOv8-nano 目标检测                                              │   │
│  │  • GPU 加速推理 (WebGPU Execution Provider)                          │   │
│  │  • 结果传递给 Rust 进行裁剪计算                                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 技术栈

### 前端 (TypeScript + Vue3)

```json
{
  "框架": "Vue 3.4+",
  "构建工具": "Vite 5+",
  "语言": "TypeScript 5.3+",
  "UI 库": "自定义 (极简设计)",
  "状态管理": "Pinia",
  "样式方案": "CSS Modules + Tailwind CSS"
}
```

### 图形渲染 (Rust + wgpu)

```toml
[dependencies]
wgpu = "0.20"           # WebGPU 绑定
wgpu-core = "0.20"      # 核心实现
image = { version = "0.25", features = ["jpeg", "png", "webp"] }
bytemuck = "1.14"       # 数据转换
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"

# AI 相关 (可选，如果 Rust 侧推理)
candle-core = "0.3"     # 或
# burn = "0.13"         # 替代方案
```

### AI 推理

```json
{
  "运行时": "onnxruntime-web",
  "模型": "YOLOv8-nano (6MB)",
  "加速": "WebGPU Execution Provider",
  "输入": "640x640 RGB 张量",
  "输出": "边界框 + 置信度"
}
```

---

## wgpu 能力最大化

### 1. GPU 加速的图像处理管线

所有图像操作都在 GPU 上完成，避免 CPU-GPU 数据传输：

```rust
// 所有操作使用 GPU 纹理，不回传 CPU
pub struct GPUImagePipeline {
    // 原始图像纹理（始终保留在 GPU）
    source_texture: wgpu::Texture,

    // 工作纹理（链式处理）
    work_texture_a: wgpu::Texture,
    work_texture_b: wgpu::Texture,

    // 各功能的渲染管线
    pipelines: ShaderPipelines,
}

pub struct ShaderPipelines {
    // 基础调整
    brightness_pipeline: wgpu::RenderPipeline,
    contrast_pipeline: wgpu::RenderPipeline,
    saturation_pipeline: wgpu::RenderPipeline,

    // 高级滤镜
    curves_pipeline: wgpu::RenderPipeline,        // 曲线调整
    hsl_pipeline: wgpu::RenderPipeline,           // HSL 调整
    color_balance_pipeline: wgpu::RenderPipeline, // 色彩平衡
    vibrance_pipeline: wgpu::RenderPipeline,      // 自然饱和度

    // 特效
    blur_pipeline: wgpu::RenderPipeline,
    sharpen_pipeline: wgpu::RenderPipeline,
    vignette_pipeline: wgpu::RenderPipeline,

    // 计算着色器
    histogram_compute: wgpu::ComputePipeline,     // 直方图
    auto_enhance_compute: wgpu::ComputePipeline,  // 自动增强
}
```

### 2. 计算着色器 (Compute Shader) 加速

使用 Compute Pipeline 处理需要全局信息的操作：

```wgsl
// 计算着色器示例：直方图计算
@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var<storage, read_write> histogram: array<atomic<u32>>;

struct WorkgroupData {
    data: array<u32, 256>,
}

var<workgroup> workgroup_hist: WorkgroupData;

@compute @workgroup_size(16, 16)
fn histogram_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(input_texture);
    let coord = vec2<i32>(id.xy);

    if (coord.x >= i32(dims.x) || coord.y >= i32(dims.y)) {
        return;
    }

    // 采样像素
    let color = textureLoad(input_texture, coord, 0);
    let luminance = u32(dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114)) * 255.0);

    // 工作组内原子累加
    atomicAdd(&workgroup_hist.data[luminance], 1u);

    // 工作组屏障
    workgroupBarrier();

    // 将工作组结果写入全局直方图
    if (id.x == 0u && id.y == 0u) {
        for (var i: u32 = 0u; i < 256u; i++) {
            if (workgroup_hist.data[i] > 0u) {
                atomicAdd(&histogram[i], workgroup_hist.data[i]);
            }
        }
    }
}
```

### 3. 纹理视图与零拷贝裁剪

裁剪操作使用纹理视图，无需复制数据：

```rust
pub struct ZeroCopyCrop {
    base_texture: wgpu::Texture,
    // 裁剪只改变视口，不复制纹理数据
    viewport: wgpu::Viewport,
}

impl ZeroCopyCrop {
    // 零拷贝裁剪：只改变渲染区域
    pub fn set_crop(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.viewport = wgpu::Viewport {
            x,
            y,
            width,
            height,
            min_depth: 0.0,
            max_depth: 1.0,
        };
    }
}
```

### 4. 多级纹理 (Mipmaps) 平滑缩放

预生成多级纹理，缩放时自动插值：

```rust
pub fn create_texture_with_mipmaps(device: &wgpu::Device, queue: &wgpu::Queue, data: &[u8], width: u32, height: u32) -> wgpu::Texture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Image Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: calculate_mip_levels(width, height), // 自动计算
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    // 上传并生成所有 mipmap 级别
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );

    // 自动生成剩余 mipmap
    generate_mipmaps(device, queue, &texture);

    texture
}
```

### 5. 批量处理并行化

导出多个尺寸时并行处理：

```rust
pub async fn export_batch(
    &self,
    sizes: Vec<ExportSize>,
) -> Vec<EncodedImage> {
    // 使用存储缓冲区存储多个输出
    let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Batch Output"),
        size: total_output_size(&sizes),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // 计算着色器并行生成所有尺寸
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Batch Export"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Batch Resize Pass"),
        });
        pass.set_pipeline(&self.batch_resize_pipeline);
        pass.set_bind_group(0, &self.batch_bind_group, &[]);
        // 一次 dispatch 处理所有尺寸
        pass.dispatch_workgroups(sizes.len() as u32, 1, 1);
    }

    self.queue.submit(Some(encoder.finish()));

    // 读取结果
    let results = self.read_buffer_async(&output_buffer).await;
    self.encode_results(results, &sizes)
}
```

### 6. 实时预览优化

滑块拖动时使用降采样预览：

```rust
pub struct RealtimePreview {
    // 全分辨率纹理
    full_texture: wgpu::Texture,
    // 降采样预览纹理 (1/4 分辨率)
    preview_texture: wgpu::Texture,
    // 当前模式
    mode: PreviewMode,
}

pub enum PreviewMode {
    Editing {    // 编辑中：使用预览纹理，60fps
        is_interactive: bool,
    },
    Exporting,  // 导出：使用全分辨率
}
```

---

## Rust 侧核心数据结构

```rust
// wasm-bindgen 入口
#[wasm_bindgen]
pub struct WasmImageEditor {
    renderer: Renderer,
    state: EditorState,
}

#[wasm_bindgen]
impl WasmImageEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        config: JsValue,
    ) -> Result<WasmImageEditor, JsValue> {
        // 初始化 wgpu 渲染器
        let renderer = Renderer::new(canvas, config)?;
        Ok(Self {
            renderer,
            state: EditorState::new(),
        })
    }

    /// 加载图像（支持 JPEG/PNG/WEBP）
    pub fn load_image(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.renderer.load_image(data)
    }

    /// AI 检测（接收来自 ONNX 的边界框）
    pub fn apply_ai_detection(&mut self, bbox: AIBoundingBox) -> Result<(), JsValue> {
        self.renderer.calculate_crop_suggestion(bbox)
    }

    /// 设置裁剪区域
    pub fn set_crop(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.state.set_crop(x, y, width, height);
        self.renderer.request_render();
    }

    /// 调整参数（实时更新）
    pub fn set_brightness(&mut self, value: f32) {
        self.state.adjustments.brightness = value;
        self.renderer.update_uniform_buffer(&self.state.adjustments);
        self.renderer.request_render();
    }

    pub fn set_contrast(&mut self, value: f32) {
        self.state.adjustments.contrast = value;
        self.renderer.update_uniform_buffer(&self.state.adjustments);
        self.renderer.request_render();
    }

    pub fn set_saturation(&mut self, value: f32) {
        self.state.adjustments.saturation = value;
        self.renderer.update_uniform_buffer(&self.state.adjustments);
        self.renderer.request_render();
    }

    /// 自动增强（基于直方图）
    pub async fn auto_enhance(&mut self) -> Result<(), JsValue> {
        let histogram = self.renderer.compute_histogram().await?;
        let optimal = compute_optimal_adjustments(&histogram);
        self.state.adjustments = optimal;
        self.renderer.update_uniform_buffer(&self.state.adjustments);
        Ok(())
    }

    /// 导出图像
    pub fn export(&self, format: ExportFormat, quality: u8) -> Result<Vec<u8>, JsValue> {
        self.renderer.export(format, quality)
    }

    /// 批量导出多个尺寸
    pub async fn export_batch(&self, configs: JsValue) -> Result<JsValue, JsValue> {
        let configs: Vec<ExportConfig> = configs.into_serde().unwrap();
        let results = self.renderer.export_batch(configs).await?;
        Ok(JsValue::from_serde(&results).unwrap())
    }
}

// 编辑器状态
#[derive(Clone, Copy)]
pub struct EditorState {
    pub original_size: (u32, u32),
    pub current_crop: Option<CropRect>,
    pub adjustments: AdjustmentParams,
    pub history: HistoryStack,
}

#[derive(Clone, Copy)]
pub struct AdjustmentParams {
    pub brightness: f32,   // -1.0 到 1.0
    pub contrast: f32,     // 0.0 到 2.0
    pub saturation: f32,   // 0.0 到 2.0
    pub exposure: f32,     // 0.0 到 2.0
    pub warmth: f32,       // -1.0 到 1.0
    pub tint: f32,         // -1.0 到 1.0
}

#[derive(Clone, Copy)]
pub struct CropRect {
    pub x: f32,      // 归一化坐标 0-1
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub ratio: Option<f32>,
}

// 渲染器
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,

    // 纹理
    source_texture: Option<wgpu::Texture>,
    display_texture: wgpu::Texture,

    // 管线
    pipelines: ShaderPipelines,
    uniform_buffer: wgpu::Buffer,

    // 渲染状态
    frame_count: u32,
    render_requested: bool,
}

impl Renderer {
    pub fn new(canvas: web_sys::HtmlCanvasElement, config: JsValue) -> Result<Self, JsValue> {
        // 创建 wgpu 实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // 创建 surface
        let surface = unsafe { instance.create_surface(&canvas) }
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // 请求适配器
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        })).ok_or_else(|| JsValue::from_str("No adapter found"))?;

        // 请求设备
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("GPU Device"),
                required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                    | wgpu::Features::TIMESTAMP_QUERY
                    | wgpu::Features::PIPELINE_STATISTICS_QUERY,
                required_limits: wgpu::Limits {
                    max_texture_dimension_2d: 8192,
                    ..Default::default()
                },
            },
            None,
        )).map_err(|e| JsValue::from_str(&e.to_string()))?;

        // 配置 surface
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: canvas.width(),
            height: canvas.height(),
            present_mode: wgpu::PresentMode::Fifo, // 垂直同步
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // 创建管线
        let pipelines = Self::create_pipelines(&device, &surface_config);

        // 创建 uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<AdjustmentParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            source_texture: None,
            display_texture: Self::create_display_texture(&device, &surface_config),
            pipelines,
            uniform_buffer,
            frame_count: 0,
            render_requested: false,
        })
    }

    fn create_pipelines(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> ShaderPipelines {
        // 加载着色器
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Image Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/image.wgsl").into()),
        });

        // 创建绑定组布局
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            ..Default::default()
        });

        // 创建渲染管线
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // TODO: 创建更多管线...

        ShaderPipelines {
            main: render_pipeline,
            // ...
        }
    }

    pub fn load_image(&mut self, data: &[u8]) -> Result<(), JsValue> {
        // 使用 image crate 解码
        let image = image::load_from_memory(data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // 转换为 RGBA
        let rgba = image.to_rgba8();

        // 创建纹理
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Source Image"),
            size: wgpu::Extent3d {
                width: rgba.width(),
                height: rgba.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 上传数据
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba.as_raw().as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(rgba.width() * 4),
                rows_per_image: Some(rgba.height()),
            },
            wgpu::Extent3d {
                width: rgba.width(),
                height: rgba.height(),
                depth_or_array_layers: 1,
            },
        );

        self.source_texture = Some(texture);
        self.render_requested = true;

        Ok(())
    }

    pub fn render(&mut self) {
        if !self.render_requested {
            return;
        }

        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // TODO: 绘制图像
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        self.render_requested = false;
        self.frame_count += 1;
    }
}
```

---

## JS 侧集成

```typescript
// packages/frontend/src/lib/editor.ts
import init, { WasmImageEditor } from '@editor-studio/wasm-engine';

let wasmLoaded = false;

export async function initWasm() {
  if (wasmLoaded) return;
  await init();
  wasmLoaded = true;
}

export class ImageEditor {
  private editor: WasmImageEditor;

  constructor(canvas: HTMLCanvasElement) {
    if (!wasmLoaded) {
      throw new Error('WASM not initialized. Call initWasm() first.');
    }
    this.editor = new WasmImageEditor(canvas);
  }

  async loadImage(file: File): Promise<void> {
    const buffer = await file.arrayBuffer();
    const data = new Uint8Array(buffer);
    this.editor.load_image(data);
  }

  setBrightness(value: number): void {
    this.editor.set_brightness(clamp(value, -1, 1));
  }

  setContrast(value: number): void {
    this.editor.set_contrast(clamp(value, 0, 2));
  }

  setSaturation(value: number): void {
    this.editor.set_saturation(clamp(value, 0, 2));
  }

  async export(format: 'jpeg' | 'png' | 'webp', quality: number): Promise<Blob> {
    const data = this.editor.export(format, quality);
    return new Blob([data], { type: `image/${format}` });
  }

  async exportBatch(configs: ExportConfig[]): Promise<Blob[]> {
    const results = await this.editor.export_batch(JSON.stringify(configs));
    return results.map((r: any) => new Blob([r.data], { type: r.mime }));
  }
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

export interface ExportConfig {
  width: number;
  height: number;
  format: 'jpeg' | 'png' | 'webp';
  quality?: number;
}
```

---

## 性能目标

| 操作 | 目标 | 实现方式 |
|------|------|----------|
| 加载 12MP 照片 | <500ms | GPU 纹理上传，并行解码 |
| AI 分析 | <2s | WebGPU 加速 ONNX 推理 |
| 滑块更新 | <16ms (60fps) | GPU 着色器，零拷贝 |
| 平移/缩放 | 60fps | Mipmap 纹理采样 |
| 导出单张 | <1s | GPU 编码，批量处理 |
| 批量导出 5 尺寸 | <3s | Compute Pipeline 并行 |

---

## 浏览器兼容性

| 浏览器 | 最低版本 | WebGPU 状态 | 备注 |
|--------|---------|------------|------|
| Chrome | 113+ | ✅ 默认开启 | 主要支持 |
| Edge | 113+ | ✅ 默认开启 | 完全兼容 |
| Firefox | Nightly | ⚠️ 需手动开启 | `dom.webgpu.enabled` |
| Safari | TP | ❌ 未稳定 | 显示降级提示 |

**Safari 降级策略**：
```
检测到不支持 WebGPU → 显示友好提示
"建议使用 Chrome/Edge 获得最佳体验"

未来考虑：WebGL2 降级方案 (v2.0)
```
