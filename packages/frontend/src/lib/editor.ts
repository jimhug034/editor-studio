/**
 * WASM Image Editor binding wrapper
 *
 * This module provides a TypeScript interface to the Rust WASM image editor.
 * The WASM module will be built from packages/wasm-engine.
 */

let wasmModule: typeof import('@editor-studio/wasm-engine') | null = null

export interface AIBoundingBox {
  x: number
  y: number
  width: number
  height: number
}

export interface CropSuggestion {
  ratio: number
  x: number
  y: number
  width: number
  height: number
  score: number
  label: string
}

export interface ExportConfig {
  width: number
  height: number
  format: 'jpeg' | 'png' | 'webp'
  quality?: number
}

export interface EncodedFile {
  name: string
  data: Uint8Array
  size: number
}

/**
 * Initialize the WASM module
 */
export async function initWasm(): Promise<void> {
  if (wasmModule) return

  try {
    wasmModule = await import('@editor-studio/wasm-engine')
    await wasmModule.default()
  } catch (error) {
    console.error('Failed to load WASM module:', error)
    throw new Error('Failed to initialize WASM module. Make sure the module is built.')
  }
}

/**
 * Create a new image editor instance
 */
export async function createEditor(canvas: HTMLCanvasElement): Promise<any> {
  await initWasm()

  if (!wasmModule) {
    throw new Error('WASM module not initialized')
  }

  // TODO: Create WasmImageEditor instance
  // return new wasmModule.WasmImageEditor(canvas)

  return null
}

/**
 * Check if WebGPU is supported
 */
export function isWebGPUSupported(): boolean {
  return 'gpu' in navigator
}

/**
 * Get WebGPU support message
 */
export function getWebGPUSupportMessage(): { supported: boolean; message: string } {
  if (isWebGPUSupported()) {
    return { supported: true, message: 'WebGPU is supported' }
  }

  return {
    supported: false,
    message: 'WebGPU is not supported in this browser. Please use Chrome 113+ or Edge 113+ for the best experience.'
  }
}
