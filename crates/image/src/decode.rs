//! 图像解码模块

use super::Image;

/// 图像解码器
pub struct ImageDecoder;

impl ImageDecoder {
    /// 从字节数据解码图像
    pub fn decode(data: &[u8]) -> Result<Image, String> {
        // 使用 image crate 解码
        let dyn_image =
            image::load_from_memory(data).map_err(|e| format!("Failed to decode image: {}", e))?;

        // 转换为 RGBA8
        let rgba_image = dyn_image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let data = rgba_image.into_raw();

        // 检测原始格式
        let format = Self::detect_format(&data);

        Ok(Image {
            data,
            width,
            height,
            format,
        })
    }

    /// 检测图像格式
    fn detect_format(_data: &[u8]) -> ImageFormat {
        // TODO: 实际检测格式
        ImageFormat::Rgba8
    }
}

/// 图像格式
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    /// JPEG 格式
    Jpeg,
    /// PNG 格式
    Png,
    /// WebP 格式
    WebP,
    /// RGBA8 原始格式
    Rgba8,
}

impl ImageFormat {
    /// MIME 类型
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::WebP => "image/webp",
            ImageFormat::Rgba8 => "image/rgba",
        }
    }

    /// 扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::WebP => "webp",
            ImageFormat::Rgba8 => "rgba",
        }
    }
}
