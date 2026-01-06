//! 数学工具函数

use editor_studio_params::CropRect;

/// 计算包含边界框的裁剪区域，保持指定宽高比
pub fn calculate_crop_containing(
    bbox: &CropRect,
    target_ratio: f32,
) -> CropRect {
    let bbox_ratio = bbox.width / bbox.height;

    let (width, height) = if bbox_ratio > target_ratio {
        // 目标更窄，以高度为准
        let height = bbox.height;
        let width = height * target_ratio;
        (width, height)
    } else {
        // 目标更宽，以宽度为准
        let width = bbox.width;
        let height = width / target_ratio;
        (width, height)
    };

    // 居中裁剪
    let x = bbox.x + (bbox.width - width) / 2.0;
    let y = bbox.y + (bbox.height - height) / 2.0;

    CropRect {
        x: x.max(0.0),
        y: y.max(0.0),
        width,
        height,
        ratio: Some(target_ratio),
    }
}

/// 计算带填充的裁剪区域
pub fn calculate_crop_with_padding(
    bbox: &CropRect,
    padding: f32, // 0.0 - 1.0
) -> CropRect {
    let padding_x = bbox.width * padding;
    let padding_y = bbox.height * padding;

    CropRect {
        x: (bbox.x - padding_x).max(0.0),
        y: (bbox.y - padding_y).max(0.0),
        width: (bbox.width + padding_x * 2.0).min(1.0 - bbox.x),
        height: (bbox.height + padding_y * 2.0).min(1.0 - bbox.y),
        ratio: None,
    }
}

/// 应用三分法则评分
pub fn rule_of_thirds_score(crop: &CropRect, subject: &CropRect) -> f32 {
    // 计算主体中心
    let subject_cx = subject.x + subject.width / 2.0;
    let subject_cy = subject.y + subject.height / 2.0;

    // 计算裁剪区域的三分点
    let third_x = crop.width / 3.0;
    let third_y = crop.height / 3.0;

    // 四个三分点
    let points = [
        (crop.x + third_x, crop.y + third_y),
        (crop.x + third_x * 2.0, crop.y + third_y),
        (crop.x + third_x, crop.y + third_y * 2.0),
        (crop.x + third_x * 2.0, crop.y + third_y * 2.0),
    ];

    // 找到最近的点距离
    let min_dist = points
        .iter()
        .map(|&(px, py)| {
            let dx = subject_cx - px;
            let dy = subject_cy - py;
            (dx * dx + dy * dy).sqrt()
        })
        .reduce(f32::min)
        .unwrap();

    // 转换为评分 (距离越小分数越高)
    (1.0 - min_dist).max(0.0)
}

/// 图像尺寸相关计算
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl ImageSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// 计算适应容器的尺寸
    pub fn fit_in(&self, container: &ImageSize) -> ImageSize {
        let scale = (container.width as f32 / self.width as f32)
            .min(container.height as f32 / self.height as f32);

        ImageSize {
            width: (self.width as f32 * scale) as u32,
            height: (self.height as f32 * scale) as u32,
        }
    }

    /// 计算填充容器的尺寸
    pub fn cover(&self, container: &ImageSize) -> ImageSize {
        let scale = (container.width as f32 / self.width as f32)
            .max(container.height as f32 / self.height as f32);

        ImageSize {
            width: (self.width as f32 * scale) as u32,
            height: (self.height as f32 * scale) as u32,
        }
    }
}
