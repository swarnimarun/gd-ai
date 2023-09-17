mod audio;
mod helper;
mod model;
mod resample;
mod token;
mod transcribe;

use std::path::Path;

use burn::module::Module;
use burn_wgpu::{AutoGraphicsApi, WgpuBackend, WgpuDevice};
use godot::{engine::ControlVirtual, prelude::*};
use model::{load::load_whisper, Whisper, WhisperConfig};
use transcribe::waveform_to_text;

use crate::token::Gpt2Tokenizer;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

type CrossBackend = WgpuBackend<AutoGraphicsApi, f32, i32>;

#[derive(GodotClass)]
#[class(base=Control)]
struct TranscribeAI {
    model: Option<(Whisper<CrossBackend>, WhisperConfig)>,
}

#[godot_api]
impl ControlVirtual for TranscribeAI {
    fn init(_control: Base<godot::engine::Control>) -> Self {
        godot_print!("intialize ai node");
        Self { model: None }
    }
}

#[godot_api]
impl TranscribeAI {
    /// load model, required to return true before rest of the functions can work
    fn load_model(&mut self, path_to_model: String) -> bool {
        self.model = load_whisper(&path_to_model).ok();
        if self.model.is_none() {
            false
        } else {
            true
        }
    }

    /// we resample to 32bit float 16kHz samples so try to provide data in a similar shape for faster performance
    fn transcribe_wav_file(&self, path_to_wav: impl AsRef<Path>) -> Option<String> {
        if self.model.is_none() {
            return None;
        }
        // TODO(swarnim): implement!
        None
    }

    /// we only support single channel data: aka pcm samples
    /// we don't resample some please provide,
    /// 32bit float 16kHz samples
    fn transcribe_wav_data(&self, wav_data: impl AsRef<[f32]>) -> Option<String> {
        if self.model.is_none() {
            return None;
        }

        let bpe = match Gpt2Tokenizer::new() {
            Ok(bpe) => bpe,
            Err(e) => {
                #[cfg(debug_assertions)]
                eprintln!("Failed to create new tokenizer, due to {}", e);
                return None;
            }
        };

        let Some((ref model, ref _config)) = self.model else {
            return None;
        };
        // pick the strongest device for AI offloading
        let whisper = model.clone().to_device(&WgpuDevice::BestAvailable);

        let data = wav_data.as_ref().to_vec();
        let (text, _tokens) = match waveform_to_text(&whisper, &bpe, data, 16000) {
            Ok((text, tokens)) => (text, tokens),
            Err(e) => {
                #[cfg(debug_assertions)]
                eprintln!("Failed to transcribe text, due to {}", e);
                return None;
            }
        };

        Some(text)
    }
}
