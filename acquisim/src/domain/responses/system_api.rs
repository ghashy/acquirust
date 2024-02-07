use serde::Serialize;

use crate::bank::Transaction;

#[derive(Serialize)]
pub struct AddAccountResponse {
    pub card_number: uuid::Uuid,
}

#[derive(Serialize)]
pub struct Account {
    pub card_number: uuid::Uuid,
    pub balance: i64,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<Account>,
}
