use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use acquisim::Application;
use acquisim::Settings;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("axum::rejection=trace".parse().unwrap())
                .add_directive("tower_sessions_core=warn".parse().unwrap())
                .add_directive("aws_config=warn".parse().unwrap()),
        )
        .compact()
        .with_level(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up tracing");

    let settings = Settings::load_configuration().unwrap();

    if let Err(e) = Application::build(settings)
        .await
        .expect("Failed to build application")
        .run_until_stopped()
        .await
    {
        eprintln!("Error: {}", e);
    }
}
