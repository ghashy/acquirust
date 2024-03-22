use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::OperationStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Notification {
    PaymentNotification(PaymentNotification),
    TokenNotification(TokenNotification),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentNotification {
    ReadyToConfirm {
        session_id: Uuid,
    },
    ReadyToCapture {
        session_id: Uuid,
    },
    PaymentFinished {
        session_id: Uuid,
        status: OperationStatus,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenNotification {
    ReadyToConfirm {
        session_id: Uuid,
    },
    Finished {
        #[serde(skip_serializing_if = "Option::is_none")]
        card_token: Option<String>,
        session_id: Uuid,
        status: OperationStatus,
    },
}
