use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddAccountRequest {
    pub password: Secret<String>,
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    pub card_number: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct OpenCreditRequest {
    pub card_number: uuid::Uuid,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct NewTransactionRequest {
    pub from: uuid::Uuid,
    pub to: uuid::Uuid,
    pub amount: i64,
}
