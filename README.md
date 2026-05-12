# Web Voice Train (Vue + Rust/WASM)

Web app that records voice and returns basic DSP metrics:

- pitch estimate (Hz)
- formants (F1/F2/F3)

## Stack

- Vue 3 + Vite for the web UI
- Rust compiled to WebAssembly (`wasm-pack`)
- A Rust DSP module under `rust-dsp/`

## Project Layout

- `src/App.vue`: one-page UI with record/stop button and results
- `src/services/wasmAnalyzer.js`: decodes recorded audio and calls WASM
- `rust-dsp/src/lib.rs`: exported WASM analyzer function (`analyze_audio`)

## Setup

1. Install dependencies:

   ```bash
   npm install
   ```

2. Install Rust and wasm-pack (if not already installed):

   ```bash
   rustup default stable
   cargo install wasm-pack
   ```

3. Build the WASM package:

   ```bash
   npm run wasm:build
   ```

4. Start the app:

   ```bash
   npm run dev
   ```

## Current Scope

- Uses recorded audio (start/stop recording), then runs analysis
- Real-time streaming is not wired yet (next milestone)

## Next steps

- Real-time analisis
- Gendered objectives
- Better local implementation of voice analysis
- Add relevant gendered voice criteria
- improve UI

## Notes on loqa-voice-dsp

The Rust crate `loqa-voice-dsp` is included in `rust-dsp/Cargo.toml` as the target DSP engine.

This app allows to choose between local implementation of DSP and the `loqa-voice-dsp` implementation. A further objective will be to improve local impelementation and compare the results.
