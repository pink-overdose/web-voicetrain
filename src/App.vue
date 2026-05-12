<script setup>
import { ref } from 'vue'
import { analyzeRecordedBlob } from './services/wasmAnalyzer'

const isRecording = ref(false)
const isAnalyzing = ref(false)
const message = ref('Ready to record')
const analysis = ref(null)
const recordedAudioUrl = ref('')
const selectedMethod = ref('baseline')

let mediaRecorder = null
let chunks = []

async function toggleRecording() {
  if (isRecording.value) {
    mediaRecorder?.stop()
    return
  }

  analysis.value = null
  message.value = 'Requesting microphone access...'

  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true })
    mediaRecorder = new MediaRecorder(stream)
    chunks = []

    mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        chunks.push(event.data)
      }
    }

    mediaRecorder.onstop = async () => {
      stream.getTracks().forEach((track) => track.stop())
      const blob = new Blob(chunks, { type: 'audio/webm' })
      recordedAudioUrl.value = URL.createObjectURL(blob)
      await runAnalysis(blob)
    }

    mediaRecorder.start()
    isRecording.value = true
    message.value = 'Recording... click stop when done'
  } catch (error) {
    message.value = `Microphone error: ${error instanceof Error ? error.message : String(error)}`
    isRecording.value = false
  }
}

async function runAnalysis(blob) {
  isRecording.value = false
  isAnalyzing.value = true
  message.value = 'Analyzing audio...'

  try {
    analysis.value = await analyzeRecordedBlob(blob, selectedMethod.value)
    message.value = 'Analysis complete'
  } catch (error) {
    message.value = `Analysis failed: ${error instanceof Error ? error.message : String(error)}`
  } finally {
    isAnalyzing.value = false
  }
}
</script>

<template>
  <main class="page">
    <h1>Web Voice Train</h1>
    <p class="subtitle">Record audio and extract pitch + formants via Rust/WASM.</p>
    <label class="method-row">
      Analysis method:
      <select v-model="selectedMethod" :disabled="isRecording || isAnalyzing">
        <option value="baseline">Baseline (simple estimator)</option>
        <option value="loqa">Loqa Voice DSP</option>
      </select>
    </label>

    <button class="record-btn" :disabled="isAnalyzing" @click="toggleRecording">
      {{ isRecording ? 'Stop Recording' : 'Start Recording' }}
    </button>

    <p class="status">{{ message }}</p>

    <audio v-if="recordedAudioUrl" :src="recordedAudioUrl" controls class="player" />

    <section v-if="analysis" class="card">
      <h2>Results</h2>
      <p><strong>Sample Rate:</strong> {{ analysis.sampleRate }} Hz</p>
      <p><strong>Duration:</strong> {{ analysis.durationSeconds.toFixed(2) }} s</p>
      <p><strong>Method:</strong> {{ analysis.method }}</p>
      <p><strong>Pitch:</strong> {{ analysis.pitchHz?.toFixed(2) ?? 'N/A' }} Hz</p>
      <p><strong>Voiced:</strong> {{ analysis.isVoiced ? 'Yes' : 'No' }}</p>
      <p><strong>Formant F1:</strong> {{ analysis.formants?.f1?.toFixed(1) ?? 'N/A' }} Hz</p>
      <p><strong>Formant F2:</strong> {{ analysis.formants?.f2?.toFixed(1) ?? 'N/A' }} Hz</p>
      <p><strong>Formant F3:</strong> {{ analysis.formants?.f3?.toFixed(1) ?? 'N/A' }} Hz</p>
    </section>
  </main>
</template>
