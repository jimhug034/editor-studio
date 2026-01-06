//! 图像处理模块

pub mod decode;
pub mod transform;

pub use decode::{ImageDecoder, ImageFormat};
pub use transform::{ImageTransform, ResizeOptions};

use editor_studio_math::ImageSize;

/// 图像数据
#[derive(Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

impl Image {
    /// 从字节数据加载图像
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        ImageDecoder::decode(data)
    }

    /// 创建新图像
    pub fn new(width: u32, height: u32, format: ImageFormat) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            data: vec![0; size],
            width,
            height,
            format,
        }
    }

    /// 获取尺寸
    pub fn size(&self) -> ImageSize {
        ImageSize::new(self.width, self.height)
    }

    /// 获取像素数
    pub fn pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// 获取数据大小
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
}

/// 图像导出配置
pub struct ExportConfig {
    pub format: ImageFormat,
    pub quality: u8, // 0-100
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ImageFormat::Jpeg,
            quality: 90,
        }
    }
}

impl ExportConfig {
    pub fn jpeg(quality: u8) -> Self {
        Self {
            format: ImageFormat::Jpeg,
            quality: quality.clamp(0, 100),
        }
    }

    pub fn png() -> Self {
        Self {
            format: ImageFormat::Png,
            quality: 100,
        }
    }

    pub fn webp(quality: u8) -> Self {
        Self {
            format: ImageFormat::WebP,
            quality: quality.clamp(0, 100),
        }
    }
}
