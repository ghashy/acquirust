use std::collections::BTreeMap;

use acquisim_api::init_payment::{InitPaymentRequest, InitPaymentResponse};
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
) -> Json<InitPaymentResponse> {
    // Authorize request
    let token = req.generate_token(&state.settings.terminal_settings.password);

    if !token.eq(req.token()) {
        tracing::warn!("Unauthorized Init request");
        return Json(InitPaymentResponse::err());
    }

    // We have only one store account in our virtual bank
    let store_card = state.bank.get_store_account().await.card();

    // We store active payments in the RAM for simplicity
    let (active_payment_id, created_at) =
        match state.active_payments.create_payment(req, store_card) {
            Ok(result) => result,
            Err(e) => {
                tracing::error!("Failed to create payment: {e}");
                return Json(InitPaymentResponse::err());
            }
        };

    // Launch async task which will track our payment
    watch_and_delete_active_payment(
        state.clone(),
        active_payment_id,
        created_at,
    );

    let url = format!(
        "{}:{}/payment_page/{}",
        state.settings.addr, state.settings.port, active_payment_id
    );

    let payment_url = match url::Url::parse(&url) {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse url: {e}");
            return Json(InitPaymentResponse::err());
        }
    };

    Json(InitPaymentResponse {
        payment_url: Some(payment_url),
        operation_status: acquisim_api::init_payment::OperationStatus::Success,
    })
}
