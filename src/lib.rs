//! ElevenLabs Text-to-Voice API client
//!
//! A type-safe, async Rust client for the ElevenLabs TTV API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_ttv::ElevenLabsTTVClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsTTVClient::new("your-api-key");
//!     
//!     let designed_voice = client
//!         .design_voice("Friendly male, late 20s, neutral American accent, modern and clear like a product demo.")
//!         .execute()
//!         .await?;
//!
//!     let designed_voice_id = designed_voice.previews.first().unwrap().generated_voice_id.clone();
//!     
//!     // results as TTVDesignVoiceResponse struct
//!     println!("Design Voice Results: {:?}", designed_voice);
//!
//!
//!     
//!    let created_voice = client
//!         .create_voice("Jack", "Friendly male, late 20s, neutral American accent, modern and clear like a product demo.", &designed_voice_id)
//!         .execute()
//!         .await?;
//!     
//!     // results as TTVCreateVoiceResponse struct
//!     println!("Created Voice Results: {:?}", created_voice);
//!
//!
//!     
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;

pub use error::ElevenLabsTTVError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsTTVClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsTTVClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a Text-to-Voice: Design Voice request
    ///
    /// Requires the description to use for the created voice.
    pub fn design_voice<S: Into<String>>(
        &self,
        voice_description: S,
    ) -> TextToVoiceDesignVoiceBuilder {
        TextToVoiceDesignVoiceBuilder::new(self.clone(), voice_description.into())
    }

    /// Start building a Text-to-Voice: Create Voice request
    pub fn create_voice<S: Into<String>>(
        &self,
        voice_name: S,
        voice_description: S,
        generated_voice_id: S,
    ) -> TextToVoiceCreateVoiceBuilder {
        TextToVoiceCreateVoiceBuilder::new(
            self.clone(),
            voice_name.into(),
            voice_description.into(),
            generated_voice_id.into(),
        )
    }

    /// Internal method to execute TTV: Design Voice request
    pub(crate) async fn execute_design_voice(
        &self,
        request: TTVDesignVoiceRequest,
    ) -> Result<TTVDesignVoiceResponse, ElevenLabsTTVError> {
        let mut url = format!("{}/text-to-voice/design", self.base_url);

        let output_format = request
            .output_format
            .clone()
            .unwrap_or_else(|| "mp3_44100_128".to_string()); // Default to: mp3_44100_128

        url = format!("{}?output_format={}", url, output_format);

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            // println!("Response: {:?}", response);
            return Err(ElevenLabsTTVError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let parse_response = response.json::<TTVDesignVoiceResponse>().await;

        match parse_response {
            Ok(ttv_response) => return Ok(ttv_response),
            Err(e) => return Err(ElevenLabsTTVError::ParseError(e)),
        }
    }

    /// Internal method to execute TTV: Create Voice request
    pub(crate) async fn execute_create_voice(
        &self,
        request: TTVCreateVoiceRequest,
    ) -> Result<TTVCreateVoiceResponse, ElevenLabsTTVError> {
        let url = format!("{}/text-to-voice", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsTTVError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let parse_response = response.json::<TTVCreateVoiceResponse>().await;

        match parse_response {
            Ok(ttv_response) => return Ok(ttv_response),
            Err(e) => return Err(ElevenLabsTTVError::ParseError(e)),
        }
    }
}

/// Builder for Text-to-Voice: Design Voice requests
pub struct TextToVoiceDesignVoiceBuilder {
    client: ElevenLabsTTVClient,
    pub voice_description: String,
    pub output_format: Option<String>,
    pub model_id: Option<String>,
    pub text: Option<String>,
    pub auto_generate_text: Option<bool>,
    pub loudness: Option<f32>,
    pub seed: Option<u32>,
    pub guidance_scale: Option<u32>,
    pub stream_previews: Option<bool>,
    pub remixing_session_id: Option<String>,
    pub remixing_session_iteration_id: Option<String>,
    pub quality: Option<f32>,
    pub reference_audio_base64: Option<String>,
    pub prompt_strength: Option<f32>,
}

impl TextToVoiceDesignVoiceBuilder {
    fn new(client: ElevenLabsTTVClient, voice_description: String) -> Self {
        Self {
            client,
            voice_description,
            output_format: None,
            text: None,
            model_id: None,
            auto_generate_text: None,
            loudness: None,
            seed: None,
            guidance_scale: None,
            stream_previews: None,
            remixing_session_id: None,
            remixing_session_iteration_id: None,
            quality: None,
            reference_audio_base64: None,
            prompt_strength: None,
        }
    }

    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    /// MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    /// Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    /// Default to: mp3_44100_128
    /// This goes in the URL path, not in the body.
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Text to generate, text length has to be between 100 and 1000.
    /// >=100 characters <=1000 characters
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Model to use for the voice generation. Possible values: eleven_multilingual_ttv_v2, eleven_ttv_v3.
    /// Default to eleven_multilingual_ttv_v2.
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Whether to automatically generate a text suitable for the voice description.
    pub fn auto_generate_text<B: Into<bool>>(mut self, auto_generate_text: B) -> Self {
        self.auto_generate_text = Some(auto_generate_text.into());
        self
    }

    /// Controls the volume level of the generated voice. -1 is quietest, 1 is loudest, 0 corresponds to roughly -24 LUFS.
    /// >=-1 <=1 Defaults to 0.5
    pub fn loudness(mut self, loudness: f32) -> Self {
        self.loudness = Some(loudness);
        self
    }

    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Controls how closely the AI follows the prompt. Lower numbers give the AI more freedom to be creative, while higher numbers force it to stick more to the prompt.
    /// High numbers can cause voice to sound artificial or robotic. We recommend to use longer, more detailed prompts at lower Guidance Scale.
    /// >=0 <=100 Defaults to 5
    pub fn guidance_scale(mut self, guidance_scale: u32) -> Self {
        self.guidance_scale = Some(guidance_scale.into());
        self
    }

    /// Determines whether the Text to Voice previews should be included in the response.
    /// If true, only the generated IDs will be returned which can then be streamed via the /v1/text-to-voice/:generated_voice_id/stream endpoint.
    /// Defaults to false
    pub fn stream_previews<B: Into<bool>>(mut self, stream_previews: B) -> Self {
        self.stream_previews = Some(stream_previews.into());
        self
    }

    /// The remixing session id.
    pub fn remixing_session_id<S: Into<String>>(mut self, remixing_session_id: S) -> Self {
        self.remixing_session_id = Some(remixing_session_id.into());
        self
    }

    /// The id of the remixing session iteration where these generations should be attached to.
    /// If not provided, a new iteration will be created.
    pub fn remixing_session_iteration_id<S: Into<String>>(
        mut self,
        remixing_session_iteration_id: S,
    ) -> Self {
        self.remixing_session_iteration_id = Some(remixing_session_iteration_id.into());
        self
    }

    /// Higher quality results in better voice output but less variety.
    /// >=-1.0 <=1.0 Default to 0.0
    pub fn quality(mut self, quality: f32) -> Self {
        self.quality = Some(quality.into());
        self
    }

    /// Reference audio to use for the voice generation. The audio should be base64 encoded.
    /// Only supported when using the eleven_ttv_v3 model.
    pub fn reference_audio_base64<S: Into<String>>(mut self, reference_audio_base64: S) -> Self {
        self.reference_audio_base64 = Some(reference_audio_base64.into());
        self
    }

    /// Controls the balance of prompt versus reference audio when generating voice samples.
    /// 0 means almost no prompt influence, 1 means almost no reference audio influence.
    /// Only supported when using the eleven_ttv_v3 model and providing reference audio.
    /// >=0 <=1 Default to 0
    pub fn prompt_strength(mut self, prompt_strength: f32) -> Self {
        self.prompt_strength = Some(prompt_strength.into());
        self
    }

    /// Execute the Text-to-Voice: Design Voice request
    pub async fn execute(self) -> Result<TTVDesignVoiceResponse, ElevenLabsTTVError> {
        let request = TTVDesignVoiceRequest {
            voice_description: self.voice_description,
            model_id: Some(self.model_id.unwrap_or_else(|| {
                models::elevanlabs_models::ELEVEN_MULTILINGUAL_TTV_V2.to_string()
            })), // Default to: eleven_multilingual_ttv_v2
            output_format: None,
            text: self.text.clone().or(None),
            auto_generate_text: self.auto_generate_text.or(if self.text.is_some() {
                None
            } else {
                Some(true)
            }),
            loudness: self.loudness.or(Some(0.5)),
            seed: self.seed.or(None),
            guidance_scale: self.guidance_scale.or(Some(5)),
            stream_previews: self.stream_previews.or(Some(false)),
            remixing_session_id: self.remixing_session_id.or(None),
            remixing_session_iteration_id: self.remixing_session_iteration_id.or(None),
            quality: self.quality.or(None),
            reference_audio_base64: self.reference_audio_base64.or(None),
            prompt_strength: self.prompt_strength.or(None),
        };

        self.client.execute_design_voice(request).await
    }
}

/// Builder for Text-to-Voice: Create Voice requests
pub struct TextToVoiceCreateVoiceBuilder {
    client: ElevenLabsTTVClient,
    voice_name: String,
    voice_description: String,
    generated_voice_id: String,
    labels: Option<String>,
    played_not_selected_voice_ids: Option<String>,
}

impl TextToVoiceCreateVoiceBuilder {
    fn new(
        client: ElevenLabsTTVClient,
        voice_name: String,
        voice_description: String,
        generated_voice_id: String,
    ) -> Self {
        Self {
            client,
            voice_name,
            voice_description,
            generated_voice_id,
            labels: None,
            played_not_selected_voice_ids: None,
        }
    }

    /// Set the labels to use
    pub fn labels<S: Into<String>>(mut self, labels: S) -> Self {
        self.labels = Some(labels.into());
        self
    }

    /// Set the played not selected voice ids
    pub fn played_not_selected_voice_ids<S: Into<String>>(
        mut self,
        played_not_selected_voice_ids: S,
    ) -> Self {
        self.played_not_selected_voice_ids = Some(played_not_selected_voice_ids.into());
        self
    }

    /// Execute the Text-to-Voice: Create Voice request
    pub async fn execute(self) -> Result<TTVCreateVoiceResponse, ElevenLabsTTVError> {
        let request = TTVCreateVoiceRequest {
            voice_name: self.voice_name,
            generated_voice_id: self.generated_voice_id,
            voice_description: self.voice_description,
            labels: self.labels.or(None),
            played_not_selected_voice_ids: self.played_not_selected_voice_ids.or(None),
        };

        self.client.execute_create_voice(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsTTVClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern_design_voice() {
        let client = ElevenLabsTTVClient::new("test-key");
        let builder = client
            .design_voice(
                "Warm, friendly female, mid-20s, neutral American accent, casual and supportive",
            )
            .model("model-456");

        // Builder pattern works
        assert_eq!(
            builder.voice_description,
            "Warm, friendly female, mid-20s, neutral American accent, casual and supportive"
                .to_string()
        );
        assert_eq!(builder.model_id, Some("model-456".to_string()));
    }

    #[test]
    fn test_builder_pattern_create_voice() {
        let client = ElevenLabsTTVClient::new("test-key");
        let builder = client
            .create_voice(
                "Elina",
                "Warm, friendly female, mid-20s, neutral American accent, casual and supportive",
                "generated-voice-id",
            )
            .labels("voice-labels");

        // Builder pattern works
        assert_eq!(builder.voice_name, "Elina".to_string());
        assert_eq!(
            builder.voice_description,
            "Warm, friendly female, mid-20s, neutral American accent, casual and supportive"
                .to_string()
        );
        assert_eq!(builder.generated_voice_id, "generated-voice-id".to_string());
        assert_eq!(builder.labels, Some("voice-labels".to_string()));
    }
}
