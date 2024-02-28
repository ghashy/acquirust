use secrecy::Secret;
use serde::{Deserialize, Serialize};

pub use acquiconnect::*;
use url::Url;

pub mod init_payment;
pub mod register_card_token;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatus {
    Success,
    Fail,
}

pub trait Tokenizable {
    fn validate_token(&self, password: &Secret<String>) -> Result<(), ()>;
}

pub trait Operation {
    fn operation_error() -> Self;
    fn operation_success(session_ui_url: Url) -> Self;
}
