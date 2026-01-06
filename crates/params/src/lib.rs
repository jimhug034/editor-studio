//! 图像调整参数定义

/// 图像调整参数
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AdjustmentParams {
    /// 亮度 (-1.0 到 1.0)
    pub brightness: f32,
    /// 对比度 (0.0 到 2.0)
    pub contrast: f32,
    /// 饱和度 (0.0 到 2.0)
    pub saturation: f32,
    /// 填充对齐
    pub _padding: f32,
}

impl AdjustmentParams {
    pub const fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            _padding: 0.0,
        }
    }

    /// 重置为默认值
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// 设置亮度
    pub fn set_brightness(&mut self, value: f32) {
        self.brightness = value.clamp(-1.0, 1.0);
    }

    /// 设置对比度
    pub fn set_contrast(&mut self, value: f32) {
        self.contrast = value.clamp(0.0, 2.0);
    }

    /// 设置饱和度
    pub fn set_saturation(&mut self, value: f32) {
        self.saturation = value.clamp(0.0, 2.0);
    }
}

/// 裁剪矩形
#[derive(Clone, Copy, Debug, Default)]
pub struct CropRect {
    /// 归一化 x 坐标 (0-1)
    pub x: f32,
    /// 归一化 y 坐标 (0-1)
    pub y: f32,
    /// 归一化宽度 (0-1)
    pub width: f32,
    /// 归一化高度 (0-1)
    pub height: f32,
    /// 宽高比 (None = 自由)
    pub ratio: Option<f32>,
}

impl CropRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            ratio: None,
        }
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// 检查裁剪区域是否有效
    pub fn is_valid(&self) -> bool {
        self.x >= 0.0
            && self.y >= 0.0
            && self.width > 0.0
            && self.height > 0.0
            && self.x + self.width <= 1.0
            && self.y + self.height <= 1.0
    }
}
