#![allow(dead_code)]

mod domain;
mod manager;
mod payment_service;
mod tinkoff;

pub(crate) use domain::email::{Email, EmailError};

pub use manager::PaymentManager;
pub use tinkoff::TinkoffInitPaymentResponse;
pub use tinkoff::TinkoffPaymentService;

pub(crate) fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
