#![allow(dead_code)]

mod domain;
mod manager;
mod payment_service;
mod tinkoff;

pub use domain::kopeck::Kopeck;
pub use tinkoff::payment::OrderId;
pub use tinkoff::payment::Payment;

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
