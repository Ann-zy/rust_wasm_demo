<template>
  <input type="file" @change="handleFileChange" accept="image/*" />
  <img v-show="compressed" :src="compressed" />
  <p>
    wasm： Original Size: {{ originalSize }} bytes<br />
    Compressed Size: {{ compressedSize }} bytes
  </p>
  <img v-show="compressedJS" :src="compressedJS" />
  <p>
    js： Original Size: {{ originalSize }} bytes<br />
    Compressed Size: {{ compressedSizeJS }} bytes
  </p>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import init, { compress_image } from '../wasm/image_compressor.js'

const compressed = ref('')
const originalSize = ref(0)
const compressedSize = ref(0)

const compressedJS = ref('')
const compressedSizeJS = ref(0)

async function handleFileChange(event: Event) {
  const fileInput = event.target as HTMLInputElement
  if (!fileInput.files || fileInput.files.length === 0) {
    return
  }

  await init()
  const file = fileInput.files[0]
  const reader = new FileReader()
  reader.onload = function (event) {
    // wasm
    originalSize.value = file.size
    const arrayBuffer = event.target?.result as ArrayBuffer
    const uint8Array = new Uint8Array(arrayBuffer)
    compressImageWasm(uint8Array, 70)

    // js
    compressImageJS(uint8Array, 70).then((compressedData) => {})
  }
  reader.readAsArrayBuffer(file)
}

async function compressImageWasm(uint8Array: Uint8Array, quality: number) {
  console.log('start：', performance.now())
  const startTime = performance.now()
  const compressedData = compress_image(uint8Array, quality)
  const blob = new Blob([compressedData], { type: 'image/png' })
  const url = URL.createObjectURL(blob)
  compressed.value = url
  // 记录压缩后的文件大小
  compressedSize.value = blob.size
  const endTime = performance.now()
  console.log(`Compression time: ${endTime - startTime} ms`)
}

function compressImageJS(uint8Array: Uint8Array, quality: number) {
  handleCompressImageJS(uint8Array, quality).then((compressedData) => {
    const blob = new Blob([compressedData], { type: 'image/png' })
    const url = URL.createObjectURL(blob)
    compressedJS.value = url
    // 记录压缩后的文件大小
    compressedSizeJS.value = blob.size
  })
}

async function handleCompressImageJS(uint8Array: Uint8Array, quality: number) {
  const startTime = performance.now()
  const blob = new Blob([uint8Array], { type: 'image/jpeg' })
  const image = await createImageBitmap(blob)
  const canvas = document.createElement('canvas')
  canvas.width = image.width
  canvas.height = image.height

  const ctx = canvas.getContext('2d')
  if (!ctx) {
    throw new Error('Canvas context is not available')
  }

  ctx.drawImage(image, 0, 0, canvas.width, canvas.height)

  // 调整画布大小
  canvas.width = 80
  canvas.height = 80
  ctx.drawImage(image, 0, 0, 80, 80)

  return new Promise((resolve, reject) => {
    canvas.toBlob(
      (blob) => {
        if (!blob) {
          reject(new Error('Failed to compress image'))
        } else {
          const endTime = performance.now()
          console.log(`Compression time (JS): ${endTime - startTime} ms`)
          resolve(blob.arrayBuffer())
        }
      },
      'image/png',
      quality / 100
    )
  })
}

onMounted(() => {})
</script>
<style scoped>
img {
  width: 80px;
  height: 80px;
}
</style>
