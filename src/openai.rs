use crate::Provider;

pub struct OpenAiProvider {
    api_key: String,
}

impl OpenAiProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }
}

impl Provider for OpenAiProvider {
    async fn get_request(&self, request: &str) -> String {
        format!("OpenAiProvider: {} using key '{}'", request, self.api_key)
    }

    async fn process_sse(&self, event: String) -> String {
        format!("OpenAiProvider: {}", event)
    }
}
