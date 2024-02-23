use std::sync::{Arc, Mutex};

use acquisim_api::init_payment::InitPaymentRequest;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{domain::card_number::CardNumber, error_chain_fmt};

#[derive(thiserror::Error)]
pub enum ActivePaymentsError {
    #[error("Can't take a mutex lock: {0}")]
    MutexError(String),
    #[error("No payment with provided id: {0}")]
    NoPaymentError(Uuid),
}

impl std::fmt::Debug for ActivePaymentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone)]
pub struct ActivePayment {
    pub request: InitPaymentRequest,
    pub store_card: CardNumber,
    _creation_time: OffsetDateTime,
    id: Uuid,
}

impl ActivePayment {
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone)]
pub struct ActivePayments(Arc<Mutex<Vec<ActivePayment>>>);

impl ActivePayments {
    pub fn new() -> Self {
        ActivePayments(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn create_payment(
        &self,
        request: InitPaymentRequest,
        store_card: CardNumber,
    ) -> Result<(Uuid, OffsetDateTime), ActivePaymentsError> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        self.0
            .lock()
            .map_err(|e| ActivePaymentsError::MutexError(e.to_string()))?
            .push(ActivePayment {
                request,
                store_card,
                _creation_time: now,
                id,
            });
        Ok((id, now))
    }

    pub fn remove_payment(&self, id: Uuid) -> Result<(), ActivePaymentsError> {
        let mut lock = self
            .0
            .lock()
            .map_err(|e| ActivePaymentsError::MutexError(e.to_string()))?;
        if let Some(pos) = lock.iter().position(|p| p.id.eq(&id)) {
            let _ = lock.swap_remove(pos);
        }
        Ok(())
    }

    pub fn try_acquire_payment(
        &self,
        id: Uuid,
    ) -> Result<ActivePayment, ActivePaymentsError> {
        let lock = self
            .0
            .lock()
            .map_err(|e| ActivePaymentsError::MutexError(e.to_string()))?;
        if let Some(payment) = lock.iter().find(|p| p.id.eq(&id)) {
            Ok(payment.clone())
        } else {
            Err(ActivePaymentsError::NoPaymentError(id))
        }
    }
}
