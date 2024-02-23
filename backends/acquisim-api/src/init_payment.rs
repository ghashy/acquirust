use std::collections::BTreeMap;

use acquiconnect::{ApiAction, Client, ClientError};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

pub struct InitPayment;

/// Initial payment operation, basic of acquiring
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitPaymentRequest {
    /// Currently unused
    pub notification_url: Url,
    pub success_url: Url,
    pub fail_url: Url,
    pub amount: i64,
    token: String,
}

impl InitPaymentRequest {
    pub fn new(
        notification_url: Url,
        success_url: Url,
        fail_url: Url,
        amount: i64,
        cashbox_password: &Secret<String>,
    ) -> Self {
        let mut req = InitPaymentRequest {
            notification_url,
            success_url,
            fail_url,
            amount,
            token: String::new(),
        };
        req.token = req.generate_token(cashbox_password);
        req
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("notification_url", self.notification_url.to_string());
        token_map.insert("success_url", self.success_url.to_string());
        token_map.insert("fail_url", self.fail_url.to_string());
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatus {
    Success,
    Fail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitPaymentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<url::Url>,
    pub operation_status: OperationStatus,
}

impl InitPaymentResponse {
    pub fn err() -> Self {
        InitPaymentResponse {
            payment_url: None,
            operation_status: OperationStatus::Fail,
        }
    }
}

impl ApiAction for InitPayment {
    type Request = InitPaymentRequest;
    type Response = InitPaymentResponse;

    fn url_path(&self) -> &'static str {
        "/api/Init"
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
