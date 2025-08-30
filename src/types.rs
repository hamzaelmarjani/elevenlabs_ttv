use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request body for Text-to-Voice: Design Voice API calls
#[derive(Debug, Clone, Serialize)]
pub struct TTVDesignVoiceRequest {
    // Description to use for the created voice.
    pub voice_description: String,

    // Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    // MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    // Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    // Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    // Default to: mp3_44100_128
    // This goes in the URL path, not in the body.
    #[serde(skip_serializing)]
    pub output_format: Option<String>,

    // Model to use for the voice generation. Possible values: eleven_multilingual_ttv_v2, eleven_ttv_v3.
    // Default to eleven_multilingual_ttv_v2.
    pub model_id: Option<String>,

    // Text to generate, text length has to be between 100 and 1000.
    // >=100 characters <=1000 characters
    pub text: Option<String>,

    // Whether to automatically generate a text suitable for the voice description.
    pub auto_generate_text: Option<bool>,

    // Controls the volume level of the generated voice. -1 is quietest, 1 is loudest, 0 corresponds to roughly -24 LUFS.
    // >=-1 <=1 Defaults to 0.5
    pub loudness: Option<f32>,

    // If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result.
    // Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    pub seed: Option<u32>,

    // Controls how closely the AI follows the prompt. Lower numbers give the AI more freedom to be creative, while higher numbers force it to stick more to the prompt.
    // High numbers can cause voice to sound artificial or robotic. We recommend to use longer, more detailed prompts at lower Guidance Scale.
    // >=0 <=100 Defaults to 5
    pub guidance_scale: Option<u32>,

    // Determines whether the Text to Voice previews should be included in the response.
    // If true, only the generated IDs will be returned which can then be streamed via the /v1/text-to-voice/:generated_voice_id/stream endpoint.
    // Defaults to false
    pub stream_previews: Option<bool>,

    // The remixing session id.
    pub remixing_session_id: Option<String>,

    // The id of the remixing session iteration where these generations should be attached to. If not provided, a new iteration will be created.
    pub remixing_session_iteration_id: Option<String>,

    // Higher quality results in better voice output but less variety.
    // >=-1.0 <=1.0 Default to 0.0
    pub quality: Option<f32>,

    // Reference audio to use for the voice generation. The audio should be base64 encoded.
    // Only supported when using the eleven_ttv_v3 model.
    pub reference_audio_base64: Option<String>,

    // Controls the balance of prompt versus reference audio when generating voice samples.
    // 0 means almost no prompt influence, 1 means almost no reference audio influence.
    // Only supported when using the eleven_ttv_v3 model and providing reference audio.
    // >=0 <=1 Default to 0
    pub prompt_strength: Option<f32>,
}

/// Request body for Text-to-Voice: Create Voice API calls
#[derive(Debug, Clone, Serialize)]
pub struct TTVCreateVoiceRequest {
    // Name to use for the created voice.
    pub voice_name: String,

    // Description to use for the created voice.
    pub voice_description: String,

    // The generated_voice_id (from Design Voice) to create, call POST /v1/text-to-voice/create-previews and fetch the generated_voice_id from the response header if don’t have one yet.
    pub generated_voice_id: String,

    // Optional, metadata to add to the created voice. Defaults to None.
    pub labels: Option<String>,

    // List of voice ids that the user has played but not selected. Used for RLHF.
    pub played_not_selected_voice_ids: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTVDesignVoiceResponse {
    /// List of voice previews
    pub previews: Vec<TTVDesignVoiceResponseVoicePreview>,
    /// The text used to preview the voices
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTVDesignVoiceResponseVoicePreview {
    /// The base64 encoded audio of the preview
    pub audio_base_64: String,
    /// The ID of the generated voice. Use it to create a voice from the preview
    pub generated_voice_id: String,
    /// The media type of the preview
    pub media_type: String,
    /// The duration of the preview in seconds
    pub duration_secs: f64,
    /// The language of the preview (can be None)
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTVCreateVoiceResponse {
    pub voice_id: String,
    pub name: Option<String>,
    pub samples: Option<Vec<Sample>>,
    pub category: Option<VoiceCategory>,
    pub fine_tuning: Option<FineTuning>,
    pub labels: Option<HashMap<String, String>>,
    pub description: Option<String>,
    pub preview_url: Option<String>,
    pub available_for_tiers: Option<Vec<String>>,
    pub settings: Option<VoiceSettings>,
    pub sharing: Option<VoiceSharing>,
    pub high_quality_base_model_ids: Option<Vec<String>>,
    pub verified_languages: Option<Vec<VerifiedLanguage>>,
    pub safety_control: Option<SafetyControl>,
    pub voice_verification: Option<VoiceVerification>,
    pub permission_on_resource: Option<String>,
    pub is_owner: Option<bool>,
    #[serde(default)]
    pub is_legacy: Option<bool>,
    #[serde(default)]
    pub is_mixed: Option<bool>,
    pub favorited_at_unix: Option<i64>,
    pub created_at_unix: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub sample_id: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub hash: Option<String>,
    pub duration_secs: Option<f64>,
    pub remove_background_noise: Option<bool>,
    pub has_isolated_audio: Option<bool>,
    pub has_isolated_audio_preview: Option<bool>,
    pub speaker_separation: Option<SpeakerSeparation>,
    pub trim_start: Option<i64>,
    pub trim_end: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerSeparation {
    pub voice_id: String,
    pub sample_id: String,
    pub status: SeparationStatus,
    pub speakers: Option<HashMap<String, Speaker>>,
    pub selected_speaker_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeparationStatus {
    NotStarted,
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speaker {
    pub speaker_id: String,
    pub duration_secs: f64,
    pub utterances: Option<Vec<Utterance>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utterance {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceCategory {
    Generated,
    Cloned,
    Premade,
    Professional,
    Famous,
    HighQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuning {
    pub is_allowed_to_fine_tune: Option<bool>,
    pub state: Option<HashMap<String, FineTuningState>>,
    pub verification_failures: Option<Vec<String>>,
    pub verification_attempts_count: Option<i64>,
    pub manual_verification_requested: Option<bool>,
    pub language: Option<String>,
    pub progress: Option<HashMap<String, f64>>,
    pub message: Option<HashMap<String, String>>,
    pub dataset_duration_seconds: Option<f64>,
    pub verification_attempts: Option<Vec<VerificationAttempt>>,
    pub slice_ids: Option<Vec<String>>,
    pub manual_verification: Option<ManualVerification>,
    pub max_verification_attempts: Option<i64>,
    pub next_max_verification_attempts_reset_unix_ms: Option<i64>,
    pub finetuning_state: Option<serde_json::Value>, // Using Value for "any" type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FineTuningState {
    NotStarted,
    Queued,
    FineTuning,
    FineTuned,
    Failed,
    Delayed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAttempt {
    pub text: String,
    pub date_unix: i64,
    pub accepted: bool,
    pub similarity: f64,
    pub levenshtein_distance: f64,
    pub recording: Option<Recording>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub recording_id: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub upload_date_unix: i64,
    pub transcription: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualVerification {
    pub extra_text: String,
    pub request_time_unix: i64,
    pub files: Vec<VerificationFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationFile {
    pub file_id: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub upload_date_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub stability: Option<f64>,
    pub use_speaker_boost: Option<bool>,
    pub similarity_boost: Option<f64>,
    pub style: Option<f64>,
    pub speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSharing {
    pub status: Option<SharingStatus>,
    pub history_item_sample_id: Option<String>,
    pub date_unix: Option<i64>,
    pub whitelisted_emails: Option<Vec<String>>,
    pub public_owner_id: Option<String>,
    pub original_voice_id: Option<String>,
    pub financial_rewards_enabled: Option<bool>,
    pub free_users_allowed: Option<bool>,
    pub live_moderation_enabled: Option<bool>,
    pub rate: Option<f64>,
    pub fiat_rate: Option<f64>,
    pub notice_period: Option<i64>,
    pub disable_at_unix: Option<i64>,
    pub voice_mixing_allowed: Option<bool>,
    pub featured: Option<bool>,
    pub category: Option<VoiceCategory>,
    pub reader_app_enabled: Option<bool>,
    pub image_url: Option<String>,
    pub ban_reason: Option<String>,
    pub liked_by_count: Option<i64>,
    pub cloned_by_count: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub review_status: Option<ReviewStatus>,
    pub review_message: Option<String>,
    pub enabled_in_library: Option<bool>,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub youtube_username: Option<String>,
    pub tiktok_username: Option<String>,
    pub moderation_check: Option<ModerationCheck>,
    pub reader_restricted_on: Option<Vec<ReaderRestriction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharingStatus {
    Enabled,
    Disabled,
    Copied,
    CopiedDisabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    NotRequested,
    Pending,
    Declined,
    Allowed,
    AllowedWithChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationCheck {
    pub date_checked_unix: Option<i64>,
    pub name_value: Option<String>,
    pub name_check: Option<bool>,
    pub description_value: Option<String>,
    pub description_check: Option<bool>,
    pub sample_ids: Option<Vec<String>>,
    pub sample_checks: Option<Vec<f64>>,
    pub captcha_ids: Option<Vec<String>>,
    pub captcha_checks: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderRestriction {
    pub resource_type: ResourceType,
    pub resource_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Read,
    Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedLanguage {
    pub language: String,
    pub model_id: String,
    pub accent: Option<String>,
    pub locale: Option<String>,
    pub preview_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SafetyControl {
    None,
    Ban,
    Captcha,
    EnterpriseBan,
    EnterpriseCaptcha,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceVerification {
    pub requires_verification: bool,
    pub is_verified: bool,
    pub verification_failures: Vec<String>,
    pub verification_attempts_count: i64,
    pub language: Option<String>,
    pub verification_attempts: Option<Vec<VerificationAttempt>>,
}

// Example usage and helper implementations
impl TTVCreateVoiceResponse {
    /// Check if the voice is ready for use
    pub fn is_ready(&self) -> bool {
        if let Some(verification) = &self.voice_verification {
            !verification.requires_verification || verification.is_verified
        } else {
            true
        }
    }

    /// Get the total duration of all samples
    pub fn total_sample_duration(&self) -> f64 {
        self.samples
            .as_ref()
            .map(|samples| samples.iter().filter_map(|s| s.duration_secs).sum())
            .unwrap_or(0.0)
    }

    /// Check if voice sharing is enabled
    pub fn is_shared(&self) -> bool {
        self.sharing
            .as_ref()
            .and_then(|s| s.status.as_ref())
            .map(|status| matches!(status, SharingStatus::Enabled))
            .unwrap_or(false)
    }
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: Some(0.5),
            use_speaker_boost: Some(true),
            similarity_boost: Some(0.5),
            style: Some(0.0),
            speed: Some(1.0),
        }
    }
}
