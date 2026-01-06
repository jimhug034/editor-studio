//! wgpu 渲染器

use crate::pipelines::RenderPipelines;
use crate::texture::TextureManager;
use editor_studio_params::AdjustmentParams;
use wasm_bindgen::prelude::*;

mod pipelines;
mod shaders;
mod texture;

/// wgpu 渲染器
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture_manager: TextureManager,
    pipelines: RenderPipelines,
    params: AdjustmentParams,
    render_requested: bool,
}

impl Renderer {
    /// 创建新的渲染器
    pub async fn new() -> Result<Self, JsValue> {
        // 初始化 wgpu 实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // 请求适配器
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                ..Default::default()
            })
            .await
            .ok_or_else(|| JsValue::from_str("No WebGPU adapter found"))?;

        // 请求设备和队列
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GPU Device"),
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits {
                        max_texture_dimension_2d: 8192,
                        ..Default::default()
                    },
                },
                None,
            )
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let texture_manager = TextureManager::new(&device);
        let pipelines = RenderPipelines::new(&device);

        Ok(Self {
            device,
            queue,
            texture_manager,
            pipelines,
            params: AdjustmentParams::default(),
            render_requested: false,
        })
    }

    /// 获取设备引用
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// 获取队列引用
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// 获取纹理管理器
    pub fn texture_manager(&self) -> &TextureManager {
        &self.texture_manager
    }

    /// 获取纹理管理器（可变）
    pub fn texture_manager_mut(&mut self) -> &mut TextureManager {
        &mut self.texture_manager
    }

    /// 设置亮度
    pub fn set_brightness(&mut self, value: f32) {
        self.params.set_brightness(value);
        self.render_requested = true;
    }

    /// 设置对比度
    pub fn set_contrast(&mut self, value: f32) {
        self.params.set_contrast(value);
        self.render_requested = true;
    }

    /// 设置饱和度
    pub fn set_saturation(&mut self, value: f32) {
        self.params.set_saturation(value);
        self.render_requested = true;
    }

    /// 重置所有调整
    pub fn reset_adjustments(&mut self) {
        self.params.reset();
        self.render_requested = true;
    }

    /// 获取当前调整参数
    pub fn params(&self) -> &AdjustmentParams {
        &self.params
    }

    /// 请求渲染
    pub fn request_render(&mut self) {
        self.render_requested = true;
    }

    /// 渲染一帧
    pub fn render(&mut self) {
        if !self.render_requested {
            return;
        }

        // TODO: 实现实际渲染逻辑

        self.render_requested = false;
    }
}
