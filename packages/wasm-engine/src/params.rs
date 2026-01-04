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
}
