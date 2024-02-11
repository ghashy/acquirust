use serde::Serialize;

use crate::{bank::Transaction, domain::card_number::CardNumber};

#[derive(Serialize)]
pub struct AddAccountResponse {
    pub card_number: CardNumber,
}

#[derive(Serialize)]
pub struct Account {
    pub card_number: CardNumber,
    pub balance: i64,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize)]
pub struct ListAccountsResponse {
    pub accounts: Vec<Account>,
}
