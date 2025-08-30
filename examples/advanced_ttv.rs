use elevenlabs_ttv::{ElevenLabsTTVClient, models};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTVClient::new(api_key);

    let voice_description =
        "Bright, energetic female, early 20s, Californian accent, playful and fast-paced.";

    let designed_voice = client
        .design_voice(voice_description)
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_TTV_V2)
        .text("Hi! I’m your smart creative assistant. Tell me what you want to make, and I’ll help you design it—step by step. Ready when you are.")
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
