//! AI Module for transformation matrix generation
//!
//! Uses Anthropic Claude API to analyze CSV data and generate transformation matrices.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use massload::ai::{AiClient, generate_matrix};
//! use massload::csv_to_json;
//!
//! // Parse CSV
//! let csv_data = csv_to_json(csv_content, ';')?;
//!
//! // Generate matrix using AI
//! let client = AiClient::from_env()?;
//! let matrix = client.generate_matrix(&csv_data[..10]).await?;
//! ```

pub mod prompt;

use serde::Deserialize;
use serde_json::Value;
use std::env;
use thiserror::Error;

use crate::transform::dsl::TransformationMatrix;

pub use prompt::{system_prompt, user_prompt_with_all_data};

/// AI-related errors
#[derive(Error, Debug)]
pub enum AiError {
    #[error("Missing API key: {0}")]
    MissingApiKey(String),

    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid JSON response: {0}")]
    InvalidJson(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Failed to parse matrix: {0}")]
    ParseError(String),
}

/// Anthropic API client
#[derive(Clone)]
pub struct AiClient {
    api_key: String,
    model: String,
    max_tokens: u32,
}

/// Anthropic API response structure
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: String,
}

/// Anthropic API error response
#[derive(Debug, Deserialize)]
struct AnthropicError {
    error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    message: String,
}

/// Default number of retries
const DEFAULT_MAX_RETRIES: u32 = 3;

/// Delay between retries in milliseconds
const RETRY_DELAY_MS: u64 = 1000;

impl AiClient {
    /// Create a new client with explicit API key
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 1024,
        }
    }

    /// Create a client from environment variable ANTHROPIC_API_KEY
    pub fn from_env() -> Result<Self, AiError> {
        // Try loading .env file
        let _ = dotenvy::dotenv();

        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| AiError::MissingApiKey("ANTHROPIC_API_KEY not set".to_string()))?;

        Ok(Self::new(api_key))
    }

    /// Set the model to use
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Generate a transformation matrix from CSV data
    ///
    /// # Arguments
    /// * `csv_preview` - First N rows of parsed CSV as JSON objects (shown to AI)
    /// * `all_records` - All records (for extracting unique values)
    ///
    /// # Returns
    /// A TransformationMatrix ready to use with the executor
    pub async fn generate_matrix_full(&self, csv_preview: &[Value], all_records: &[Value]) -> Result<TransformationMatrix, AiError> {
        let schema = load_flat_schema()?;
        self.generate_matrix_with_schema_full(csv_preview, all_records, &schema).await
    }

    /// Generate matrix with custom schema (with retries)
    pub async fn generate_matrix_with_schema_full(
        &self,
        csv_preview: &[Value],
        all_records: &[Value],
        schema: &Value,
    ) -> Result<TransformationMatrix, AiError> {
        let mut last_error = None;
        
        for attempt in 1..=DEFAULT_MAX_RETRIES {
            match self.try_generate_matrix(csv_preview, all_records, schema).await {
                Ok(matrix) => return Ok(matrix),
                Err(e) => {
                    eprintln!("   ⚠️  Attempt {}/{} failed: {}", attempt, DEFAULT_MAX_RETRIES, e);
                    last_error = Some(e);
                    
                    if attempt < DEFAULT_MAX_RETRIES {
                        eprintln!("   ↻ Retrying in {}ms...", RETRY_DELAY_MS);
                        tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| AiError::ApiError("Unknown error".to_string())))
    }

    /// Single attempt to generate matrix
    async fn try_generate_matrix(
        &self,
        csv_preview: &[Value],
        all_records: &[Value],
        schema: &Value,
    ) -> Result<TransformationMatrix, AiError> {
        let response = self.call_api(csv_preview, all_records, schema).await?;
        parse_matrix_from_response(&response)
    }

    /// Call Anthropic API
    async fn call_api(&self, csv_preview: &[Value], all_records: &[Value], schema: &Value) -> Result<String, AiError> {
        println!("   📡 Calling Anthropic API...");
        println!("      Model: {}", self.model);
        println!("      Max tokens: {}", self.max_tokens);
        println!("      Preview rows: {}, Total rows for unique values: {}", csv_preview.len(), all_records.len());
        
        let client = reqwest::Client::new();

        let messages = prompt::build_messages_with_all_data(csv_preview, all_records, schema);
        let system = prompt::system_prompt();

        let request_body = serde_json::json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "temperature": 0,
            "system": system,
            "messages": messages
        });

        println!("      Sending request...");
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        println!("      Response status: {}", status);
        
        let body = response
            .text()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !status.is_success() {
            // Try to parse error
            if let Ok(error) = serde_json::from_str::<AnthropicError>(&body) {
                println!("      ✗ API error: {}", error.error.message);
                return Err(AiError::ApiError(error.error.message));
            }
            println!("      ✗ HTTP error: {}", status);
            return Err(AiError::ApiError(format!("HTTP {}: {}", status, body)));
        }

        let response: AnthropicResponse =
            serde_json::from_str(&body).map_err(|e| AiError::InvalidJson(e.to_string()))?;

        // Extract text from response
        let text = response
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>()
            .join("");

        if text.is_empty() {
            return Err(AiError::InvalidJson("Empty response".to_string()));
        }

        println!("      ✓ Received {} bytes", text.len());
        Ok(text)
    }
}

/// Load the flat schema from embedded file
fn load_flat_schema() -> Result<Value, AiError> {
    let schema_str = include_str!("../../schemas/midds-musical-work-flat.json");
    serde_json::from_str(schema_str).map_err(|e| AiError::ParseError(e.to_string()))
}

/// Parse transformation matrix from AI response
fn parse_matrix_from_response(response: &str) -> Result<TransformationMatrix, AiError> {
    // Try to extract JSON from response (may have markdown code blocks)
    let json_str = extract_json(response);

    TransformationMatrix::from_json(&json_str).map_err(|e| {
        AiError::ParseError(format!(
            "Failed to parse matrix: {}. Response was: {}",
            e,
            &response[..response.len().min(500)]
        ))
    })
}

/// Extract JSON from a response that may contain markdown code blocks
fn extract_json(text: &str) -> String {
    // Try to find JSON in code block
    if let Some(start) = text.find("```json") {
        if let Some(end) = text[start..].find("```\n").or_else(|| text[start..].rfind("```")) {
            let json_start = start + 7; // len of "```json"
            if json_start < start + end {
                return text[json_start..start + end].trim().to_string();
            }
        }
    }

    // Try to find JSON in generic code block
    if let Some(start) = text.find("```") {
        let after_start = start + 3;
        // Skip language identifier if present
        let content_start = text[after_start..]
            .find('\n')
            .map(|i| after_start + i + 1)
            .unwrap_or(after_start);

        if let Some(end) = text[content_start..].find("```") {
            return text[content_start..content_start + end].trim().to_string();
        }
    }

    // Try to find raw JSON object
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if start < end {
                return text[start..=end].to_string();
            }
        }
    }

    text.to_string()
}

/// Convenience function to generate matrix (creates client internally)
pub async fn generate_matrix(csv_preview: &[Value]) -> Result<TransformationMatrix, AiError> {
    let client = AiClient::from_env()?;
    client.generate_matrix_full(csv_preview, csv_preview).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_code_block() {
        let response = r#"Here's the matrix:

```json
{
  "version": "1.0",
  "transforms": {}
}
```

Done!"#;

        let json = extract_json(response);
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"transforms\""));
    }

    #[test]
    fn test_extract_raw_json() {
        let response = r#"{"version": "1.0", "transforms": {}}"#;
        let json = extract_json(response);
        assert_eq!(json, response);
    }

    #[test]
    fn test_load_schema() {
        let schema = load_flat_schema().unwrap();
        assert!(schema.get("properties").is_some());
    }
}

