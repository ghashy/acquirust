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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatus {
    Success,
    Cancel,
    Fail(OperationError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationError {
    Unexpected(String),
    BadRequest,
    SessionNotFound,
    Cancelled,
    Failed { reason: String },
    NotAuthorizedRequest,
}

pub trait Tokenizable {
    fn validate_token(&self, password: &Secret<String>) -> Result<(), ()>;
}

pub trait Operation {
    fn operation_error(reason: OperationError) -> Self;
    fn operation_success(session_ui_url: Url, session_id: Uuid) -> Self;
}
