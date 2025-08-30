use elevenlabs_ttv::ElevenLabsTTVClient;
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
