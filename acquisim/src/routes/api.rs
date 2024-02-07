use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing, Json,
    Router,
};
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use tokio::sync::TryLockError;

use crate::{bank::BankOperationError, error_chain_fmt, startup::AppState};

// ───── Request Types ────────────────────────────────────────────────────── //

#[derive(Deserialize)]
pub struct InitPaymentRequest {
    initiator_card: uuid::Uuid,
    initiator_password: Secret<String>,
    amount: i64,
}

#[derive(Serialize)]
pub struct InitPaymentResponse {
    #[serde(rename = "PaymentURL")]
    payment_url: url::Url,
}

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
enum ApiError {
    #[error("Mutex lock error: {0}")]
    MutexLockError(#[from] TryLockError),
    #[error("Bank operation error: {0}")]
    BankOperationError(#[from] BankOperationError),
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
) -> Result<StatusCode, ApiError> {
    Ok(StatusCode::OK)
}
