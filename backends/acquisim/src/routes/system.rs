use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use tokio::sync::TryLockError;

use crate::bank::BankOperationError;
use crate::bank::Transaction;
use crate::domain::requests::system_api::AddAccountRequest;
use crate::domain::requests::system_api::DeleteAccountRequest;
use crate::domain::requests::system_api::NewTransactionRequest;
use crate::domain::requests::system_api::OpenCreditRequest;
use crate::domain::responses::system_api::AddAccountResponse;
use crate::domain::responses::system_api::ListAccountsResponse;
use crate::error_chain_fmt;
use crate::middleware::BasicAuthLayer;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
enum SystemApiError {
    #[error("Mutex lock error: {0}")]
    MutexLockError(#[from] TryLockError),
    #[error("Bank operation error: {0}")]
    BankOperationError(#[from] BankOperationError),
}

impl std::fmt::Debug for SystemApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for SystemApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("System api error: {self}");
        match self {
            SystemApiError::MutexLockError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            SystemApiError::BankOperationError(_) => {
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn system_router(state: AppState) -> Router {
    Router::new()
        .route("/account", routing::post(add_account))
        .route("/account", routing::delete(delete_account))
        .route("/list_accounts", routing::get(list_accounts))
        .route("/credit", routing::post(open_credit))
        .route("/transaction", routing::post(new_transaction))
        .route("/list_transactions", routing::get(list_transactions))
        .with_state(state.clone())
        .layer(BasicAuthLayer { state })
}

#[tracing::instrument(name = "Add a new account to the bank", skip_all)]
async fn add_account(
    State(state): State<AppState>,
    Json(req): Json<AddAccountRequest>,
) -> Result<Json<AddAccountResponse>, SystemApiError> {
    let card_number = state.bank.add_account(&req.password).await;
    Ok(Json(AddAccountResponse { card_number }))
}

#[tracing::instrument(name = "Delete existing account", skip_all)]
#[axum::debug_handler]
async fn delete_account(
    State(state): State<AppState>,
    Json(req): Json<DeleteAccountRequest>,
) -> Result<StatusCode, SystemApiError> {
    state.bank.delete_account(req.card_number).await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "List info about accounts", skip_all)]
async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<ListAccountsResponse>, SystemApiError> {
    let accounts = state.bank.list_accounts().await;
    Ok(Json(ListAccountsResponse { accounts }))
}

#[tracing::instrument(name = "Open credit for account", skip_all)]
async fn open_credit(
    State(state): State<AppState>,
    Json(req): Json<OpenCreditRequest>,
) -> Result<StatusCode, SystemApiError> {
    state.bank.open_credit(req.card_number, req.amount).await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Create a new transaction", skip_all)]
async fn new_transaction(
    State(state): State<AppState>,
    Json(req): Json<NewTransactionRequest>,
) -> Result<StatusCode, SystemApiError> {
    let sender = state.bank.find_account(&req.from).await?;
    let receiver = state.bank.find_account(&req.to).await?;
    state
        .bank
        .new_transaction(&sender, &receiver, req.amount)
        .await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Get a vec with transactions", skip_all)]
async fn list_transactions(
    State(state): State<AppState>,
) -> Json<Vec<Transaction>> {
    Json(state.bank.list_transactions().await)
}
