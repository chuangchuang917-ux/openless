//! Unified Dictation Pipeline Deep Module.
//! Encapsulates audio capture, ASR transcription, LLM polish/translation, and text insertion strategy.
//! Hides internal pipeline complexity behind a minimal interface: `start_session` and `cancel`.

use crate::types::{ChineseScriptPreference, OutputLanguagePreference, PolishMode, WindowsInsertionMode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PipelineSessionConfig {
    pub mode: PolishMode,
    pub chinese_script_preference: ChineseScriptPreference,
    pub output_language_preference: OutputLanguagePreference,
    pub translation_active: bool,
    pub translation_target: String,
}

impl Default for PipelineSessionConfig {
    fn default() -> Self {
        Self {
            mode: PolishMode::Light,
            chinese_script_preference: ChineseScriptPreference::Auto,
            output_language_preference: OutputLanguagePreference::Auto,
            translation_active: false,
            translation_target: String::new(),
        }
    }
}

pub struct PipelineOutcome {
    pub raw_text: String,
    pub final_text: String,
    pub is_streamed: bool,
    pub error: Option<String>,
}

pub struct DictationPipeline {
    cancelled: Arc<AtomicBool>,
}

impl DictationPipeline {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn reset_cancel(&self) {
        self.cancelled.store(false, Ordering::SeqCst);
    }

    pub fn is_streaming_eligible(
        streaming_enabled: bool,
        translation_active: bool,
        mode: PolishMode,
        raw_uses_llm: bool,
        script_pref: ChineseScriptPreference,
        windows_insertion_mode: crate::types::WindowsInsertionMode,
    ) -> bool {
        crate::coordinator::dictation::streaming_insert_eligible(
            streaming_enabled,
            translation_active,
            mode,
            raw_uses_llm,
            script_pref,
            windows_insertion_mode,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_lifecycle_and_cancellation() {
        let pipeline = DictationPipeline::new();
        assert!(!pipeline.is_cancelled());

        pipeline.cancel();
        assert!(pipeline.is_cancelled());

        pipeline.reset_cancel();
        assert!(!pipeline.is_cancelled());
    }

    #[test]
    fn pipeline_streaming_eligibility_delegation() {
        assert!(DictationPipeline::is_streaming_eligible(
            true,
            false,
            PolishMode::Light,
            false,
            ChineseScriptPreference::Traditional,
            crate::types::WindowsInsertionMode::SendInput,
        ));
    }
}
