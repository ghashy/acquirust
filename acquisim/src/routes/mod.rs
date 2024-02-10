use anyhow::Context;
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing, Json, Router,
};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{self, Digest, Sha256};
use tokio::sync::TryLockError;
use url::Url;
use uuid::Uuid;

use crate::{html_gen::SubmitPaymentPage, startup::AppState};

pub mod api;
pub mod system;

#[derive(Debug, Deserialize)]
struct Credentials {
    card_number: u32,
    password: Secret<String>,
}

#[tracing::instrument(name = "Get payment html page", skip_all)]
pub async fn get_payment_html_page(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    match state.active_payments.try_acquire_payment(payment_id) {
        Ok(p) => match SubmitPaymentPage::new(p.request.amount).render() {
            Ok(body) => Ok(Html(body)),
            Err(e) => {
                tracing::error!("Failed to render payment html page: {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            tracing::error!("Failed to get payment html page: {e}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[tracing::instrument(name = "Trigger payment", skip_all)]
pub async fn trigger_payment(
    State(state): State<AppState>,
    Json(creds): Json<Credentials>,
) {
    todo!()
}
