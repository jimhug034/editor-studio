//! WebAssembly 绑定 - Editor Studio
//!
//! 这是 WASM 模块的主入口点，通过 wasm-bindgen 导出 API 给 JavaScript。

use wasm_bindgen::prelude::*;

// 在控制台显示 panic 信息
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// WASM 图像编辑器
#[wasm_bindgen]
pub struct WasmImageEditor {
    renderer: Option<editor_studio_renderer::Renderer>,
}

#[wasm_bindgen]
impl WasmImageEditor {
    /// 创建新的图像编辑器实例
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<WasmImageEditor, JsValue> {
        let renderer = editor_studio_renderer::Renderer::new().await?;
        Ok(WasmImageEditor {
            renderer: Some(renderer),
        })
    }

    /// 加载图像数据
    ///
    /// # 参数
    /// * `data` - 图像数据的字节数组 (JPEG/PNG/WEBP)
    pub fn load_image(&mut self, data: &[u8]) -> Result<(), JsValue> {
        use editor_studio_image::Image;

        // 解码图像
        let image = Image::from_bytes(data)
            .map_err(|e| JsValue::from_str(&e))?;

        // 上传到 GPU 纹理
        if let Some(ref mut renderer) = self.renderer {
            let device = renderer.device();
            let queue = renderer.queue();
            let texture_manager = renderer.texture_manager_mut();

            texture_manager.load_image(
                device,
                queue,
                &image.data,
                image.width,
                image.height,
            )?;
        }

        Ok(())
    }

    /// 设置亮度调整 (-1.0 到 1.0)
    #[wasm_bindgen]
    pub fn set_brightness(&mut self, value: f32) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_brightness(value);
        }
    }

    /// 设置对比度调整 (0.0 到 2.0)
    #[wasm_bindgen]
    pub fn set_contrast(&mut self, value: f32) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_contrast(value);
        }
    }

    /// 设置饱和度调整 (0.0 到 2.0)
    #[wasm_bindgen]
    pub fn set_saturation(&mut self, value: f32) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_saturation(value);
        }
    }

    /// 重置所有调整
    #[wasm_bindgen]
    pub fn reset_adjustments(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.reset_adjustments();
        }
    }

    /// 请求渲染一帧
    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        if let Some(ref mut renderer) = self.renderer {
            renderer.render();
        }
        Ok(())
    }

    /// 导出图像为 JPEG 格式
    ///
    /// # 参数
    /// * `quality` - JPEG 质量 (0-100)
    #[wasm_bindgen]
    pub fn export_jpeg(&self, quality: u8) -> Result<JsValue, JsValue> {
        // TODO: 实现导出逻辑
        Ok(JsValue::NULL)
    }
}
