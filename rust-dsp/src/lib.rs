use serde::Serialize;
use wasm_bindgen::prelude::*;
use loqa_voice_dsp::{detect_pitch, extract_formants};

#[derive(Serialize)]
struct Formants {
    f1: f32,
    f2: f32,
    f3: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalysisResult {
    sample_rate: u32,
    pitch_hz: Option<f32>,
    is_voiced: bool,
    formants: Option<Formants>,
    method: String,
}

#[wasm_bindgen]
pub fn analyze_audio(samples: Vec<f32>, sample_rate: u32) -> Result<JsValue, JsValue> {
    analyze_audio_with_method(samples, sample_rate, "baseline".to_string())
}

#[wasm_bindgen]
pub fn analyze_audio_with_method(
    samples: Vec<f32>,
    sample_rate: u32,
    method: String,
) -> Result<JsValue, JsValue> {
    if samples.is_empty() || sample_rate == 0 {
        return Err(JsValue::from_str("invalid input audio"));
    }

    let normalized_method = method.trim().to_lowercase();
    let (pitch_hz, is_voiced, formants) = if normalized_method == "loqa" {
        analyze_with_loqa(&samples, sample_rate)
    } else {
        analyze_with_baseline(&samples, sample_rate)
    };

    let result = AnalysisResult {
        sample_rate: sample_rate,
        pitch_hz: pitch_hz,
        is_voiced: is_voiced,
        formants,
        method: if normalized_method == "loqa" {
            "loqa".to_string()
        } else {
            "baseline".to_string()
        },
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn analyze_with_loqa(samples: &[f32], sample_rate: u32) -> (Option<f32>, bool, Option<Formants>) {
    let pitch = detect_pitch(samples, sample_rate, 80.0, 400.0).ok();
    let pitch_hz = pitch
        .as_ref()
        .and_then(|p| if p.is_voiced { Some(p.frequency) } else { None });
    let is_voiced = pitch.as_ref().map(|p| p.is_voiced).unwrap_or(false);

    let formants = extract_formants(samples, sample_rate, 14).ok().map(|f| Formants {
        f1: f.f1,
        f2: f.f2,
        f3: f.f3,
    });

    (pitch_hz, is_voiced, formants)
}

fn analyze_with_baseline(
    samples: &[f32],
    sample_rate: u32,
) -> (Option<f32>, bool, Option<Formants>) {
    let rms = (samples.iter().map(|v| v * v).sum::<f32>() / samples.len() as f32).sqrt();
    let is_voiced = rms > 0.01;
    let pitch_hz = if is_voiced {
        estimate_pitch_autocorrelation(samples, sample_rate)
    } else {
        None
    };
    let formants = if is_voiced {
        estimate_formants_fft_peaks(samples, sample_rate)
    } else {
        None
    };
    (pitch_hz, is_voiced, formants)
}

fn estimate_pitch_autocorrelation(samples: &[f32], sample_rate: u32) -> Option<f32> {
    let min_hz = 75.0f32;
    let max_hz = 450.0f32;
    let min_lag = (sample_rate as f32 / max_hz) as usize;
    let max_lag = (sample_rate as f32 / min_hz) as usize;

    if max_lag >= samples.len() || min_lag >= max_lag {
        return None;
    }

    let mut best_lag = 0usize;
    let mut best_corr = f32::MIN;
    for lag in min_lag..max_lag {
        let mut corr = 0.0f32;
        for i in 0..(samples.len() - lag) {
            corr += samples[i] * samples[i + lag];
        }
        if corr > best_corr {
            best_corr = corr;
            best_lag = lag;
        }
    }

    if best_lag == 0 {
        None
    } else {
        Some(sample_rate as f32 / best_lag as f32)
    }
}

fn estimate_formants_fft_peaks(samples: &[f32], sample_rate: u32) -> Option<Formants> {
    let n = 2048usize.min(samples.len());
    if n < 512 {
        return None;
    }

    // Naive DFT for bootstrap simplicity (replace with loqa_voice_dsp/realfft next).
    let mut magnitudes = vec![0.0f32; n / 2];
    for k in 0..(n / 2) {
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        for (i, sample) in samples.iter().take(n).enumerate() {
            let phase = 2.0f32 * std::f32::consts::PI * k as f32 * i as f32 / n as f32;
            re += sample * phase.cos();
            im -= sample * phase.sin();
        }
        magnitudes[k] = (re * re + im * im).sqrt();
    }

    let hz_per_bin = sample_rate as f32 / n as f32;
    let f1 = peak_in_band(&magnitudes, hz_per_bin, 200.0, 1000.0)?;
    let f2 = peak_in_band(&magnitudes, hz_per_bin, 800.0, 3000.0)?;
    let f3 = peak_in_band(&magnitudes, hz_per_bin, 1800.0, 4500.0)?;
    Some(Formants { f1, f2, f3 })
}

fn peak_in_band(magnitudes: &[f32], hz_per_bin: f32, min_hz: f32, max_hz: f32) -> Option<f32> {
    let start = (min_hz / hz_per_bin).floor() as usize;
    let end = ((max_hz / hz_per_bin).ceil() as usize).min(magnitudes.len().saturating_sub(1));
    if start >= end {
        return None;
    }

    let mut best_idx = start;
    let mut best_mag = f32::MIN;
    for idx in start..=end {
        if magnitudes[idx] > best_mag {
            best_mag = magnitudes[idx];
            best_idx = idx;
        }
    }
    Some(best_idx as f32 * hz_per_bin)
}
