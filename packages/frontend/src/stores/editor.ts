import { defineStore } from 'pinia'
import { ref, shallowRef } from 'vue'

export const useEditorStore = defineStore('editor', () => {
  // Image state
  const imageFile = ref<File | null>(null)
  const imageUrl = ref<string | null>(null)
  const imageWidth = ref(0)
  const imageHeight = ref(0)

  // Transform state
  const scale = ref(1)
  const offsetX = ref(0)
  const offsetY = ref(0)

  // Adjustment state
  const brightness = ref(0)
  const contrast = ref(0)
  const saturation = ref(0)

  // Crop state
  const cropX = ref(0)
  const cropY = ref(0)
  const cropWidth = ref(0)
  const cropHeight = ref(0)
  const cropRatio = ref<number | null>(null)

  // Loading state
  const isLoading = ref(false)
  const isWasmLoaded = ref(false)

  function loadImage(file: File) {
    imageFile.value = file
    imageUrl.value = URL.createObjectURL(file)
    isLoading.value = true
  }

  function resetImage() {
    imageFile.value = null
    imageUrl.value = null
    URL.revokeObjectURL(imageUrl.value || '')
    imageWidth.value = 0
    imageHeight.value = 0
    resetTransform()
    resetAdjustments()
  }

  function resetTransform() {
    scale.value = 1
    offsetX.value = 0
    offsetY.value = 0
  }

  function resetAdjustments() {
    brightness.value = 0
    contrast.value = 0
    saturation.value = 0
  }

  function resetCrop() {
    cropX.value = 0
    cropY.value = 0
    cropWidth.value = 0
    cropHeight.value = 0
    cropRatio.value = null
  }

  return {
    // Image state
    imageFile,
    imageUrl,
    imageWidth,
    imageHeight,
    // Transform state
    scale,
    offsetX,
    offsetY,
    // Adjustment state
    brightness,
    contrast,
    saturation,
    // Crop state
    cropX,
    cropY,
    cropWidth,
    cropHeight,
    cropRatio,
    // Loading state
    isLoading,
    isWasmLoaded,
    // Actions
    loadImage,
    resetImage,
    resetTransform,
    resetAdjustments,
    resetCrop
  }
})
