use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BASE_URL: &str = "https://api.enrow.io";

// --- Request types ---

#[derive(Debug, Clone, Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VerifyEmailParams {
    pub email: String,
    pub webhook: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BulkVerification {
    pub email: String,
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone)]
pub struct VerifyEmailsParams {
    pub verifications: Vec<BulkVerification>,
    pub webhook: Option<String>,
}

// --- Response types ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
    pub id: String,
    pub email: Option<String>,
    pub qualification: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub credits_used: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkVerificationResult {
    pub batch_id: String,
    pub total: u32,
    pub status: String,
    pub credits_used: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkVerificationResults {
    pub batch_id: String,
    pub status: String,
    pub total: u32,
    pub completed: Option<u32>,
    pub credits_used: Option<u32>,
    pub results: Option<Vec<VerificationResult>>,
}

// --- Internal helpers ---

#[derive(Debug, Deserialize)]
struct ApiError {
    message: Option<String>,
}

fn build_client(api_key: &str) -> Result<Client, Box<dyn std::error::Error>> {
    use reqwest::header::{HeaderMap, HeaderValue};

    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(api_key)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

async fn check_response(
    response: reqwest::Response,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let status = response.status();
    let body: serde_json::Value = response.json().await?;

    if !status.is_success() {
        let msg = body
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown API error");
        return Err(format!("API error {}: {}", status.as_u16(), msg).into());
    }

    Ok(body)
}

fn build_settings(
    webhook: &Option<String>,
) -> Option<Settings> {
    if webhook.is_some() {
        Some(Settings {
            webhook: webhook.clone(),
        })
    } else {
        None
    }
}

// --- Public API ---

/// Start a single email verification. Returns a `VerificationResult` containing the verification ID.
/// Poll with `get_verification_result` to retrieve the final result, or provide a webhook URL.
pub async fn verify_email(
    api_key: &str,
    params: &VerifyEmailParams,
) -> Result<VerificationResult, Box<dyn std::error::Error>> {
    let client = build_client(api_key)?;

    let mut body = serde_json::Map::new();
    body.insert("email".into(), serde_json::Value::String(params.email.clone()));

    if let Some(settings) = build_settings(&params.webhook) {
        body.insert("settings".into(), serde_json::to_value(settings)?);
    }

    let response = client
        .post(format!("{}/email/verify/single", BASE_URL))
        .json(&body)
        .send()
        .await?;

    let data = check_response(response).await?;
    let result: VerificationResult = serde_json::from_value(data)?;
    Ok(result)
}

/// Retrieve the result of a single email verification by its ID.
pub async fn get_verification_result(
    api_key: &str,
    id: &str,
) -> Result<VerificationResult, Box<dyn std::error::Error>> {
    let client = build_client(api_key)?;

    let response = client
        .get(format!("{}/email/verify/single?id={}", BASE_URL, id))
        .send()
        .await?;

    let data = check_response(response).await?;
    let result: VerificationResult = serde_json::from_value(data)?;
    Ok(result)
}

/// Start a bulk email verification. Returns a `BulkVerificationResult` with a batch ID.
/// Poll with `get_verification_results` to retrieve results, or provide a webhook URL.
pub async fn verify_emails(
    api_key: &str,
    params: &VerifyEmailsParams,
) -> Result<BulkVerificationResult, Box<dyn std::error::Error>> {
    let client = build_client(api_key)?;

    let verifications: Vec<serde_json::Value> = params
        .verifications
        .iter()
        .map(|v| {
            let mut entry = serde_json::Map::new();
            entry.insert("email".into(), serde_json::Value::String(v.email.clone()));

            if let Some(ref custom) = v.custom {
                entry.insert("custom".into(), serde_json::to_value(custom).unwrap());
            }

            serde_json::Value::Object(entry)
        })
        .collect();

    let mut body = serde_json::Map::new();
    body.insert("verifications".into(), serde_json::Value::Array(verifications));

    if let Some(settings) = build_settings(&params.webhook) {
        body.insert("settings".into(), serde_json::to_value(settings)?);
    }

    let response = client
        .post(format!("{}/email/verify/bulk", BASE_URL))
        .json(&body)
        .send()
        .await?;

    let data = check_response(response).await?;
    let result: BulkVerificationResult = serde_json::from_value(data)?;
    Ok(result)
}

/// Retrieve the results of a bulk email verification by its batch ID.
pub async fn get_verification_results(
    api_key: &str,
    id: &str,
) -> Result<BulkVerificationResults, Box<dyn std::error::Error>> {
    let client = build_client(api_key)?;

    let response = client
        .get(format!("{}/email/verify/bulk?id={}", BASE_URL, id))
        .send()
        .await?;

    let data = check_response(response).await?;
    let result: BulkVerificationResults = serde_json::from_value(data)?;
    Ok(result)
}
