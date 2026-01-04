use crate::params::AdjustmentParams;
use wasm_bindgen::prelude::*;

/// wgpu 渲染器
pub struct Renderer {
    // TODO: 添加 wgpu 设备、队列、纹理等字段
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    params: AdjustmentParams,
}

impl Renderer {
    /// 创建新的渲染器
    pub async fn new() -> Result<Self, JsValue> {
        // TODO: 初始化 wgpu 设备

        Ok(Renderer {
            device: None,
            queue: None,
            params: AdjustmentParams::default(),
        })
    }

    /// 设置亮度
    pub fn set_brightness(&mut self, value: f32) {
        self.params.brightness = value;
    }

    /// 设置对比度
    pub fn set_contrast(&mut self, value: f32) {
        self.params.contrast = value;
    }

    /// 设置饱和度
    pub fn set_saturation(&mut self, value: f32) {
        self.params.saturation = value;
    }

    /// 重置所有调整
    pub fn reset_adjustments(&mut self) {
        self.params = AdjustmentParams::default();
    }

    /// 渲染一帧
    pub fn render(&mut self) {
        // TODO: 实现渲染逻辑
    }
}
