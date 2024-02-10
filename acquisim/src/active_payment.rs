use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::Context;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{error_chain_fmt, routes::api::InitPaymentRequest};

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

pub struct ActivePayment {
    pub request: InitPaymentRequest,
    creation_time: OffsetDateTime,
    id: Uuid,
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
    ) -> Result<Uuid, ActivePaymentsError> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        self.0
            .lock()
            .map_err(|e| ActivePaymentsError::MutexError(e.to_string()))?
            .push(ActivePayment {
                request,
                creation_time: now,
                id,
            });
        Ok(id)
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
        let mut lock = self
            .0
            .lock()
            .map_err(|e| ActivePaymentsError::MutexError(e.to_string()))?;
        if let Some(pos) = lock.iter().position(|p| p.id.eq(&id)) {
            Ok(lock.swap_remove(pos))
        } else {
            Err(ActivePaymentsError::NoPaymentError(id))
        }
    }
}
