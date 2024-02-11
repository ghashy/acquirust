mod active_payment;
mod bank;
mod config;
mod html_gen;
mod middleware;
mod routes;
mod startup;

pub mod domain;
pub mod tasks;

pub use config::Settings;
pub use startup::Application;

pub fn error_chain_fmt(
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
