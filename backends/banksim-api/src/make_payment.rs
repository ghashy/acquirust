use std::collections::BTreeMap;

use airactions::{ApiAction, ClientError, ReqwestClient};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::Tokenizable;

// ───── Api Action ───────────────────────────────────────────────────────── //

pub struct MakePayment;

impl ApiAction for MakePayment {
    type Request = MakePaymentRequest;
    type Response = MakePaymentResponse;

    fn url_path(&self) -> &'static str {
        "/api/MakePayment"
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

/// Initial payment operation, basic of acquiring
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MakePaymentRequest {
    /// Currently unused
    pub amount: i64,
    pub recipient_token: String,
    token: String,
}

impl MakePaymentRequest {
    pub fn new(
        recipient_card_token: String,
        amount: i64,
        cashbox_password: &Secret<String>,
    ) -> Self {
        let mut req = MakePaymentRequest {
            amount,
            token: String::new(),
            recipient_token: recipient_card_token,
        };

        req.token = req.generate_token(cashbox_password);
        req
    }

    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("recipient_token", self.recipient_token.clone());
        token_map.insert("amount", self.amount.to_string());
        token_map.insert("password", cashbox_password.expose_secret().clone());

        let concatenated: String = token_map.into_values().collect();
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concatenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        format!("{:x}", hash_result)
    }
}

impl Tokenizable for MakePaymentRequest {
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
pub struct MakePaymentResponse {
    result: Result<(), String>,
}

impl MakePaymentResponse {
    pub fn err(reason: String) -> Self {
        MakePaymentResponse {
            result: Err(reason),
        }
    }

    pub fn success() -> Self {
        MakePaymentResponse { result: Ok(()) }
    }
}
