use std::collections::BTreeMap;

use airactions::{ApiAction, ClientError, ReqwestClient};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::{Operation, OperationStatus, Tokenizable};

use self::beneficiaries::Beneficiaries;

pub mod beneficiaries;

// ───── Api Action ───────────────────────────────────────────────────────── //

pub struct InitPayment;

impl ApiAction for InitPayment {
    type Request = InitPaymentRequest;
    type Response = InitPaymentResponse;

    fn url_path(&self) -> &'static str {
        "/api/InitPayment"
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
/// If there are more than zero beneficiaries, it is `SPLIT PAYMENT`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitPaymentRequest {
    /// Currently unused
    pub notification_url: Url,
    pub success_url: Url,
    pub fail_url: Url,
    pub amount: i64,
    pub beneficiaries: beneficiaries::Beneficiaries,
    token: String,
}

impl InitPaymentRequest {
    /// If you want to perform split payment, include store's card token
    /// and all others into the beneficiaries list.
    pub fn new(
        notification_url: Url,
        success_url: Url,
        fail_url: Url,
        amount: i64,
        cashbox_password: &Secret<String>,
        beneficiaries: Option<Beneficiaries>,
    ) -> Self {
        let mut req = InitPaymentRequest {
            notification_url,
            success_url,
            fail_url,
            amount,
            token: String::new(),
            beneficiaries: beneficiaries.unwrap_or(Beneficiaries::NONE),
        };
        req.token = req.generate_token(cashbox_password);
        req
    }

    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("notification_url", self.notification_url.to_string());
        token_map.insert("success_url", self.success_url.to_string());
        token_map.insert("fail_url", self.fail_url.to_string());
        token_map.insert("amount", self.amount.to_string());
        token_map.insert("password", cashbox_password.expose_secret().clone());

        if !self.beneficiaries.is_empty() {
            token_map.insert("beneficiaries", self.beneficiaries.as_str());
        }

        let concatenated: String = token_map.into_values().collect();
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concatenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        format!("{:x}", hash_result)
    }
}

impl Tokenizable for InitPaymentRequest {
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
pub struct InitPaymentResponse {
    pub status: OperationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<Url>,
}

impl InitPaymentResponse {
    pub fn err(reason: String) -> Self {
        InitPaymentResponse {
            payment_url: None,
            status: OperationStatus::Fail(reason),
        }
    }

    pub fn success(payment_url: Url) -> Self {
        InitPaymentResponse {
            payment_url: Some(payment_url),
            status: OperationStatus::Success,
        }
    }
}

impl Operation for InitPaymentResponse {
    fn operation_error(reason: String) -> InitPaymentResponse {
        Self::err(reason)
    }

    fn operation_success(session_ui_url: Url) -> InitPaymentResponse {
        InitPaymentResponse::success(session_ui_url)
    }
}

// ───── Notification Type ────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentOperationNotification {
    pub operation_status: OperationStatus,
}

impl PaymentOperationNotification {
    pub fn err(reason: String) -> Self {
        PaymentOperationNotification {
            operation_status: OperationStatus::Fail(reason),
        }
    }

    pub fn success() -> Self {
        PaymentOperationNotification {
            operation_status: OperationStatus::Success,
        }
    }
}
