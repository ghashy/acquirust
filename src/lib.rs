#![allow(dead_code)]
use phonenumber::PhoneNumber;
use serde::Serializer;

mod manager;
mod payment_service;

pub mod domain;
pub mod mapi;

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

pub fn serialize_phonenumber<S>(
    number: &Option<PhoneNumber>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match number {
        Some(number) => serializer.serialize_str(
            &number.format().mode(phonenumber::Mode::E164).to_string(),
        ),
        None => serializer.serialize_none(),
    }
}
