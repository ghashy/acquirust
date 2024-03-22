use crate::OperationStatus;
use crate::Tokenizable;
use std::collections::BTreeMap;

use airactions::{ApiAction, ClientError, ReqwestClient};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;
use uuid::Uuid;

// ───── Api Action ───────────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Webhook {
    Confirm,
    Capture,
    Cancel,
}

impl std::fmt::Display for Webhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Webhook::Confirm => f.write_str("Confirm"),
            Webhook::Capture => f.write_str("Capture"),
            Webhook::Cancel => f.write_str("Cancel"),
        }
    }
}

impl ApiAction for Webhook {
    type Request = WebhookRequest;
    type Response = WebhookResponse;

    fn url_path(&self) -> &'static str {
        match self {
            Webhook::Confirm => "/session/confirm",
            Webhook::Capture => "/session/capture",
            Webhook::Cancel => "/session/cancel",
        }
    }

    async fn perform_action(
        req: Self::Request,
        addr: Url,
        client: &ReqwestClient,
    ) -> Result<Self::Response, ClientError> {
        match client.post(addr).json(&req).send().await {
            Ok(response) => Ok(response.json().await?),
            Err(e) => Err(e)?,
        }
    }
}

// ───── Request Type ─────────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookRequest {
    pub webhook: Webhook,
    pub session_id: Uuid,
    token: String,
}

impl WebhookRequest {
    pub fn new(
        webhook: Webhook,
        session_id: Uuid,
        cashbox_password: &Secret<String>,
    ) -> Self {
        let mut req = WebhookRequest {
            session_id,
            token: String::new(),
            webhook,
        };
        req.token = req.generate_token(cashbox_password);
        req
    }

    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("session_id", self.session_id.to_string());
        token_map.insert("password", cashbox_password.expose_secret().clone());
        token_map.insert("webhook", self.webhook.to_string());

        let concatenated: String = token_map.into_values().collect();
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concatenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        format!("{:x}", hash_result)
    }
}

impl Tokenizable for WebhookRequest {
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
pub struct WebhookResponse {
    pub session_id: Uuid,
    pub status: OperationStatus,
}

// impl_request_action!(
//     Confirm,
//     ConfirmRequest,
//     ConfirmResponse,
//     "/session/confirm"
// );

// impl_request_action!(
//     Capture,
//     CaptureRequest,
//     CaptureResponse,
//     "/session/capture"
// );

// impl_request_action!(Cancel, CancelRequest, CancelResponse, "/session/cancel");
