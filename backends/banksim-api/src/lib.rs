use secrecy::Secret;
use serde::{Deserialize, Serialize};

pub use airactions::*;
use url::Url;
use uuid::Uuid;

pub mod init_payment;
pub mod make_payment;
pub mod notifications;
pub mod register_card_token;
pub mod session;
pub mod token_info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatus {
    Success,
    Cancel,
    Fail(OperationError),
}

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
pub enum OperationError {
    #[error("Unexpected")]
    Unexpected(String),
    #[error("Bad request")]
    BadRequest,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Session was cancelled")]
    Cancelled,
    #[error("Failed: {reason}")]
    Failed { reason: String },
    #[error("Request not authorized")]
    NotAuthorizedRequest,
}

pub trait Tokenizable {
    fn validate_token(&self, password: &Secret<String>) -> Result<(), ()>;
}

pub trait Operation {
    fn operation_error(reason: OperationError) -> Self;
    fn operation_success(session_ui_url: Url, session_id: Uuid) -> Self;
}
