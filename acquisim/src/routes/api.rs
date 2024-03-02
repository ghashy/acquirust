use acquisim_api::{Operation, Tokenizable};
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing, Json,
    Router,
};

use serde::{Serialize};
use tokio::sync::TryLockError;
use url::Url;

use acquisim_api::init_payment::{InitPaymentRequest, InitPaymentResponse};
use acquisim_api::register_card_token::{
    RegisterCardTokenRequest, RegisterCardTokenResponse,
};

use crate::interaction_sessions::{IntoSession, SessionError};
use crate::{
    bank::BankOperationError, error_chain_fmt, startup::AppState,
    tasks::wait_and_remove,
};

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/InitPayment",
            routing::post(
                init_session::<InitPaymentRequest, InitPaymentResponse>,
            ),
        )
        .route(
            "/InitCardTokenRegistration",
            routing::post(
                init_session::<
                    RegisterCardTokenRequest,
                    RegisterCardTokenResponse,
                >,
            ),
        )
        .with_state(state)
}

#[tracing::instrument(name = "Init session", skip_all)]
async fn init_session<Request, Response>(
    State(state): State<AppState>,
    Json(req): Json<Request>,
) -> Json<impl Serialize + 'static>
where
    Request: Tokenizable + IntoSession,
    Response: Operation + Serialize + 'static,
{
    // Authorize request
    if req.validate_token(&state.settings.terminal_settings.password).is_err() {
        tracing::warn!("Unauthorized request");
        return Json(Response::operation_error("Unauthorized".to_string()));
    }

    // We have only one store account in our virtual bank
    let store_card = state.bank.get_store_account().await.card();

    let session = req.create_session(store_card);
    let id = session.id();
    let created_at = session.creation_time;

    // We store active sessions in the RAM for simplicity
    match state.interaction_sessions.insert(session) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to initiate session: {e}");
            return Json(Response::operation_error(
                "Internal error".to_string(),
            ));
        }
    };

    // Launch async task which will track our session
    wait_and_remove(state.interaction_sessions, id, created_at);

    let url = format!(
        "{}:{}/{}/{}",
        state.settings.addr, state.settings.port, Request::page_endpoint(), id
    );

    let session_ui_url = match Url::parse(&url) {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse url: {e}");
            return Json(Response::operation_error(
                "Internal error".to_string(),
            ));
        }
    };

    Ok(Json(Response::operation_success(session_ui_url)))
}
