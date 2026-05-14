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
- Window audio in windows of 40-50 ms
- Filter voiced segments (necessary, except for VOT) (is_voices of PitchResult (loqa crate))
- graph pitch (+ range SD, once averaged on 5-10s if real-time)
- graph H1-H2 as a function of time
- Ability to set gendered objectives
- Better local implementation of voice analysis
- Add relevant gendered voice criteria
- improve UI

## Notes from 1.

### Formants
Triangle vocalique puis moyenne sur plusieurs occurences de chaque voyelle (on peut d'abord se restreindre à /i/, /u/ et /a/)
Visualisation des zones moyennes femme/homme pour chaque voyelle.

### F0
Comme Voice Tools (spectrogramme avec range visible).

#### fr
F0 moyenne: (parole lue/semie-spontanée)
* H: 111/101 Hz
* F: 211/186 Hz

F0 range:
* H: 74/136 Hz
* F: 198/287 Hz

#### en
F0 moyenne: (parole lue/semie-spontanée)
* H: 110/100 Hz
* F: 195/181 Hz

F0 range:
* H: 83/146 Hz
* F: 196/275 Hz

### VOT
Sur un axe temporel

(plus élevé chez les femmes pour les occlusives sourdes, différence plus ambigue sur les occlusives voisées, autant en français qu'en anglais)

#### fr
VOT moyen non voisées:
* H: 43 ms
* F: 52 ms 

VOT moyen voisées:
* H: -63 ms
* F: -70 ms

#### en
VOT moyen non voisées:
* H: 58 ms
* F: 73 ms

VOT moyen voisées:
* H: 1 ms
* F: 0 ms

### H1-H2
Sur un axe d'intensité

Corrélation positive avec le glottal open quotient
beaucoup plus élevé chez les femmes que chez les hommes
(différence encore plus importante en anglais)

#### fr
* H: 1,96 dB
* F: 5,60 dB

#### en
* H: 0,81 dB
* F: 6,20 dB

## Notes on loqa-voice-dsp

The Rust crate `loqa-voice-dsp` is included in `rust-dsp/Cargo.toml` as the target DSP engine.

This app allows to choose between local implementation of DSP and the `loqa-voice-dsp` implementation. A further objective will be to improve local impelementation and compare the results.

## References

1. Pépiot, Erwan, and Arnold, Aron. “Cross-Gender Differences in English/French Bilingual Speakers: A Multiparametric Study.” Perceptual and Motor Skills, vol. 128, no. 1, 2020, pp. 153–77, <https://doi.org/10.1177/0031512520973514>.
