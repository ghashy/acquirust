use crate::{Operation, OperationStatus, Tokenizable};
use std::collections::BTreeMap;

use acquiconnect::{ApiAction, Client, ClientError};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

// ───── Api Action ───────────────────────────────────────────────────────── //

pub struct RegisterCardToken;

impl ApiAction for RegisterCardToken {
    type Request = RegisterCardTokenRequest;
    type Response = RegisterCardTokenResponse;

    fn url_path(&self) -> &'static str {
        "/api/RegisterCardToken"
    }

    async fn perform_action(
        req: Self::Request,
        addr: Url,
        client: &Client,
    ) -> Result<Self::Response, ClientError> {
        match client.post(addr).json(&req).send().await {
            Ok(response) => Ok(response.json().await?),
            Err(e) => Err(e)?,
        }
    }
}

// ───── Request Type ─────────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterCardTokenRequest {
    pub notification_url: Url,
    pub fail_url: Url,
    pub success_url: Url,
    token: String,
}

impl RegisterCardTokenRequest {
    pub fn new(
        notification_url: Url,
        success_url: Url,
        fail_url: Url,
        cashbox_password: &Secret<String>,
    ) -> Self {
        let mut req = RegisterCardTokenRequest {
            notification_url,
            token: String::new(),
            fail_url,
            success_url,
        };
        req.token = req.generate_token(cashbox_password);
        req
    }

    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("notification_url", self.notification_url.to_string());
        token_map.insert("password", cashbox_password.expose_secret().clone());

        let concatenated: String = token_map.into_values().collect();
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concatenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        format!("{:x}", hash_result)
    }
}

impl Tokenizable for RegisterCardTokenRequest {
    fn validate_token(&self, password: &Secret<String>) -> Result<(), ()> {
        let token = self.generate_token(password);
        if token.eq(&self.token) {
            Ok(())
        } else {
            Err(())
        }
    }
}

// ───── Response Type ────────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterCardTokenResponse {
    pub registration_url: Option<Url>,
    pub status: OperationStatus,
}

impl RegisterCardTokenResponse {
    pub fn err(reason: String) -> Self {
        RegisterCardTokenResponse {
            registration_url: None,
            status: OperationStatus::Fail(reason),
        }
    }

    pub fn success(registration_url: Url) -> Self {
        RegisterCardTokenResponse {
            registration_url: Some(registration_url),
            status: OperationStatus::Success,
        }
    }
}

impl Operation for RegisterCardTokenResponse {
    fn operation_error(reason: String) -> Self {
        RegisterCardTokenResponse::err(reason)
    }

    fn operation_success(session_ui_url: Url) -> Self {
        RegisterCardTokenResponse::success(session_ui_url)
    }
}

// ───── Notification Type ────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterCardTokenOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    pub operation_status: OperationStatus,
}

impl RegisterCardTokenOperationResult {
    pub fn err(reason: String) -> Self {
        RegisterCardTokenOperationResult {
            card_token: None,
            operation_status: OperationStatus::Fail(reason),
        }
    }

    pub fn success(token: String) -> Self {
        RegisterCardTokenOperationResult {
            card_token: Some(token),
            operation_status: OperationStatus::Success,
        }
    }
}
