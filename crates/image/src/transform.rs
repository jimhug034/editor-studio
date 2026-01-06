//! 图像变换模块

use super::Image;
use editor_studio_params::CropRect;

/// 图像变换操作
pub struct ImageTransform;

impl ImageTransform {
    /// 裁剪图像
    pub fn crop(image: &Image, crop_rect: &CropRect) -> Image {
        let x = (crop_rect.x * image.width as f32) as u32;
        let y = (crop_rect.y * image.height as f32) as u32;
        let width = (crop_rect.width * image.width as f32) as u32;
        let height = (crop_rect.height * image.height as f32) as u32;

        let mut result = Image::new(width, height, image.format);

        for row in 0..height {
            for col in 0..width {
                let src_idx = ((y + row) * image.width + (x + col)) as usize * 4;
                let dst_idx = (row * width + col) as usize * 4;

                result.data[dst_idx] = image.data[src_idx];
                result.data[dst_idx + 1] = image.data[src_idx + 1];
                result.data[dst_idx + 2] = image.data[src_idx + 2];
                result.data[dst_idx + 3] = image.data[src_idx + 3];
            }
        }

        result
    }

    /// 调整图像大小
    pub fn resize(image: &Image, options: ResizeOptions) -> Image {
        let (new_width, new_height) = match options {
            ResizeOptions::ExactSize { width, height } => (width, height),
            ResizeOptions::FitIn { max_width, max_height } => {
                let scale = (max_width as f32 / image.width as f32)
                    .min(max_height as f32 / image.height as f32);
                (
                    (image.width as f32 * scale) as u32,
                    (image.height as f32 * scale) as u32,
                )
            }
            ResizeOptions::Cover { width, height } => {
                let scale = (width as f32 / image.width as f32)
                    .max(height as f32 / image.height as f32);
                (
                    (image.width as f32 * scale) as u32,
                    (image.height as f32 * scale) as u32,
                )
            }
        };

        let mut result = Image::new(new_width, new_height, image.format);

        // 简单的双线性插值
        let x_ratio = image.width as f32 / new_width as f32;
        let y_ratio = image.height as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let px = (x as f32 * x_ratio) as u32;
                let py = (y as f32 * y_ratio) as u32;
                let src_idx = (py * image.width + px) as usize * 4;
                let dst_idx = (y * new_width + x) as usize * 4;

                result.data[dst_idx] = image.data[src_idx];
                result.data[dst_idx + 1] = image.data[src_idx + 1];
                result.data[dst_idx + 2] = image.data[src_idx + 2];
                result.data[dst_idx + 3] = image.data[src_idx + 3];
            }
        }

        result
    }

    /// 旋转图像
    pub fn rotate(image: &Image, degrees: f32) -> Image {
        // TODO: 实现旋转
        image.clone()
    }

    /// 翻转图像
    pub fn flip(image: &Image, horizontal: bool, vertical: bool) -> Image {
        let mut result = Image::new(image.width, image.height, image.format);

        for y in 0..image.height {
            for x in 0..image.width {
                let src_x = if horizontal { image.width - 1 - x } else { x };
                let src_y = if vertical { image.height - 1 - y } else { y };

                let src_idx = (src_y * image.width + src_x) as usize * 4;
                let dst_idx = (y * image.width + x) as usize * 4;

                result.data[dst_idx] = image.data[src_idx];
                result.data[dst_idx + 1] = image.data[src_idx + 1];
                result.data[dst_idx + 2] = image.data[src_idx + 2];
                result.data[dst_idx + 3] = image.data[src_idx + 3];
            }
        }

        result
    }
}

/// 调整大小选项
pub enum ResizeOptions {
    /// 精确尺寸
    ExactSize { width: u32, height: u32 },
    /// 适应容器
    FitIn { max_width: u32, max_height: u32 },
    /// 填充容器
    Cover { width: u32, height: u32 },
}
