mod renderer;
mod params;
mod shaders;

use wasm_bindgen::prelude::*;

// 当 `wee_alloc` 特性启用时，使用 wee_alloc 作为全局分配器
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 在控制台显示 panic 信息
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// WASM 图像编辑器
#[wasm_bindgen]
pub struct WasmImageEditor {
    renderer: renderer::Renderer,
}

#[wasm_bindgen]
impl WasmImageEditor {
    /// 创建新的图像编辑器实例
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<WasmImageEditor, JsValue> {
        let renderer = renderer::Renderer::new().await?;
        Ok(WasmImageEditor { renderer })
    }

    /// 加载图像数据
    pub fn load_image(&mut self, _data: &[u8]) -> Result<(), JsValue> {
        // TODO: 实现图像加载
        Ok(())
    }

    /// 设置亮度调整 (-1.0 到 1.0)
    pub fn set_brightness(&mut self, value: f32) {
        self.renderer.set_brightness(value.clamp(-1.0, 1.0));
    }

    /// 设置对比度调整 (0.0 到 2.0)
    pub fn set_contrast(&mut self, value: f32) {
        self.renderer.set_contrast(value.clamp(0.0, 2.0));
    }

    /// 设置饱和度调整 (0.0 到 2.0)
    pub fn set_saturation(&mut self, value: f32) {
        self.renderer.set_saturation(value.clamp(0.0, 2.0));
    }

    /// 重置所有调整
    pub fn reset_adjustments(&mut self) {
        self.renderer.reset_adjustments();
    }

    /// 请求渲染一帧
    pub fn render(&mut self) -> Result<(), JsValue> {
        self.renderer.render();
        Ok(())
    }
}
