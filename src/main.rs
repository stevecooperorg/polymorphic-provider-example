mod anthropic;
mod openai;

#[tokio::main]
async fn main() {
    let anthropic_provider = anthropic::AnthropicProvider::new("anthropic-key");
    let openai_provider = openai::OpenAiProvider::new("openai-key");
    let polymorpohic_provider =
        ApiProvider::Anthropic(anthropic::AnthropicProvider::new("super-secret-key"));

    println!("{}", generic_request(&anthropic_provider, "prompt").await);
    println!("{}", generic_request(&openai_provider, "prompt").await);
    println!(
        "{}",
        generic_request(&polymorpohic_provider, "prompt").await
    );
}

async fn generic_request<T: Provider>(provider: &T, request: &str) -> String {
    provider.get_request(request).await
}

pub enum ApiProvider {
    OpenAi(openai::OpenAiProvider),
    Anthropic(anthropic::AnthropicProvider),
}

trait Provider {
    async fn get_request(&self, request: &str) -> String;
    async fn process_sse(&self, event: String) -> String;
}

impl Provider for ApiProvider {
    async fn get_request(&self, request: &str) -> String {
        match self {
            ApiProvider::OpenAi(provider) => provider.get_request(request).await,
            ApiProvider::Anthropic(provider) => provider.get_request(request).await,
        }
    }

    async fn process_sse(&self, event: String) -> String {
        match self {
            ApiProvider::OpenAi(provider) => provider.process_sse(event).await,
            ApiProvider::Anthropic(provider) => provider.process_sse(event).await,
        }
    }
}
