let wasmModule = null

async function loadWasmModule() {
  if (wasmModule) {
    return wasmModule
  }

  const wasmJsUrl = new URL('../../rust-dsp/pkg/voice_dsp_wasm.js', import.meta.url).href
  const mod = await import(/* @vite-ignore */ wasmJsUrl)
  await mod.default()
  wasmModule = mod
  return wasmModule
}

async function decodeBlobToMono(blob) {
  const arrayBuffer = await blob.arrayBuffer()
  const audioContext = new AudioContext()
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer.slice(0))
  const channelData = audioBuffer.getChannelData(0)
  const mono = new Float32Array(channelData.length)
  mono.set(channelData)
  await audioContext.close()

  return {
    samples: mono,
    sampleRate: audioBuffer.sampleRate,
    durationSeconds: audioBuffer.duration,
  }
}

export async function analyzeRecordedBlob(blob, method = 'baseline') {
  const decoded = await decodeBlobToMono(blob)
  const mod = await loadWasmModule()
  const result = mod.analyze_audio_with_method(decoded.samples, decoded.sampleRate, method)

  return {
    ...result,
    durationSeconds: decoded.durationSeconds,
  }
}
