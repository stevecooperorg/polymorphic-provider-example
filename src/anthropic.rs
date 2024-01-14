use crate::Provider;

pub struct AnthropicProvider {
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }
}

impl Provider for AnthropicProvider {
    async fn get_request(&self, request: &str) -> String {
        format!(
            "AnthropicProvider: {} using key '{}'",
            request, self.api_key
        )
    }

    async fn process_sse(&self, event: String) -> String {
        format!("AnthropicProvider: {}", event)
    }
}
