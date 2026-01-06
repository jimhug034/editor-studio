//! 纹理管理模块

use editor_studio_params::AdjustmentParams;
use wasm_bindgen::prelude::*;

/// 纹理管理器
pub struct TextureManager {
    source_texture: Option<wgpu::Texture>,
    bind_group: Option<wgpu::BindGroup>,
    uniform_buffer: wgpu::Buffer,
}

impl TextureManager {
    /// 创建新的纹理管理器
    pub fn new(device: &wgpu::Device) -> Self {
        // 创建 uniform buffer 用于调整参数
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<AdjustmentParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            source_texture: None,
            bind_group: None,
            uniform_buffer,
        }
    }

    /// 加载图像数据到纹理
    pub fn load_image(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), JsValue> {
        // 创建纹理
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Source Image Texture"),
            size: wgpu::Extent3d {
                width,
                height,
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
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.source_texture = Some(texture);
        Ok(())
    }

    /// 获取源纹理
    pub fn source_texture(&self) -> Option<&wgpu::Texture> {
        self.source_texture.as_ref()
    }

    /// 获取 uniform buffer
    pub fn uniform_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }

    /// 更新 uniform buffer 数据
    pub fn update_uniform_buffer(&self, queue: &wgpu::Queue, params: &AdjustmentParams) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(params));
    }
}
