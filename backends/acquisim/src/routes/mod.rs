use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    Json,
};
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::card_number::CardNumber, html_gen::SubmitPaymentPage,
    startup::AppState,
};

pub mod api;
pub mod system;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    card_number: CardNumber,
    password: Secret<String>,
}

#[tracing::instrument(name = "Get payment html page", skip_all)]
pub async fn payment_html_page(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    // Try to create submit payment url for client (browser)
    let submit_payment_url = match format!(
        "http://{}:{}/payment/{}",
        state.settings.addr, state.settings.port, payment_id
    )
    .parse()
    {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse string as url: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Try to return html payment page
    match state.active_payments.try_acquire_payment(payment_id) {
        Ok(p) => match SubmitPaymentPage::new(
            p.request.amount,
            payment_id,
            submit_payment_url,
        )
        .render()
        {
            Ok(body) => Ok(Html(body)),
            Err(e) => {
                tracing::error!("Failed to render payment html page: {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            tracing::error!("Failed to get payment html page: {e}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

#[tracing::instrument(name = "Trigger payment", skip_all)]
pub async fn trigger_payment(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
    Json(creds): Json<Credentials>,
) -> Result<Redirect, StatusCode> {
    let payment = match state.active_payments.try_acquire_payment(payment_id) {
        Ok(p) => p,
        Err(e) => {
            // No such payment
            tracing::error!("Payment not found: {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Authorize card and password
    let account = match state
        .bank
        .authorize_account(&creds.card_number, &creds.password)
        .await
    {
        Ok(acc) => acc,
        Err(e) => {
            // Not authorized
            tracing::error!("Can't authorize account: {e}");
            return Ok(Redirect::to(payment.request.fail_url.as_str()));
        }
    };

    // Check store account
    let store_account = state.bank.get_store_account().await;
    if !store_account.card().eq(&payment.store_card) {
        tracing::error!("Faild to perform payment: wrong store account!");
        return Ok(Redirect::to(payment.request.fail_url.as_str()));
    }

    // Perform transaction
    let result = match state
        .bank
        .new_transaction(&account, &store_account, payment.request.amount)
        .await
    {
        Ok(()) => {
            if let Err(e) = state.active_payments.remove_payment(payment.id()) {
                tracing::error!("Failed to delete active payment: {e}")
            }
            Ok(Redirect::to(payment.request.success_url.as_str()))
        }
        Err(e) => {
            tracing::error!("Transaction failed: {e}");
            Ok(Redirect::to(payment.request.fail_url.as_str()))
        }
    };

    if let Err(e) = state.active_payments.remove_payment(payment.id()) {
        tracing::error!("Failed to remove payment from active: {e}");
    }
    // TODO: send notifications to store backend
    // Implement it here before returning result:

    result
}
