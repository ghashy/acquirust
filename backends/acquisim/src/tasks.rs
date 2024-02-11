use time::OffsetDateTime;
use uuid::Uuid;

use crate::startup::AppState;

pub fn watch_and_delete_active_payment(
    app_state: AppState,
    payment_id: Uuid,
    created_at: OffsetDateTime,
) {
    tokio::spawn(async move {
        let interval =
            (created_at + time::Duration::hours(1)) - OffsetDateTime::now_utc();
        match interval.try_into() {
            Ok(duration) => {
                tokio::time::sleep(duration).await;
            }
            Err(e) => {
                tracing::error!("Failed to calculate std::time::Duration from time::Duration: {e}")
            }
        }
        match app_state.active_payments.remove_payment(payment_id) {
            Ok(()) => {
                tracing::info!(
                    "Active payment with id: {payment_id} is deleted!"
                )
            }
            Err(e) => {
                tracing::error!(
                    "Failed to remove active payment with id: {payment_id}, error: {e}"
                )
            }
        }
    });
}
