<template>
  <div class="editor-canvas">
    <!-- Drop zone overlay -->
    <div v-if="!store.imageUrl" class="drop-zone" @drop="handleDrop" @dragover.prevent @dragenter.prevent>
      <div class="drop-zone-content">
        <svg class="drop-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <h2>Drop your photo here</h2>
        <p>or click to browse</p>
        <input
          ref="fileInput"
          type="file"
          accept="image/jpeg,image/png,image/webp"
          @change="handleFileSelect"
          class="visually-hidden"
        >
        <button class="btn-primary" @click="triggerFileInput">Choose Photo</button>
      </div>
    </div>

    <!-- WebGPU warning -->
    <div v-if="!isWebGPUSupported && !store.imageUrl" class="warning-banner">
      {{ webgpuMessage }}
    </div>

    <!-- Canvas container (for when image is loaded) -->
    <div v-if="store.imageUrl" class="canvas-container">
      <p class="placeholder">Image loaded: {{ store.imageWidth }} x {{ store.imageHeight }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useEditorStore } from '@/stores/editor'
import { getWebGPUSupportMessage } from '@/lib/editor'

const store = useEditorStore()
const fileInput = ref<HTMLInputElement | null>(null)

const webgpuSupport = getWebGPUSupportMessage()
const isWebGPUSupported = ref(webgpuSupport.supported)
const webgpuMessage = ref(webgpuSupport.message)

onMounted(() => {
  store.isWasmLoaded = false
})

function triggerFileInput() {
  fileInput.value?.click()
}

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    loadFile(file)
  }
}

function handleDrop(event: DragEvent) {
  event.preventDefault()
  const file = event.dataTransfer?.files[0]
  if (file && file.type.startsWith('image/')) {
    loadFile(file)
  }
}

function loadFile(file: File) {
  store.loadImage(file)

  // Get image dimensions
  const img = new Image()
  img.onload = () => {
    store.imageWidth = img.naturalWidth
    store.imageHeight = img.naturalHeight
    store.isLoading = false
  }
  img.src = URL.createObjectURL(file)
}
</script>

<style scoped>
.editor-canvas {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

.drop-zone {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 2px dashed #3a3a3a;
  border-radius: 8px;
  margin: 1rem;
  background-color: #1a1a1a;
  transition: border-color 0.2s ease, background-color 0.2s ease;
}

.drop-zone:hover {
  border-color: #5a5a5a;
  background-color: #222;
}

.drop-zone-content {
  text-align: center;
  color: #a0a0a0;
}

.drop-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 1rem;
  color: #606060;
}

.drop-zone-content h2 {
  margin: 0 0 0.5rem;
  font-size: 1.25rem;
  color: #ffffff;
}

.drop-zone-content p {
  margin: 0 0 1.5rem;
  font-size: 0.875rem;
}

.btn-primary {
  padding: 0.75rem 1.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: #ffffff;
  background-color: #3b82f6;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.btn-primary:hover {
  background-color: #2563eb;
}

.warning-banner {
  padding: 0.75rem 1rem;
  margin: 0 1rem;
  background-color: #3c1a1a;
  border: 1px solid #5c2a2a;
  border-radius: 6px;
  color: #fca5a5;
  font-size: 0.875rem;
}

.canvas-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder {
  color: #606060;
}
</style>
