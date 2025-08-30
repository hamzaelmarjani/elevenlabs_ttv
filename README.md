# elevenlabs_ttv

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_ttv.svg)](https://crates.io/crates/elevenlabs_ttv)
[![Docs.rs](https://docs.rs/elevenlabs_ttv/badge.svg)](https://docs.rs/elevenlabs_ttv)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Text To Voice API](https://elevenlabs.io/docs/api-reference/text-to-voice/design). Design a voice via a prompt. Ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring TTV requests
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Elevanlabs TTV APIs, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime

## Check-out Also:

**This project is part of a milestone to implement all ElevenLabs APIs in Rust.**

- **[Elevenlabs TTS](https://crates.io/crates/elevenlabs_tts)**: ElevenLabs Text-to-Speech API. ✅
- **[Elevenlabs TTD](https://crates.io/crates/elevenlabs_ttd)**: ElevenLabs Text-to-Dialogue API. ✅
- **[Elevenlabs STT](https://crates.io/crates/elevenlabs_stt)**: ElevenLabs Speech-to-Text API. ✅
- **[Elevenlabs SFX](https://crates.io/crates/elevenlabs_sfx)**: ElevenLabs Sound Effects API. ✅
- **[Elevenlabs VC](https://crates.io/crates/elevenlabs_vc)**: ElevenLabs Voice Changer API. ✅
- **[Elevenlabs VC](https://crates.io/crates/elevenlabs_vc)**: ElevenLabs Text To Voice API. ✅
- **Elevenlabs CM**: ElevenLabs Music Compose API. ⏳
- **Elevenlabs AUI**: ElevenLabs Audio Isolation API. ⏳
- **Elevenlabs DUB**: ElevenLabs Dubbing API. ⏳

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_ttv = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_ttv::{ElevenLabsTTVClient, voices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsTTVClient::new("your-api-key");

    let voice_description =
        "Smooth, elegant female, 30s, soft French accent, refined and sophisticated.";

    let designed_voice = client.design_voice(voice_description).execute().await?;
    let designed_voice_id = designed_voice
        .previews
        .first()
        .unwrap()
        .generated_voice_id
        .clone();

    println!("Designed Voice Results {:?}", designed_voice);

    let created_voice = client
        .create_voice("Elina", voice_description, &designed_voice_id)
        .execute()
        .await?;

    println!("Created Voice Results {:?}", created_voice);

    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_ttv::{ElevenLabsTTVClient, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTVClient::new(api_key);

    let voice_description =
        "Smooth, elegant female, 30s, soft French accent, refined and sophisticated.";

    let designed_voice = client.design_voice(voice_description).execute().await?;
    let designed_voice_id = designed_voice
        .previews
        .first()
        .unwrap()
        .generated_voice_id
        .clone();

    println!("Designed Voice Results {:?}", designed_voice);

    let created_voice = client
        .create_voice("Elina", voice_description, &designed_voice_id)
        .execute()
        .await?;

    println!("Created Voice Results {:?}", created_voice);

    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_ttv::{ElevenLabsTTVClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTVClient::new(api_key);

    let voice_description =
        "Bright, energetic female, early 20s, Californian accent, playful and fast-paced.";
    let voice_text =
        "Hi! I’m your smart creative assistant. Tell me what you want to make, and I’ll help you design it—step by step. Ready when you are."

    let designed_voice = client
        .design_voice(voice_description)
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_TTV_V2)
        .text(voice_text)
        .auto_generate_text(true)
        .loudness(1.0)
        .guidance_scale(20)
        .quality(1.0)
        .execute()
        .await?;
    let designed_voice_id = designed_voice
        .previews
        .first()
        .unwrap()
        .generated_voice_id
        .clone();

    println!("Designed Voice Results {:?}", designed_voice);

    let created_voice = client
        .create_voice("Andrea", voice_description, &designed_voice_id)
        .execute()
        .await?;

    println!("Created Voice Results {:?}", created_voice);

    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_ttv

# Run the advanced example
cargo run --example advanced_ttv
```

## API Overview

| Method                                     | Description                                                                                                 |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `ElevenLabsTTVClient::new(String)`         | Create client instance, requires API key (String), (required)\*                                             |
| `.design_voice(String)`                    | Run design voice exec, requires voice_description (String) (required)\*                                     |
| `.output_format(String)`                   | Output format of the generated audio (optional)                                                             |
| `.model_id(String)`                        | Model to use for the voice generation. (optional)                                                           |
| `.text(String)`                            | Text to transform to sample preview. (optional)                                                             |
| `.auto_generate_text(bool)`                | Automatically generate a text to transform to sample preview. (optional)                                    |
| `.loudness(f32)`                           | Controls the volume level of the generated voice. (optional)                                                |
| `.seed(u32)`                               | Our system will make a best effort to sample deterministically (optional)                                   |
| `.guidance_scale(u32)`                     | Controls how closely the AI follows the prompt (optional)                                                   |
| `.stream_previews(bool)`                   | Includes text to voice previews in the response (optional)                                                  |
| `.remixing_session_id(String)`             | The remixing session id (optional)                                                                          |
| `.remixing_session_iteration_id(String)`   | The id of the remixing session iteration (optional)                                                         |
| `.quality(f32)`                            | Higher quality results in better voice output but less variety (optional)                                   |
| `.reference_audio_base64(String)`          | Reference audio to use for the voice generation (optional)                                                  |
| `.prompt_strength(f32)`                    | Controls the balance of prompt versus reference audio (optional)                                            |
| `.execute()`                               | Run request → design voice (required)\*                                                                     |
| ------------------------------------------ | ----------------------------------------------------------------------------------                          |
| `ElevenLabsTTVClient::new(String)`         | Create client instance, requires API key (String), (required)\*                                             |
| `.create_voice(String, String , String)`   | Run create voice exec, requires voice_name, voice_description & generated_voice_id as (String) (required)\* |
| `.labels(String)`                          | Metadata to add to the created voice (optional)                                                             |
| `.played_not_selected_voice_ids(String)`   | List of voice ids that the user has played but not selected (optional)                                      |
| `.execute()`                               | Run request → create voice (required)\*                                                                     |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.design_voice(voice_description).execute().await {
    Ok(output) => println!("Designed voice results: {:?}", output),
    Err(e) => eprintln!("TTV request failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/text-to-voice/design) for the most up-to-date API information.
