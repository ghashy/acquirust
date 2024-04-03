use crate::{OperationStatus, Tokenizable};
use std::collections::BTreeMap;

use airactions::{ApiAction, ClientError, ReqwestClient};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

// ───── Api Action ───────────────────────────────────────────────────────── //

pub struct TokenInfo;

impl ApiAction for TokenInfo {
    type Request = TokenInfoRequest;
    type Response = TokenInfoResponse;

    fn url_path(&self) -> &'static str {
        "/token/info"
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
pub struct TokenInfoRequest {
    pub card_token: String,
    token: String,
}

impl TokenInfoRequest {
    pub fn new(card_token: String, cashbox_password: &Secret<String>) -> Self {
        let mut req = TokenInfoRequest {
            card_token,
            token: String::new(),
        };
        req.token = req.generate_token(cashbox_password);
        req
    }
    pub fn generate_token(&self, cashbox_password: &Secret<String>) -> String {
        let mut token_map = BTreeMap::new();
        token_map.insert("card_token", self.card_token.clone());
        token_map.insert("password", cashbox_password.expose_secret().clone());

        let concatenated: String = token_map.into_values().collect();
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concatenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        format!("{:x}", hash_result)
    }
}

impl Tokenizable for TokenInfoRequest {
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
pub struct TokenInfoResponse {
    /// If there is given token, this will be Ok(True)
    /// If token is inactive, this will be Ok(False)
    /// Otherwise error will be in String
    pub status: Result<bool, String>,
}
