use std::collections::BTreeMap;

use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing, Json,
    Router,
};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{self, Digest, Sha256};
use tokio::sync::TryLockError;
use url::Url;

use crate::{
    active_payment::ActivePaymentsError, bank::BankOperationError,
    error_chain_fmt, startup::AppState, tasks::watch_and_delete_active_payment,
};

// ───── Request Types ────────────────────────────────────────────────────── //

/// Initial payment operation, basic of acquiring
#[derive(Clone, Serialize, Deserialize)]
pub struct InitPaymentRequest {
    notification_url: Url,
    success_url: Url,
    fail_url: Url,
    pub amount: i64,
    token: String,
}

impl InitPaymentRequest {
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

#[derive(Serialize)]
pub struct InitPaymentResponse {
    payment_url: url::Url,
}

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
enum ApiError {
    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Mutex lock error: {0}")]
    MutexLockError(#[from] TryLockError),
    #[error("Bank operation error: {0}")]
    BankOperationError(#[from] BankOperationError),
    #[error("Unauthorized operation")]
    UnauthorizedError,
    #[error("Active payment operation fail")]
    ActivePaymentsError(#[from] ActivePaymentsError),
    #[error("Failed to parse url")]
    UrlParsingError(#[from] url::ParseError),
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("System api error: {self}");
        match self {
            ApiError::MutexLockError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ApiError::BankOperationError(_) => {
                StatusCode::BAD_REQUEST.into_response()
            }
            ApiError::UnauthorizedError => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            ApiError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ApiError::ActivePaymentsError(_) => {
                StatusCode::NOT_FOUND.into_response()
            }
            ApiError::UrlParsingError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/Init", routing::post(init_payment))
        .with_state(state)
}

#[tracing::instrument(name = "Init payment", skip_all)]
async fn init_payment(
    State(state): State<AppState>,
    Json(req): Json<InitPaymentRequest>,
) -> Result<Json<InitPaymentResponse>, ApiError> {
    // Authorize request
    let token = req.generate_token(&state.settings.terminal_settings.password);

    if !token.eq(&req.token) {
        return Err(ApiError::UnauthorizedError);
    }

    let store_card = state.bank.get_store_account().await.card();

    let (active_payment_id, created_at) =
        state.active_payments.create_payment(req, store_card)?;

    watch_and_delete_active_payment(
        state.clone(),
        active_payment_id,
        created_at,
    );

    let url = format!(
        "{}:{}/payment/{}",
        state.settings.addr, state.settings.port, active_payment_id
    );
    let payment_url = url::Url::parse(&url)?;

    Ok(Json(InitPaymentResponse { payment_url }))
}
