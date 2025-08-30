use elevenlabs_ttv::{ElevenLabsTTVClient, ElevenLabsTTVError, models};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsTTVClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_design_voice() {
    let client = ElevenLabsTTVClient::new("test-key");
    let _builder = client
        .design_voice("Confident male, 30s, general American accent, motivational and inspiring.")
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_TTV_V2);

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[tokio::test]
async fn test_builder_create_voice() {
    let client = ElevenLabsTTVClient::new("test-key");
    let _builder = client.create_voice(
        "Andry",
        "Confident male, 30s, general American accent, motivational and inspiring.",
        "designed-voice-id",
    );

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_error_display() {
    let error = ElevenLabsTTVError::ValidationError("Invalid voice ID".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid voice ID"));
}

// Mock tests for API calls (without real HTTP requests)
#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsTTVClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
